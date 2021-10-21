#[test]
fn test_vec() {
    let mut arr1 = vec![0; 5];
    let mut arr2: Vec<i32> = Vec::new();
    arr2.push(1);
    arr2.push(2);
    println!("arr1: {:?}, arr2: {:?}", &arr1, &arr2);
    arr1.append(&mut arr2);
    println!("arr1: {:?}, arr2: {:?}", &arr1, &arr2);
}

#[test]
fn test_vec2() {
    let datas = [1, 3, 5, 7, 2, 4, 6, 8];
    let mut ret: Vec<bool> = datas.iter().map(|x| *x > 5).collect();
    ret.retain(|x| *x);
    println!("大于5的数量是: {}", ret.len());
}

#[test]
fn test_vec3() {
    let datas = [1, 3, 5, 7, 2, 4, 6, 8];
    let mut count = datas.iter().filter(|x| **x > 5).count();
    println!("大于5的数量是: {}", count);
}

#[test]
fn test_vec4() {
    let data1 = [
        1, 2, 3, 4,
        5, 6, 7, 8,
        9, 10, 11, 12,
        13,14,15,16
    ]; // 4 * 4 行列数据

    let width = 4;
    let height = 4;

    let (x,y, w, h) = (1, 1, 2, 2);

    for i in 0..h {
        let y1 = y + i;
        for j in 0..w { 
            let x1 = x + j;
            println!("({},{})", x1, y1);
        }
    }
}
