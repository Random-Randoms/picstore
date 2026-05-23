use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Form {
    pub user: String,
    pub scope: String,
    pub picture: i64,
}

impl Form {
    pub fn into_parts(self) -> (String, String, i64) {
        (self.user, self.scope, self.picture)
    }
}
