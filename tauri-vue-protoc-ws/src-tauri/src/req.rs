use crate::{
  binary::Binary,
  protos::example::{get_response::Status, GetRequest, GetResponse},
};
use futures_util::{SinkExt, StreamExt};
use poem::{
  handler,
  web::{
    websocket::{Message as WsMessage, WebSocket},
    Json,
  },
  IntoResponse,
};
use protobuf::Message;

#[handler]
pub async fn hello() -> impl IntoResponse {
  "hello"
}

#[handler]
pub async fn proto(bin: Binary) -> impl IntoResponse {
  let req = GetRequest::parse_from_bytes(&*bin).unwrap();
  log::info!("{:?}", &req);
  let data = req.write_to_bytes().unwrap();
  log::info!("{:?}", &data);
  Binary(data)
}

#[handler]
pub async fn proto_json(bin: Binary) -> impl IntoResponse {
  let req = GetRequest::parse_from_bytes(&*bin).unwrap();
  log::info!("{:?}", &req);
  let data = Json(req);
  log::info!("{:?}", &data);
  data
}

/*
    定义 websocket 行为
*/

#[handler]
pub async fn ws(ws: WebSocket) -> impl IntoResponse {
  ws.on_upgrade(move |socket| async move {
    log::info!("websocket connect~");
    let (mut sink, mut stream) = socket.split();

    // 接收来自websocket client的消息
    while let Some(Ok(msg)) = stream.next().await {
      match msg {
        WsMessage::Ping(msg) => {
          log::info!("recv Ping req");
        }
        WsMessage::Text(msg) => {
          log::info!("text msg: {:?}", &msg);
        }
        WsMessage::Binary(msg) => {
          let req = GetRequest::parse_from_bytes(&msg).unwrap();
          log::info!("get req: {:?}", &req);
          let mut res = GetResponse::new();
          res.status = Status::OK.into();
          res.address = "beijing".into();
          let data = res.write_to_bytes().unwrap();
          log::info!("data: {:?}", &data);
          sink.send(WsMessage::binary(data)).await.unwrap();
        }
        _ => (),
      }
    }
    log::info!("stream ended");
  })
}
