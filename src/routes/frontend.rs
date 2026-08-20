use askama::Template;
use axum::{
    Form, Router,
    response::{Html, IntoResponse, Redirect, Response},
    routing::get,
};
use axum_extra::extract::{CookieJar, cookie::Cookie};
use serde::Deserialize;

use crate::{
    app::AppState,
    auth::user::{UnauthenticatedUser, User},
    error::AppError,
    repository::Repository,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(index))
        .route("/login", get(login_page).post(login))
        .route("/dashboard", get(dashboard))
        .route("/modal/new-asset", get(new_asset_modal))
        .route("/modal/edit-asset/{id}", get(edit_asset_modal))
}

#[derive(Template)]
#[template(path = "login.html")]
struct LoginPage;

async fn login_page() -> Result<Html<String>, AppError> {
    let html = LoginPage.render()?;
    Ok(Html(html))
}

#[derive(Deserialize)]
struct LoginForm {
    username: String,
    password: String,
}

async fn login(
    repository: Repository,
    jar: CookieJar,
    Form(request): Form<LoginForm>,
) -> Result<impl IntoResponse, AppError> {
    let unauth_user = UnauthenticatedUser::new(request.username, request.password);
    let user = match unauth_user.authenticate(&repository).await {
        Ok(user) => user,
        Err(AppError::UserDoesNotExist) => unauth_user.register(&repository).await?,
        Err(other_err) => return Err(other_err),
    };

    let token = user.auth_token()?;
    let cookie = Cookie::build(("token", token)).http_only(true);

    Ok((jar.add(cookie), Redirect::to("/dashboard")))
}

async fn index(maybe_user: Option<User>) -> Result<Response, AppError> {
    match maybe_user {
        Some(_) => Ok(Redirect::to("/dashboard").into_response()),
        None => Ok(Redirect::to("/login").into_response()),
    }
}

#[derive(Template)]
#[template(path = "dashboard.html")]
struct DashboardPage;

async fn dashboard(_user: User) -> Result<Html<String>, AppError> {
    let html = DashboardPage.render()?;
    Ok(Html(html))
}

#[derive(Template)]
#[template(path = "new_asset_modal.html")]
struct NewAssetModal;

async fn new_asset_modal() -> Result<Html<String>, AppError> {
    let html = NewAssetModal.render()?;
    Ok(Html(html))
}

#[derive(Template)]
#[template(path = "edit_asset_modal.html")]
struct EditAssetModal {
    asset: crate::models::Asset,
}

async fn edit_asset_modal(
    repository: Repository,
    axum::extract::Path(id): axum::extract::Path<i64>,
) -> Result<Html<String>, AppError> {
    let assets = repository.list_assets().await?;
    let asset = assets.into_iter().find(|a| a.id == id)
        .ok_or(AppError::AssetDoesNotExist)?;

    let html = EditAssetModal { asset }.render()?;
    Ok(Html(html))
}
