use axum::{
    response::{IntoResponse, Json},
    routing::get,
    Router,
    extract::{Path},
};
use crate::models::user::User;
use crate::error::AppError;

pub fn router() -> Router {
    Router::new().route("/users/:id", get(get_user))
}

pub async fn get_user(Path(user_id): Path<u32>) -> Result<impl IntoResponse, AppError> {
    let url = format!("https://jsonplaceholder.typicode.com/users/{}", user_id);

    let resp = reqwest::get(&url).await?;

    if resp.status().as_u16() == 404 {
            return Err(AppError::NotFound);
    }

    let user: User = resp.json().await?;
    Ok(Json(user))
}
