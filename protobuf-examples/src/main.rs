mod protos;
use protobuf::Message;
use protos::example::{GetRequest, GetResponse};

use crate::protos::example::get_response::Status;

fn main() {
    // 构造一个对象
    let mut out_msg = GetRequest::new();
    out_msg.set_name("John Smith".into());
    out_msg.set_age(25);
    out_msg.features.push("one".into());
    out_msg.features.push("two".into());

    // 从 对象 转 字节数组
    let out_bytes = out_msg.write_to_bytes().unwrap();
    println!("out_msg: {:?}", &out_msg);
    println!("out_bytes: {:?}", &out_bytes);

    // 从 对象 转 json字符串
    let json_data = serde_json::to_string(&out_msg).unwrap();
    println!("json_data: {:?}", &json_data);

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

    assert_eq!(in_resp.status, out_resp.status);
    assert_eq!(in_resp.zipcode, out_resp.zipcode);
    assert_eq!(in_resp.get_address(), out_resp.get_address());

    let msg = serde_json::to_string(&out_resp).unwrap();
    println!("msg: {:?}", &msg);

    // 测试合并 1
    let mut req1 = GetRequest::new();
    req1.set_name("John Smith".into());
    println!("test1 req1: {:?}", &req1);

    let mut req2 = GetRequest::new();
    req2.set_age(25);
    println!("req2: {:?}", &req2);

    let ret = req2.write_to_bytes().unwrap();
    req1.merge_from_bytes(&ret).unwrap();

    println!("req1: {:?}", &req1);

    // 测试合并 2 覆盖
    let mut req1 = GetRequest::new();
    req1.set_name("John Smith".into());
    req1.set_age(25);
    println!("test2 req1: {:?}", &req1);

    let mut req2 = GetRequest::new();
    req2.set_name("maxu".into());
    println!("req2: {:?}", &req2);

    let ret = req2.write_to_bytes().unwrap();
    req1.merge_from_bytes(&ret).unwrap();

    println!("req1: {:?}", &req1);
}
