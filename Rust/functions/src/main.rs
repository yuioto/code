fn main() {
    println!("Hello, world!");
    another_function(5);
    print_labeled_measurement(5, 'h');


    //语句与表达式
    //作用域{}是一个表达式
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");

    //函数的返回值等同于函数体最后一个表达式的值
    let z = five();
    println!("The value of z is: {z}");

    let a = plus_one(5);
    println!("The value of a is: {a}");
}

fn another_function(x:i32) {
    println!("Another function.");
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    //只有最后一个返回值被返回
    //;为一个语句
    //9;
    5
}

fn plus_one(a: i32) -> i32 {
    a + 1
    //“mismatched types”（类型不匹配）
    //函数 plus_one 的定义说明它要返回一个 i32 类型的值
    //；被定义为语句
    //a + 1;
}
