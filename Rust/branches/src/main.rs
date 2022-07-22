//控制流

fn main() {

    //if

    let number = 3;

    //代码中的条件 必须 是 bool 值
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
    

    //因为 if 是一个表达式，我们可以在 let 语句的右侧使用它
    let condition = true;
    //因为变量必须只有一个类型。Rust 需要在编译时就确切的知道 number2 变量的类型
    //let number2 = if condition { 5 } else { "six" };
    let number2 = if condition { 5 } else { 6 };

    println!("The value of number2 is: {number2}");


    //loop

    //loop 关键字告诉 Rust 一遍又一遍地执行一段代码直到你明确要求停止。
    loop {
        println!("again!");
        //break 关键字来告诉程序何时停止循环。
        break;
        //循环中的 continue 关键字告诉程序跳过这个循环迭代中的任何剩余代码，并转到下一个迭代。
    }


    
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    //循环标签（loop label）你可以选择在一个循环上指定一个 循环标签（loop label），然后将标签与 break 或 continue 一起使用，使这些关键字应用于已标记的循环而不是最内层的循环。
    //你可以选择在一个循环上指定一个 循环标签（loop label），然后将标签与 break 或 continue 一起使用，使这些关键字应用于已标记的循环而不是最内层的循环。
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");


    //while
    let mut number3 = 3;

    while number3 != 0 {
        println!("{number3}!");

        number3 -= 1;
    }

    println!("LIFTOFF!!!");


/*Docs book https://kaisery.github.io/trpl-zh-cn/ch03-05-control-flow.html?highlight=branches#%E4%BD%BF%E7%94%A8-for-%E9%81%8D%E5%8E%86%E9%9B%86%E5%90%88
for 循环的安全性和简洁性使得它成为 Rust 中使用最多的循环结构。
即使是在想要循环执行代码特定次数时，例如上面使用 while 循环的倒计时例子，大部分 Rustacean 也会使用 for 循环。
这么做的方式是使用 Range，它是标准库提供的类型，用来生成从一个数字开始到另一个数字之前结束的所有数字的序列。
*/


    //使用for遍历集合
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }


    //作为更简洁的替代方案，可以使用 for 循环来对一个集合的每个元素执行一些代码。
    let b = [10, 20, 30, 40, 50];

    for element in b {
        println!("the value is: {element}");
    }

    //下面是一个使用 for 循环来倒计时的例子，它还使用了一个我们还未讲到的方法，rev，用来反转 range
    for number4 in (1..4).rev() {
        println!("{number4}!");
    }
    println!("LIFTOFF!!!");
}
