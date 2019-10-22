use std::io::stdin;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    //打印游戏提示
    println!("猜数字游戏开始");
    //产生一个不可变的随机数字
    let nu = rand::thread_rng().gen_range(1, 100);
    //一个无限循环
    loop {
        println!("请输入您的结果");
        //定义一个空的字符串变量
        let mut strings = String::new();
        stdin().read_line(&mut strings).expect("输入接收错误");
        let x: u32 = match strings.trim().parse() {
            Ok(mum) => mum,
            Err(_) => {
                println!("您输入的数字错误");
                continue
            },
        };
        println!("您入您的结果是{}",x);
        //猜中后跳出循环
        match x.cmp(&nu) {
            Ordering::Greater=>println!("您输入的结果大了"),
            Ordering::Less=>println!("您输入的结果小了"),
            Ordering::Equal=>{
                println!("您猜中了");
                break
            },
        }
    }
}
