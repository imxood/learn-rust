use std::time::Duration;
use tokio::net::UdpSocket;

#[tokio::main]
pub async fn main() {
    println!("Hello, world!");
    let local_sock = tokio::time::timeout(
        Duration::from_millis(500),
        UdpSocket::bind("127.0.0.1:3000"),
    )
    .await
    .expect("udpsocket bind timeout")
    .expect("udpsocket bind failed");
    println!("local_sock: {:?}", &local_sock);
    tokio::time::timeout(
        Duration::from_secs(5),
        local_sock.send_to(b"hello world", "127.0.0.1:1030"),
    )
    .await
    .expect("send timeout")
    .expect("send failed");
    println!("ok");
}
