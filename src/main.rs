use std::net::SocketAddr;
use axum::routing::get;
use utoipa::{OpenApi, ToSchema};
use utoipa_scalar::{Scalar, Servable};
use serde::{Serialize, Deserialize};
use axum::{extract::Path, response::IntoResponse, http::StatusCode};
use uuid::Uuid;

#[derive(OpenApi)]
#[openapi(paths(get_user_by_id))]
/// API
struct ApiDoc;

/// User Account
#[derive(Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
/// Represents a user account within the business.
pub struct User {
    #[schema(
        example = "550e8400-e29b-41d4-a716-446655440000",
    )]
    /// Unique identifier for the user.
    pub uuid: Uuid,
    #[schema(
        example = "Jane",
    )]
    /// First name of the user.
    pub first_name: String,
    #[schema(
        example = "Doe",
    )]
    /// Last name of the user.
    pub last_name: String,
    #[schema(
        example = "jane.doe@example.com",
    )]
    /// Email address of the user.
    pub email: String,
    #[schema(
        example = true,
    )]
    /// Whether the user's account is enabled.
    pub enabled: bool,
    #[schema(
        example = true,
    )]
    /// Whether the user's account is activated.
    pub activated: bool,
}

/// Get user account by user id
#[utoipa::path(
    get,
    path = "/business/{businessId}/users/{userId}",
    responses(
        (status = 200, description = "User", body = User)
    ),
    params(
        ("businessId" = Uuid, Path, description = "Business id of the user"),
        ("userId" = String, Path, description = "User id to get user"),
    )
)]
async fn get_user_by_id(Path(user_id): Path<Uuid>) -> impl IntoResponse {
    if user_id == Uuid::nil() {
        return (StatusCode::NOT_FOUND, "User not found").into_response();
    }
    let user = User {
        uuid: user_id,
        first_name: "Jane".to_string(),
        last_name: "Doe".to_string(),
        email: "jane.doe@example.com".to_string(),
        enabled: true,
        activated: true,
    };
    match serde_json::to_string(&user) {
        Ok(body) => (StatusCode::OK, body).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Unknown error").into_response(),
    }
}

#[tokio::main]
async fn main() {
    let socket_address: SocketAddr = "127.0.0.1:8080".parse().unwrap();
    let listener = tokio::net::TcpListener::bind(socket_address).await.unwrap();

    let html = r#"
<!doctype html>
<html>
<head>
    <title>API</title>
    <meta charset="utf-8"/>
    <meta
            name="viewport"
            content="width=device-width, initial-scale=1"/>
</head>
<body>

<script
        id="api-reference"
        data-configuration='{"theme":"laserwave"}'
        type="application/json">
    $spec
</script>
<script src="https://cdn.jsdelivr.net/npm/@scalar/api-reference"></script>
</body>
</html>
"#;

    let app = axum::Router::new()
        .route("/users/{user_id}", get(get_user_by_id))
        .merge(Scalar::with_url(
            "/api",
            ApiDoc::openapi()
        ).custom_html(html));

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap()
}
