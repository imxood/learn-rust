use serde_derive::{Deserialize, Serialize};

#[test]
fn test_json() {
    #[derive(Deserialize, Serialize, Debug)]
    pub struct Distance {
        distance: usize,
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub enum InteractiveStyle {
        None,
        Camera(Distance),
        Origin,
    }

    let interactive_style = InteractiveStyle::None;
    println!("{:?}", serde_json::to_string(&interactive_style).unwrap());

    let interactive_style = InteractiveStyle::Camera(Distance { distance: 11 });
    println!("{:?}", serde_json::to_string(&interactive_style).unwrap());

    let interactive_style = InteractiveStyle::Origin;
    println!("{:?}", serde_json::to_string(&interactive_style).unwrap());

    let c = '1';
    println!("{:?}", serde_json::to_string(&c).unwrap());

    let i = c.to_digit(10).unwrap();
    println!("i: {}", i);

    let v = vec![1, 2, 3];
    println!("{:?}", serde_json::to_string(&v).unwrap());
}

#[test]
fn test_toml() {
    #[derive(Serialize, Deserialize, Debug)]
    struct A {
        a: usize,
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct B {
        a: A,
        b: usize,
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct C {
        a: A,
        b: B,
        c: usize,
    }

    let c = C {
        a: A { a: 1 },
        b: B {
            a: A { a: 1 },
            b: 1,
        },
        c: 1,
    };

    let recipe_toml = toml::Value::try_from(c).unwrap();
    println!("recipe_toml(fails) = {}", recipe_toml.to_string());

    // println!("{:?}", toml::to_string(&c).unwrap());
}

#[test]
fn test_bytes() {
    let mut device = Device::new();
    let mut data: Vec<u8> = vec![1, 2, 3, 4, 5, 6];
    device.data.append(&mut data);
    let json_data = serde_json::to_string(&device).unwrap();
    println!("json_data: {:?}", &json_data);
}

#[derive(Debug, Serialize, Deserialize)]
struct Device {
    pub ip: String,
    pub port: u16,
    #[serde(with = "serde_bytes")]
    pub data: Vec<u8>,
}

impl Device {
    fn new() -> Self {
        Self {
            ip: "127.0.0.1".into(),
            port: 12345,
            data: Vec::new(),
        }
    }
}
