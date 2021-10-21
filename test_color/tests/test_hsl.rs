use hsl::HSL;

#[test]
fn test_hsl() {
    let red = [255, 255, 0];
    let red_hsl = HSL::from_rgb(&red);
    println!("red_hsl: {:?}", &red_hsl);

    let yellow = [255, 255, 0];
    let yellow_hsl = HSL::from_rgb(&yellow);
    println!("yellow_hsl: {:?}", &yellow_hsl);
}
