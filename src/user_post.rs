use crate::app_error::AppError;
use crate::validated_json::ValidatedJson;
use axum::response::IntoResponse;

#[cfg_attr(feature = "apidoc", derive(utoipa::ToSchema))]
#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, validator::Validate)]
pub struct UserAddRequest {
    #[validate(length(min = 3, max = 30))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 6, max = 100))]
    pub password: String,
}

#[cfg_attr(feature = "apidoc", utoipa::path(
    post,
    path = "/api/users",
    request_body = UserAddRequest,
    responses(
        (status = 200),
        (status = 400, description = "Bad Request"),
        (status = 500, description = "Internal error")
    )
))]
pub async fn create(
    ValidatedJson(user_add_request): ValidatedJson<UserAddRequest>,
) -> Result<impl IntoResponse, AppError> {
    println!("Creating user: {:?}", user_add_request);
    // Implementation of the create function goes here
    Ok("User created successfully")
}
