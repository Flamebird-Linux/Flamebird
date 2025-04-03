// src/cli/remove.rs

// 引入 clap 库中的 Command 结构体，用于构建命令行接口
use clap::Command;

// 定义一个公共函数 command Command 实例
// 该函数的作用是创建一个名为 "remove" 的命令行子命令
pub fn command() -> Command {
    // 使用 clap 的 Command::new 方法创建一个新的命令，命令名称为 "remove"
    Command::new("remove")
        // 设置该命令的简要描述信息
        .about("Remove a package")
        // 设置该命令的长描述信息，提供更多的上下文信息
        .long_about("Remove a package from the local system")
}