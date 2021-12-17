use std::{net::UdpSocket, time::Duration};

#[test]
fn test_udp_client() {
    let socket = UdpSocket::bind("127.0.0.1:10001").unwrap();
    if let Err(e) = socket.connect("127.0.0.1:10000") {
        println!("upd spcket 连接失败, e: {:?}", &e);
        return;
    }
    loop {
        let mut buf = [0u8; 10];
        let size = socket.send(&mut [1, 2, 3, 4, 5, 6]).unwrap();
        println!("send size: {}", size);
        let ret = socket.recv_from(&mut buf);
        println!("ret: {:?}, buf: {:?}", &ret, &buf);
        std::thread::sleep(Duration::from_millis(2000));
    }
}

// #[test]
// fn test_udp_client1() {
//     let socket = UdpSocket::bind("127.0.0.1:10001").unwrap();
//     if let Err(e) = socket.connect("127.0.0.1:10000") {
//         println!("upd spcket 连接失败, e: {:?}", &e);
//         return;
//     }
//     loop {
//         let mut buf = [0u8; 10];
//         let size = socket.send(&mut [1, 2, 3, 4, 5, 6]).unwrap();
//         println!("send size: {}", size);
//         std::thread::sleep(Duration::from_millis(2000));
//     }
// }