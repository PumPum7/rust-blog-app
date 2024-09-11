use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Blogpost {
    pub id: String,
    pub text: String,
    pub date: String,
    pub image: Option<String>,
    pub username: String,
    pub avatar: Option<String>,
}
