use poem::{
    handler, http::StatusCode, listener::TcpListener, route, route::get, web::Path, Result, Server,
};

#[handler]
fn hello(Path(name): Path<String>) -> Result<String> {
    Err(poem::Error::new(StatusCode::BAD_REQUEST).with_reason_string(format!("hello: {}", name)))
    // Ok(format!("hello: {}", name))
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug");
    }
    tracing_subscriber::fmt::init();

    let app = route().at("/hello/:name", get(hello));
    let listener = TcpListener::bind("127.0.0.1:3000");
    let server = Server::new(listener).await?;
    server.run(app).await
}
