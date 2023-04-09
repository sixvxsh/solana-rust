fn main() {
    // // scalar var
    let unsigned: u8 = 10;
    let signed: i8 = -100;
    let float: f32 = 1.0;
    println!("unsign: {} sign: {} float: {}", unsigned, signed, float);
    let _letter= "c1234";
    let _emoji = "\u{1f600}";
    println!("letter: {}, emoji: {}", _letter, _emoji);
    let is_true: bool = true;
    println!("isTrue:{}", is_true);
    // // array
    let _arr: [u8; 3] = [1,2,3];
    let _other_arr: [u8; 5] = [100,200,21,3,5];
    let _arr2: [u8;5] = [5;5];
    println!("index: {}, length: {}", _arr[2], _other_arr.len());
    println!("other array is: {:?}" ,_other_arr);
    println!(" arr2 is: {:?}", _arr2);
    // tuple
    let tuple: (u8,bool,f32) = (5,true,2.1);
    let mytuple = (7, "sia", -100);
    println!("first {}, second {} , third {}" , tuple.0, tuple.1, tuple.2);
    println!("{:?}" , mytuple);

    let(a,b,c) = tuple;
    println!("first {} , second {} , third {}" , a ,b , c);


    // call the function in main
    println!("{}" , is_even(2));


    // mutability
    let mut num = 5;
    num = 6;

    println!("{}" , num);
}


//  create the function out of main
pub fn is_even(num: u8) -> bool {
    let digit: u8 = num % 2;
    digit == 0 // return bool ( not use semicolon; for returning a function)
}
