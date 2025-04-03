// 引入 clap 库中的 Arg、ArgAction 和 Command 结构体
// Arg 用于定义命令行参数，ArgAction 用于指定参数的行为，Command 用于构建命令行接口
use clap::{Arg, ArgAction, Command};

/// 注释说明：这个函数用于构建命令行接口，目前有待添加实际的子命令
fn cli_main() -> Command {
    // 创建一个新的命令，命令名称为 "flameforge"
    Command::new("flameforge")
        // 设置命令的描述信息，表明该命令行工具是下一代包管理器
       .about("Next generation package manager")
        // 添加一个命令行参数
       .arg(
            // 定义一个名为 "version" 的参数
            Arg::new("version")
                // 设置参数的短选项为 'v'，即可以通过 -v 来使用该参数
               .short('v')
                // 设置参数的长选项为 "version"，即可以通过 --version 来使用该参数
               .long("version")
                // 设置参数的行为为当参数被使用时，将其对应的值设为 true
               .action(ArgAction::SetTrue),
        )
        // 要求用户至少提供一个参数或子命令，否则显示帮助信息
       .arg_required_else_help(true)
        // 添加一个名为 "version" 的子命令
       .subcommand(Command::new("version").about("Display version and exit"))
}

/// 注释说明：这个函数用于打印程序的版本信息，目前只是打印一个占位符信息
fn print_version() {
    println!("TODO: Set a version");
}

/// 注释说明：程序的主入口点
fn main() {
    // 调用 cli_main 函数构建命令行接口，并解析用户输入的命令行参数
    // get_matches 方法会返回一个包含解析结果的结构体
    let matches = cli_main().get_matches();
    // 检查用户是否使用了 "version" 参数（无论是 -v 还是 --version）
    if matches.get_flag("version") {
        // 如果使用了，调用 print_version 函数打印版本信息
        print_version();
        // 并结束程序的执行
        return;
    }
    // 再次调用 cli_main 函数构建命令行接口并解析用户输入的参数，然后获取子命令
    match cli_main().get_matches().subcommand() {
        // 如果用户输入的子命令是 "version"
        Some(("version", _)) => print_version(),
        // 如果用户输入的子命令不是 "version" 或者没有提供有效的子命令
        // 这里使用 unreachable!() 宏，理论上不应该执行到这里（但目前逻辑存在冗余调用的问题）
        _ => unreachable!(),
    }
}