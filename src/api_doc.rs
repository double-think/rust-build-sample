use crate::{user_post, user_post::UserAddRequest};
use utoipa::openapi::security::{Http, HttpAuthScheme, SecurityScheme};
use utoipa::{Modify, OpenApi};

pub struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components: &mut utoipa::openapi::Components = openapi.components.as_mut().unwrap(); // we can unwrap safely since there already is components registered.
        components.add_security_scheme(
            "bearerAuth",
            SecurityScheme::Http(Http::new(HttpAuthScheme::Bearer)),
        )
    }
}

#[derive(OpenApi)]
#[openapi(paths(
    user_post::create,

), components(schemas(
    UserAddRequest
)), modifiers(&SecurityAddon))]
pub struct ApiDoc;
