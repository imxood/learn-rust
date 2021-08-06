use serde_json::Value;

const JSON: &str = r#"{
  "name": "John Doe",
  "age": 43
}"#;

#[test]
fn test_json() {
    let v: Value = serde_json::from_str(JSON).unwrap();
    println!("{} is {} years old", v["name"], v["age"]);
}