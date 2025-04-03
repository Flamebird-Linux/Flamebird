// main.rs

// 引入 clap 库中的 Command 结构体，用于构建命令行接口
use clap::Command;

/// TODO：添加当前子命令！
// 定义一个名为 cli_main 的函数，该函数返回一个 Command 实例
// 此函数的作用是构建整个命令行接口的结构
fn cli_main() -> Command {
    // 创建一个新的命令，名称为 "flameforge"
    // 这个名称将作为整个命令行工具的主命令名称
    Command::new("flameforge")
        // 设置该命令的描述信息，简要说明该命令行工具的用途
        .about("下一代包管理器")
        // 要求用户至少提供一个参数或子命令，否则显示帮助信息
        .arg_required_else_help(true)
        // 要求用户必须提供一个子命令，否则显示帮助信息
        .subcommand_required(true)
        // 为该命令添加一个子命令，名称为 "version"
        .subcommand(Command::new("version")
            // 设置 "version" 子命令的描述信息，说明该子命令的作用
            .about("显示版本并退出"))
}

/// 主程序入口点，程序从这里开始执行
fn main() {
    // 调用 cli_main 函数构建命令行接口，并解析用户输入的命令行参数
    // get_matches 方法会返回一个包含解析结果的结构体
    // subcommand 方法用于获取用户输入的子命令及其参数
    match cli_main().get_matches().subcommand() {
        // 如果用户输入的子命令是 "version"
        Some(("version", _)) => {
            // 打印版本信息相关的提示语
            println!("让我休息一下我才刚刚开始！");
        }
        // 如果用户输入的子命令不是 "version" 或者没有提供有效的子命令
        _ => {
            // 打印一个提示信息，说明需要实现一个真正的命令行接口
            println!("我们应该实现一个真正的 CLI，嗯？");
        }
    }
}