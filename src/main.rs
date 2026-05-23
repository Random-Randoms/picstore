mod form;
mod postgres;
mod s3;

use form::{download::Form as DownloadForm, upload::Form as UploadForm};
use postgres::PgClient;
use s3::S3Client;

use actix_multipart::form::MultipartForm;
use actix_web::{App, HttpResponse, HttpServer, Responder, get, middleware::Logger, post, web};
use log::{error, info};
use minio::s3::{MinioClient, creds::StaticProvider, http::BaseUrl};
use tokio_postgres::{Client as PostgresClient, NoTls};

#[get("/")]
async fn index() -> impl Responder {
    info!("index request");
    HttpResponse::Ok().body("Hello world!")
}

#[get("/pictures")]
async fn download_picture(
    pg_client: web::Data<PostgresClient>,
    minio_client: web::Data<MinioClient>,
    req: web::Json<DownloadForm>,
) -> impl Responder {
    info!("picture download");
    let (user_name, scope_name, picture) = req.into_inner().into_parts();
    let valid = match pg_client
        .user_scope_picture_valid(user_name.as_str(), scope_name.as_str(), picture)
        .await
    {
        Err(e) => return HttpResponse::InternalServerError().body(format!("Err: {e}")),
        Ok(valid) => valid,
    };
    if !valid {
        return HttpResponse::Forbidden().body("No such picture or such user in that scope");
    }
    let picture = match minio_client.download_picture(picture).await {
        Err(e) => return HttpResponse::InternalServerError().body(format!("Err: {e}")),
        Ok(picture) => picture,
    };
    HttpResponse::Ok().body(picture)
}

#[post("/pictures")]
async fn upload_picture(
    pg_client: web::Data<PostgresClient>,
    minio_client: web::Data<MinioClient>,
    MultipartForm(req): MultipartForm<UploadForm>,
) -> impl Responder {
    info!("picture upload");
    let (user, scope_name) = req.json.into_inner().into_parts();
    let scope = match pg_client
        .user_in_scope(user.as_str(), scope_name.as_str())
        .await
    {
        Err(e) => return HttpResponse::InternalServerError().body(format!("Err: {e}")),
        Ok(scope) => scope,
    };
    let Some(scope) = scope else {
        return HttpResponse::Forbidden().body(format!("User {user} not in scope {scope_name}"));
    };
    let picture = match pg_client.add_picture(scope).await {
        Err(e) => return HttpResponse::InternalServerError().body(format!("Err: {e}")),
        Ok(picture) => picture,
    };
    if let Err(e) = minio_client.upload_picture(picture, req.file.file).await {
        return HttpResponse::InternalServerError()
            .body(format!("picture upload failed with error {e}"));
    }
    HttpResponse::Ok().body(format!("New picture id is {picture}"))
}

#[post("/users/{name}")]
async fn add_user(client: web::Data<PostgresClient>, name: web::Path<String>) -> impl Responder {
    info!("add user");
    match client.add_user(name.as_str()).await {
        Err(e) => HttpResponse::InternalServerError().body(format!("Err: {e}")),
        Ok(()) => HttpResponse::Ok().body(format!("User {name} created")),
    }
}

#[post("/scopes/{name}")]
async fn add_scope(client: web::Data<PostgresClient>, name: web::Path<String>) -> impl Responder {
    info!("add scope");
    match client.add_scope(name.as_str()).await {
        Err(e) => HttpResponse::InternalServerError().body(format!("Err: {e}")),
        Ok(()) => HttpResponse::Ok().body(format!("Scope {name} created")),
    }
}

#[post("/scopes/{scope}/{name}")]
async fn add_user_scope(
    client: web::Data<PostgresClient>,
    req: web::Path<(String, String)>,
) -> impl Responder {
    let (scope, user) = req.into_inner();
    info!("add user in scope");
    match client.add_user_scope(user.as_str(), scope.as_str()).await {
        Err(e) => HttpResponse::InternalServerError().body(format!("Err: {e}")),
        Ok(()) => HttpResponse::Ok().body(format!("User {user} added to scope {scope}")),
    }
}

async fn default() -> impl Responder {
    info!("unmatched enpoint, default response");
    HttpResponse::Ok().body("default response")
}

async fn catch_main() -> anyhow::Result<()> {
    env_logger::init();

    let (pg_client, pg_connection) = tokio_postgres::connect(
        "host=postgres user=postgres password=postgres dbname=postgres",
        NoTls,
    )
    .await?;

    info!("postgres connected");

    let minio_client = MinioClient::new(
        "http://minio:9000".parse::<BaseUrl>()?,
        Some(StaticProvider::new("minioadmin", "minioadmin", None)),
        None,
        None,
    )?;

    info!("minio connected");

    tokio::spawn(async move {
        if let Err(e) = pg_connection.await {
            error!("connection error: {}", e);
        }
    });

    pg_client.create_tables().await?;

    info!("postgres tables created");

    minio_client.create_buckets().await?;

    info!("minio buckets created");

    let web_data_pg = web::Data::new(pg_client);
    let web_data_minio = web::Data::new(minio_client);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web_data_pg.clone())
            .app_data(web_data_minio.clone())
            .service(index)
            .service(download_picture)
            .service(upload_picture)
            .service(add_user)
            .service(add_scope)
            .service(add_user_scope)
            .default_service(web::to(default))
    })
    .bind("0.0.0.0:3000")?
    .run()
    .await?;

    info!("all done");

    Ok(())
}

#[actix_web::main]
async fn main() {
    if let Err(e) = catch_main().await {
        error!("{e:?}");
    };
}
