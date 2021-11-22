mod protos;
use protobuf::Message;
use protos::example::{GetRequest, GetResponse};

use crate::protos::example::get_response::Status;

fn main() {
    let out_msg = GetRequest::default_instance();
    println!("out_msg: {:?}", out_msg);

    // 构造一个对象
    let mut out_msg = GetRequest::new();
    out_msg.set_name("John Smith".into());
    // out_msg.set_age(25);
    out_msg.features.push("one".into());
    out_msg.features.push("two".into());

    // 从 对象 转 字节数组
    let out_bytes = out_msg.write_to_bytes().unwrap();
    println!("out_msg: {:?}", &out_msg);
    println!("out_bytes: {:?}", &out_bytes);

    // let json_data = serde_json::to_string(&out_msg).unwrap();
    // println!("json_data: {:?}", &json_data);

    //////////////////////////////////////
    // 从 字节数组 转 对象
    let in_msg: GetRequest = Message::parse_from_bytes(&out_bytes).unwrap();

    let in_name = in_msg.get_name();

    assert_eq!(in_name, "John Smith");

    //////////////////////////////////

    // 构造一个对象
    let mut out_resp = GetResponse::new();
    out_resp.set_status(Status::OK);
    out_resp.set_address("1243 main street".into());
    out_resp.set_city("anytown".into());
    out_resp.set_zipcode(54321);
    out_resp.set_data(vec![1, 2, 3, 4, 5, 6]);

    //////////////////////////////////////
    // 从 对象 转 字节数组
    let out_bytes: Vec<u8> = out_resp.write_to_bytes().unwrap();

    println!("out_resp: {:?}", &out_resp);
    println!("out_bytes: {:?}", &out_bytes);

    // 从 字节数组 转 对象
    let in_resp: GetResponse = Message::parse_from_bytes(&out_bytes).unwrap();

    // let json_data = serde_json::to_string(&out_resp).unwrap();
    // println!("json_data: {:?}", &json_data);

    assert_eq!(in_resp.status, out_resp.status);
    assert_eq!(in_resp.zipcode, out_resp.zipcode);
    assert_eq!(in_resp.get_address(), out_resp.get_address());

    let msg = serde_json::to_string(&out_resp).unwrap();
    println!("msg: {:?}", &msg);

    // let s = vec!["hello", "world!"];
    // let s: Vec<Chars> = s.iter().map(|x| Chars::from(*x)).collect();
    // let mut ss = Strings::new();
    // ss.set_item(s);
    // println!("out_bytes: {:?}", ss.write_to_bytes().unwrap());
    // let ss = serde_json::to_string(&ss).unwrap();
    // println!("ss: {:?}", &ss);
}
