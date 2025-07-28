use crate::app_error::AppError;
use axum::{
    Json, async_trait,
    body::Bytes,
    extract::{FromRequest, Request},
};
use serde::de::DeserializeOwned;
use validator::Validate;

pub struct ValidatedJson<T>(pub T);

#[async_trait]
impl<S, T> FromRequest<S> for ValidatedJson<T>
where
    S: Send + Sync,
    T: DeserializeOwned + Validate,
    Bytes: FromRequest<S>,
{
    type Rejection = AppError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let json = Json::<T>::from_request(req, state)
            .await
            .map_err(AppError::JsonRejection)?
            .0;
        json.validate().map_err(AppError::Validation)?;
        Ok(ValidatedJson(json))
    }
}
