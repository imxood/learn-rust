use poem::{EndpointExt, Route, Server, get, listener::TcpListener, middleware::Cors, post};

use crate::req;

pub async fn start_web(ip: &str, port: u16) -> Result<(), std::io::Error> {
  let app = Route::new()
    .at("/ws", get(req::ws))
    .at("/hello", get(req::hello))
    .at("/proto", post(req::proto))
    .at("/proto_json", post(req::proto_json))
    .with(Cors::new());

  let web_addr = format!("{}:{}", ip, port);
  log::debug!("web_addr: {:?}", &web_addr);

  let listener = TcpListener::bind(web_addr);
  let server = Server::new(listener).await?;
  server.run(app).await
}
