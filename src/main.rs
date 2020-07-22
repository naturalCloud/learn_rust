

use std::io;//ues 导入包

use rand::{ Rng};
use std::cmp::Ordering;

fn main() {
    println!("you input string is {} ", guess_num());
}


fn guess_num() -> u32 {
    println!("game begin \n");

    //mut 可变变量
    // let aa 不可变变量
    let mut guess = String::new();

    //从标准输入读取
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    //随机数
    let secret_number = rand::thread_rng().gen_range(1, 101);

    //转换字符串为数字
    let guess: u32  = guess.trim().parse().expect("转转失败");

    //switch case 类似
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("small"),
        Ordering::Equal => {println!("相等")},
        Ordering::Greater => {println!("大于")}
    }
    println!("{}",secret_number);

    return guess;
}