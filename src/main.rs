#[cfg(feature = "apidoc")]
mod api_doc;

#[cfg(feature = "apidoc")]
use utoipa::OpenApi;

mod app_error;
mod user_post;
mod validated_json;

#[tokio::main]
pub async fn main() {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    let router = {
        let base = axum::Router::new()
            .route("/", axum::routing::get(|| async { "Hello, World!" }))
            .route("/api/users", axum::routing::post(user_post::create));

        #[cfg(feature = "apidoc")]
        {
            let swagger_ui = utoipa_swagger_ui::SwaggerUi::new("/swagger-ui")
                .url("/api-docs/openapi.json", api_doc::ApiDoc::openapi());

            base.merge(swagger_ui)
        }
        #[cfg(not(feature = "apidoc"))]
        {
            base
        }
    };

    axum::serve(listener, router).await.unwrap();
}
