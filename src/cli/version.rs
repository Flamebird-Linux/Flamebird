// src/cli/version.rs

// 引入 clap 库中的 Command 结构体，用于构建命令行接口
use clap::Command;

// 定义一个不可变的常量 VERSION，其类型为字符串切片
// env!("CARGO_PKG_VERSION") 是一个宏调用，它会在编译时从 Cargo.toml 文件里获取项目的版本号
// 后续在需要显示版本信息的地方，就可以使用这个常量
const VERSION: &str = env!("CARGO_PKG_VERSION");

// 定义一个公共函数 command Command 实例
// 该函数的作用是创建一个名为 "version" 的命令行子命令
pub fn command() -> Command {
    // 使用 clap 的 Command::new 方法创建一个新的命令，命令名称为 "version"
    Command::new("version")
        // 设置该命令的简要描述信息
       .about("Display version and exit")
}

// 定义一个公共函数 print_version，用于打印程序的版本信息
pub fn print() {
    // 使用 println! 宏将程序名称 "flameforge" 和版本号打印到标准输出
    // 字符串中的 {} 是格式化字符串的占位符，会被常量 VERSION 的实际值替换
    println!("flameforge {}", VERSION);
}