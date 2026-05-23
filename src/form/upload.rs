use actix_multipart::form::{MultipartForm, json::Json as MpJson, tempfile::TempFile};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Data {
    pub user: String,
    pub scope: String,
}

impl Data {
    pub fn into_parts(self) -> (String, String) {
        (self.user, self.scope)
    }
}

#[derive(Debug, MultipartForm)]
pub struct Form {
    #[multipart(limit = "100MB")]
    pub file: TempFile,
    pub json: MpJson<Data>,
}
