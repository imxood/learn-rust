
#[test]
fn test_error() {
    
    fn stringify(x: u32) -> String { format!("error code: {}", x) }

    let x: Result<u32, u32> = Err(13);
    x.map_err(op)
    /// assert_eq!(x.map_err(stringify), Err("error code: 13".to_string()));
}