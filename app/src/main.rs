use hexagonal::DrivenAdapter;
use poem::listener::TcpListener;
use poem::EndpointExt;
use poem::Route;
use poem::Server;
use poem_openapi::OpenApiService;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;
mod dto;

mod handlers;
mod response;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let storage = Arc::new(RwLock::new(HashMap::new()));
    let driven = DrivenAdapter::new(storage);

    let api_service =
        OpenApiService::new(handlers::Api, "hexagonal", "1.0").server("http://localhost:9292/api");

    let ui = api_service.swagger_ui();
    let spec = api_service.spec();

    let route = Route::new()
        .nest("/api", api_service)
        .nest("/", ui)
        .at("/spec", poem::endpoint::make_sync(move |_| spec.clone()))
        .data(driven);

    Server::new(TcpListener::bind("0.0.0.0:9292"))
        .run(route)
        .await
}
