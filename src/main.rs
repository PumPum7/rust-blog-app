use axum::{
    extract::{Multipart, State},
    http::StatusCode,
    response::{Html, Redirect},
    routing::{get, post},
    Router,
};
use chrono::Utc;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tower_http::services::ServeDir;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
struct Blogpost {
    id: String,
    text: String,
    date: String,
    image: Option<String>,
    username: String,
    avatar: Option<String>,
}

struct AppState {
    db: Mutex<Connection>,
}

#[tokio::main]
async fn main() {
    let db = Connection::open("blogposts.db").expect("Failed to open database");
    db.execute(
        "CREATE TABLE IF NOT EXISTS blogposts (
            id TEXT PRIMARY KEY,
            text TEXT NOT NULL,
            date TEXT NOT NULL,
            image TEXT,
            username TEXT NOT NULL,
            avatar TEXT
        )",
        [],
    )
    .expect("Failed to create table");

    let app_state = Arc::new(AppState { db: Mutex::new(db) });

    let app = Router::new()
        .route("/", get(redirect_home))
        .route("/home", get(home))
        .route("/submit", post(submit_post))
        .route("/posts", get(get_posts))
        .nest_service("/images", ServeDir::new("images"))
        .with_state(app_state);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn redirect_home() -> Redirect {
    Redirect::to("/home")
}

async fn home() -> Html<String> {
    Html(include_str!("index.html").to_string())
}

async fn submit_post(
    State(state): State<Arc<AppState>>,
    mut multipart: Multipart,
) -> Result<Redirect, (StatusCode, String)> {
    let mut blogpost = Blogpost {
        id: Uuid::new_v4().to_string(),
        text: String::new(),
        date: Utc::now().to_rfc3339(),
        image: None,
        username: String::new(),
        avatar: None,
    };

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?
    {
        let name = field.name().unwrap().to_string();
        match name.as_str() {
            "text" => {
                blogpost.text = field
                    .text()
                    .await
                    .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?
            }
            "username" => {
                blogpost.username = field
                    .text()
                    .await
                    .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?
            }
            "image" => {
                let filename = format!("images/{}.png", Uuid::new_v4());
                let data = field
                    .bytes()
                    .await
                    .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
                tokio::fs::write(&filename, data)
                    .await
                    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
                blogpost.image = Some(filename);
            }
            "avatar_url" => {
                let url = field
                    .text()
                    .await
                    .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
                let response = reqwest::get(&url)
                    .await
                    .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
                let filename = format!("images/{}.png", Uuid::new_v4());
                let mut file = tokio::fs::File::create(&filename)
                    .await
                    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
                let mut content = std::io::Cursor::new(
                    response
                        .bytes()
                        .await
                        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?,
                );
                tokio::io::copy(&mut content, &mut file)
                    .await
                    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
                blogpost.avatar = Some(filename);
            }
            _ => {}
        }
    }

    let db = state.db.lock().unwrap();
    db.execute(
        "INSERT INTO blogposts (id, text, date, image, username, avatar) VALUES (?, ?, ?, ?, ?, ?)",
        params![
            blogpost.id,
            blogpost.text,
            blogpost.date,
            blogpost.image,
            blogpost.username,
            blogpost.avatar
        ],
    )
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Redirect::to("/home"))
}

async fn get_posts(
    State(state): State<Arc<AppState>>,
) -> Result<Html<String>, (StatusCode, String)> {
    let db = state.db.lock().unwrap();
    let mut stmt = db
        .prepare("SELECT id, text, date, image, username, avatar FROM blogposts ORDER BY date DESC")
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let posts = stmt
        .query_map([], |row| {
            Ok(Blogpost {
                id: row.get(0)?,
                text: row.get(1)?,
                date: row.get(2)?,
                image: row.get(3)?,
                username: row.get(4)?,
                avatar: row.get(5)?,
            })
        })
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let mut html = String::from("<div id='feed'>");
    for post in posts {
        html.push_str(&format!(
            r#"
            <div class='blogpost'>
                <div class='blogpost-header'>
                    {}
                    <h3>{}</h3>
                </div>
                {}
                <p>{}</p>
                <p>Date: {}</p>
            </div>
            "#,
            post.avatar.map_or(String::new(), |avatar| format!(
                r#"<img src='{}' alt='User avatar'>"#,
                avatar
            )),
            post.username,
            post.image.map_or(String::new(), |img| format!(
                r#"<img src='{}' alt='Post image' class='blogpost-image'>"#,
                img
            )),
            post.text,
            post.date
        ));
    }
    html.push_str("</div>");

    Ok(Html(html))
}
