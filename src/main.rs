
fn main() {
    // // scalar var
    // let unsigned: u8 = 10;
    // let signed: i8 = -100;
    // let float: f32 = 1.0;
    // println!("unsign: {} sign: {} float: {}", unsigned, signed, float);
    // let _letter= "c1234";
    // let _emoji = "\u{1f600}";
    // println!("letter: {}, emoji: {}", _letter, _emoji);
    // let is_true: bool = true;
    // println!("isTrue:{}", is_true);
    // // array
    // let _arr: [u8; 3] = [1,2,3];
    // let _other_arr: [u8; 5] = [100,200,21,3,5];
    // let _arr2: [u8;5] = [5;5];
    // println!("index: {}, length: {}", _arr[2], _other_arr.len());
    // println!("other array is: {:?}" ,_other_arr);
    // println!(" arr2 is: {:?}", _arr2);
    // // tuple
    // let tuple: (u8,bool,f32) = (5,true,2.1);
    // let mytuple = (7, "sia", -100);
    // println!("first {}, second {} , third {}" , tuple.0, tuple.1, tuple.2);
    // println!("{:?}" , mytuple);

    // let(a,b,c) = tuple;
    // println!("first {} , second {} , third {}" , a ,b , c);


    // call the function in main
    // println!("{}" , is_even(2));


    // mutability
    // let mut num = 5;
    // num = 6;

    // println!("{}" , num);


    // slice array
    // let arr = [5,6,7,8,10];
    // let slice = &arr[0 .. 4];
    // doing_slice(arr, slice);


    // create strings
    // let str = "hello world";
    // let mut string: String = String::from("Hello World");

    // let slice = &string[.. 6];
    // slice.len();

    // string.push('1');
    // string.push_str("!Sia");
    // string = string.replace("Hello" , "Hi");
    // println!("{}" , string);

    //flow control {if}
    // let n = 0;
    // if n > 0 {
    // println!("greater than 0");
    // } else if n < 0 {
    //     println!("less than 0");
    // } else {
    //     println!("is 0");
    // }

    // flow control {for}
    // for i in 0..6 {
    //     println!("{}" , i);
    // }


    // flow control {while}
    // let mut i = 0;
    // while i < 5 {
    //     println!("{}", i);
    //     i += 1;
    //     if i == 3 {
    //         println!("exit");
    //         continue // or break
    //     }
    // }

    //call the function
    // example(25);


    // predefined data and don't we can after change it   
    // let mt_str = "hi";
    
    // this type of data will saved in heap and we can change this variable after
    // let my_string = String::from("new str");


    // ownership
    // let mut a = String::from("hello");
    // a = some_function(a);
    // println!("in main function: {}", a );

    //borrowing - we borrow a value with & (refrence)
    // let  a = String::from("hello");
    // some_function(&a);
    // println!("in main function: {}", a );
    
    // borrowing - i want to change the refrence with &mut
    let  mut a = String::from("hello");
    some_function(&mut a);
    println!("in main function: {}", a );


}


//  create the function out of main
// pub fn is_even(num: u8) -> bool {
//     let digit: u8 = num % 2;
//     digit == 0 // return bool ( not use semicolon; for returning a function)
// }

fn example(number: i16) {
    println!("number {}" , number )
}



// fn doing_slice(arr:[u8; 5] , slice: &[u8]) {
//     println!("arr: {:?}" , arr);
//     println!("slice: {:?}", slice);
//     println!("length: {}", slice.len());
//     println!("{} {}", slice[0], slice[2]);
// }


// fn some_function(input: String) -> String {
//     println!("some function: {}", input);
//     input
// }


// borrowing
// fn some_function(input: &String) {
//     println!("some function: {}", input);
// }

// borrowing with changing refrence - &mut
fn some_function(input: &mut String) {
    input.push_str(" how is it doing?");
    println!("some function: {}", input);
}