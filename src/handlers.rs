use crate::errors::AppError;
use crate::models::Blogpost;
use crate::state::AppState;
use axum::{
    extract::{Multipart, State},
    response::{Html, Redirect},
};
use chrono::Utc;
use sqlx::{Row};
use std::sync::Arc;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use uuid::Uuid;

pub async fn redirect_home() -> Redirect {
    Redirect::to("/home")
}

pub async fn home() -> Html<String> {
    Html(include_str!("index.html").to_string())
}

pub async fn submit_post(
    State(state): State<Arc<AppState>>,
    mut multipart: Multipart,
) -> Result<Redirect, AppError> {
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
        .map_err(|e| AppError::BadRequest(e.to_string()))?
    {
        let name = field.name().unwrap().to_string();
        match name.as_str() {
            "text" => {
                blogpost.text = field
                    .text()
                    .await
                    .map_err(|e| AppError::BadRequest(e.to_string()))?;
            }
            "username" => {
                blogpost.username = field
                    .text()
                    .await
                    .map_err(|e| AppError::BadRequest(e.to_string()))?;
            }
            "image" => {
                if let Ok(data) = field.bytes().await {
                    if !data.is_empty() {
                        let filename = format!("images/{}.png", Uuid::new_v4());
                        tokio::fs::write(&filename, data).await?;
                        blogpost.image = Some(filename);
                    }
                }
            }
            "avatar_url" => {
                if let Ok(url) = field.text().await {
                    if !url.is_empty() {
                        let response = reqwest::get(&url)
                            .await
                            .map_err(|e| AppError::RequestError(e))?;
                        let filename = format!("images/{}.png", Uuid::new_v4());
                        let mut file = File::create(&filename).await?;
                        let content = response
                            .bytes()
                            .await
                            .map_err(|e| AppError::RequestError(e))?;
                        file.write_all(&content).await?;
                        blogpost.avatar = Some(filename);
                    }
                }
            }
            _ => {}
        }
    }

    sqlx::query(
        "INSERT INTO blogposts (id, text, date, image, username, avatar) VALUES (?, ?, ?, ?, ?, ?)"
    )
        .bind(&blogpost.id)
        .bind(&blogpost.text)
        .bind(&blogpost.date)
        .bind(&blogpost.image)
        .bind(&blogpost.username)
        .bind(&blogpost.avatar)
        .execute(&state.db)
        .await?;

    Ok(Redirect::to("/home"))
}

pub async fn get_posts(State(state): State<Arc<AppState>>) -> Result<Html<String>, AppError> {
    let posts = sqlx::query(
        "SELECT id, text, date, image, username, avatar FROM blogposts ORDER BY date DESC"
    )
        .fetch_all(&state.db)
        .await?
        .into_iter()
        .map(|row| Blogpost {
            id: row.get("id"),
            text: row.get("text"),
            date: row.get("date"),
            image: row.get("image"),
            username: row.get("username"),
            avatar: row.get("avatar"),
        })
        .collect::<Vec<_>>();

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
            post.date.split('T').next().unwrap_or(&post.date)
        ));
    }
    html.push_str("</div>");

    Ok(Html(html))
}