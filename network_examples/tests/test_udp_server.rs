use std::net::UdpSocket;

#[test]
fn test_udp_server() -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:10000")?;

    loop {
        let mut buf = [0u8; 1500];
        let (amt, src) = socket.recv_from(&mut buf)?;

        println!("server recv size: {}", &amt);
        let buf = &mut buf[..amt];
        buf.reverse();
        socket.send_to(buf, &src)?;
    }
}


#[test]
fn test_udp_server1() -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:10000")?;

    loop {
        let mut buf = [0u8; 1500];
        let (amt, src) = socket.recv_from(&mut buf)?;

        println!("server recv size: {}", &amt);
    }
}
