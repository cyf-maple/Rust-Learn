use std::io;

fn main() {
    // simple print 简单打印
    println!("Hello, world!");
    // immutable variable 不可变的 变量
    let x = 5;
    println!("The value of x is: {}", x);
    // cannot mutate immutable variable `x` 无法修改 不可变的 变量 `x`
    // x = 6; // is wrong 错误的

    // change to 修改为
    let x = 6;
    println!("The value of x is: {}", x);
    // or mutable variable 或者改为 可变 变量
    let mut x = 10;
    println!("The value of x is: {}", x);
    x = 20;
    println!("The value of x is: {}", x);

    //constant 常量
    const MAX_POINTS: u32 = 100_000;
    println!("The value of MAX_POINTS is: {}", MAX_POINTS);
    // const MAX_POINTS: u32 = 100_000; // is wrong 错误的
    // A constant has only one name and cannot be mutable.
    // 常量只有一个名称并且不能变

    // Data Type
    // -128 to 127
    let _x: i8 = 0; // - 2 ^ 7 to 2 ^ 7 - 1
                    // 0 to 255
    let _x: u8 = 0; // 0 to 2 ^ 8 - 1
                    // float
    let _x: f64 = 0.0;
    // bool
    let _x: bool = false;
    // char
    let _x: char = '👾';
    // tuple
    let _x: (i32, bool, char) = (1, true, '👾');
    let x: (i8, bool, char) = (1, true, '👾');
    println!("({}, {}, {})", x.0, x.1, x.2);
    // array
    let arr: [i32; 4] = [1, 2, 3, 2];
    println!("{}", arr[2]);

    // input
    let mut input: String = "".to_owned();
    io::stdin().read_line(&mut input).expect("failed to input");
    println!("{}", input);
    let int_input: i32 = input.trim().parse().unwrap();
    println!("{}", int_input + 2);
    //
    //
    //
    //
    //
}
