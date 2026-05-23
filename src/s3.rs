use std::io::{BufReader, Read};

use minio::s3::{MinioClient, segmented_bytes::SegmentedBytes, types::S3Api};

const BUCKET: &str = "picture";

fn picture_name(id: i64) -> String {
    format!("obj-{id}")
}

pub trait S3Client {
    async fn upload_picture<T: Read>(&self, picture: i64, data: T) -> anyhow::Result<()>;

    async fn download_picture(&self, picture: i64) -> anyhow::Result<bytes::Bytes>;

    async fn create_buckets(&self) -> anyhow::Result<()>;

    async fn delete_picture(&self, picture: i64) -> anyhow::Result<()>;

    async fn update_picture<T: Read>(&self, picture: i64, data: T) -> anyhow::Result<()>;
}

impl S3Client for MinioClient {
    async fn upload_picture<T: Read>(&self, picture: i64, data: T) -> anyhow::Result<()> {
        self.put_object(
            BUCKET,
            picture_name(picture),
            SegmentedBytes::from(bytes::Bytes::from(
                BufReader::new(data)
                    .bytes()
                    .collect::<Result<Vec<_>, _>>()?,
            )),
        )?
        .build()
        .send()
        .await?;
        Ok(())
    }

    async fn download_picture(&self, picture: i64) -> anyhow::Result<bytes::Bytes> {
        Ok(self
            .get_object(BUCKET, picture_name(picture))?
            .build()
            .send()
            .await?
            .into_bytes()
            .await?)
    }

    async fn create_buckets(&self) -> anyhow::Result<()> {
        if self.bucket_exists(BUCKET)?.build().send().await?.exists() {
            Ok(())
        } else {
            self.create_bucket(BUCKET)?.build().send().await?;
            Ok(())
        }
    }

    async fn delete_picture(&self, picture: i64) -> anyhow::Result<()> {
        self.delete_object(BUCKET, picture_name(picture))?
            .build()
            .send()
            .await?;
        Ok(())
    }

    async fn update_picture<T: Read>(&self, picture: i64, data: T) -> anyhow::Result<()> {
        self.delete_picture(picture).await?;
        self.upload_picture(picture, data).await
    }
}
