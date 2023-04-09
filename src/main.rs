fn main() {
    let unsigned: u8 = 10;
    let signed: i8 = -100;
    let float: f32 = 1.0;
    println!("unsign: {} sign: {} float: {}", unsigned, signed, float);

    let _letter= "c1234";
    let _emoji = "\u{1f600}";
    println!("letter: {}, emoji: {}", _letter, _emoji);

    let is_true: bool = true;
    println!("isTrue:{}", is_true);

    let _arr: [u8; 3] = [1,2,3];
    let _other_arr: [u8; 5] = [100,200,21,3,5];
    println!("index: {}, length: {}", _arr[2], _other_arr.len());
    println!("other array is: {:?}" ,_other_arr);
}
