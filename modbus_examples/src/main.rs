use modbus::Config;

fn main() {
    use modbus::tcp;
    use modbus::{Client, Coil};

    let mut client = tcp::Transport::new_with_cfg("192.0.0.80", Config);

    client.write_single_coil(1, Coil::On).unwrap();
    client.write_single_coil(3, Coil::On).unwrap();

    let res = client.read_coils(0, 5).unwrap();
}
