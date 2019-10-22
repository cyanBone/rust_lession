fn main() {
    //打印hello, world
    println!("Hello, world!");
    //声明一个strings变量
    //mut为变量标记
    //String::new()产生一个空的String类型负值给strings变量
    let mut strings = String::new();
    //终端输入
    //expect是如果出错就返回无法读取行
    //此处可以在开头用use std::io;来省去std::io::为io::
    //read_line返回一个Result<T, Error>枚举类型的变量
    //expect可以当read_line返回错误时panic出"无法读取行"
    std::io::stdin().read_line(&mut strings).expect("无法读取行");
    //打印strings里的内容
    //{}为占位符，多个参数用","隔开
    println!("你输入的数据为：{}",strings);
}
