use actix_multipart::form::{MultipartForm, json::Json as MpJson, tempfile::TempFile};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Data {
    user: String,
    scope: String,
    picture: i64,
}

impl Data {
    pub fn into_parts(self) -> (String, String, i64) {
        (self.user, self.scope, self.picture)
    }
}

#[derive(Debug, MultipartForm)]
pub struct Form {
    #[multipart(limit = "100MB")]
    pub file: TempFile,
    pub json: MpJson<Data>,
}
