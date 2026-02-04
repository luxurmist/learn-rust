// format! 将格式化文本写到 字符串String/&str
// print! 将格式化文本输出到 标准输出io::stdout
// println! 另追加一个 换行符\n
// eprint! 输出到 标准错误io::stderr
// eprintln! 另追加一个 换行符\n

fn main() {
    println!("formatted_print");
    // 空格式
    println!("{}", 255);
    println!("0-{}", 255u8);
    println!("{} or {}", "yes", "no");
    // 位置参数
    println!("{1} or {0}, {0} or {1}", "yes", "no");
    // 命名参数
    println!("{yes}/{no}", yes = "Y", no = "N");
    // 指定格式
    println!(
        "o{:o} = b{:b} = x{:x} = X{:X}  = e{:e} = E{:E}",
        32, 32, 32, 32, 32, 32
    );
    // 指定宽度，Dollar符号指定由哪个参数指定宽度，需为整数
    println!("{number:>width$}", number = 1, width = 8);
    println!("|{1:0$}|", 8, "x");
    // <左对齐 ^居中对齐 >右对齐
    println!("|{1:>8}|{0:<8}|{2}|{3:^8}|", 3, 4, 5, 6);
    // 填充，默认空格，数字0填充0
    println!("{number:>0width$}", number = 1, width = 7);
    let width = 7;
    println!("|{:^width$}|", "x");
    // Debug格式化
    println!("{:?}", (17, 18));
    let name = "axe";
    println!("{name} is axe");
    let pi = 3.1415926;
    println!("pi: |{:8.2}|", pi);
    // + 始终输出符号
    println!("|{:+}|", 8);
    // #? Debug格式  #x #X #b #o 输出格式
    println!("|{:+#08x}|", 42);
}
