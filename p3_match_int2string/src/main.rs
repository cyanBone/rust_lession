extern crate rand;

use std::cmp::Ordering;
use std::io::stdin;

use rand::{Rng, thread_rng};

fn main() {
    //产生一个随机数负值给常量x
    let x = thread_rng().gen_range(0, 100);
    println!("答案是：{}", x);
    //产生一个空字符串负值给变量strings
    let mut strings = String::new();
    //引用传入一个可变的strings
    stdin().read_line(&mut strings).expect("输入错误");
    //字符转数字
    let nu: u32 = strings.trim().parse().expect("数据转化错误");
    //数字和数字比较
    match nu.cmp(&x) {
        Ordering::Less => println!("猜的数字小了"),
        Ordering::Greater => println!("猜的数字大了"),
        Ordering::Equal => println!("猜的数字对了"),
    }
//    //数字转字符
//    let s: String = x.to_string();
//    //接收的字符串先过滤下在匹配
//    match s == strings.trim() {
//        true => println!("你猜对了"),
//        false => println!("你猜错了")
//    }
}
