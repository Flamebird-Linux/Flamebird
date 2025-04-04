// src/cli/mod.rs

// 导入 clap 库中的 Arg、ArgAction 和 Command 结构体，用于构建命令行接口
use clap::{Arg, ArgAction, Command};

// 引入当前模块下的 info 子模块 文件路径为 src/cli/info.rs
mod info;
// 引入当前模块下的 install 子模块 文件路径为 src/cli/install.rs
mod install;
// 引入当前模块下的 mod 子模块 文件路径为 src/cli/list.rs
mod list;
// 引入当前模块下的 remove 子模块 文件路径为 src/cli/remove.rs
mod remove;
// 引入当前模块下的 version 子模块 文件路径为 src/cli/version.rs
mod version;

// 定义 command 函数，该函数返回一个 Command 结构体实例
fn command() -> Command {
    // 创建一个新的命令，名称为 "flameforge"
    Command::new("flameforge")
        // 为该命令添加描述信息
       .about("Next generation package manager")
        // 为命令添加一个参数，名称为 "version"
       .arg(
            // 创建一个名为 "version" 的参数
            Arg::new("version")
                // 为该参数设置短选项 "-v"
               .short('v')
                // 为该参数设置长选项 "--version"
               .long("version")
                // 当该参数被指定时，将其值设置为 true
               .action(ArgAction::SetTrue),
        )
        // 要求至少指定一个参数，否则显示帮助信息
       .arg_required_else_help(true)
       // 为命令添加一个子命令，该子命令由 info command 函数生成
       .subcommand(info::command())
       // 为命令添加一个子命令，该子命令由 install command 函数生成
       .subcommand(install::command())
       // 为命令添加一个子命令，该子命令由 list command 函数生成
       .subcommand(list::command())
       // 为命令添加一个子命令，该子命令由 remove command 函数生成
       .subcommand(remove::command())
        // 为命令添加一个子命令，该子命令由 version command 函数生成
       .subcommand(version::command())
}

// 定义 process 函数，用于处理命令行参数
pub fn process() {
    // 调用 command 函数获取命令行解析器，并解析命令行参数
    let matches = command().get_matches();
    // 检查是否指定了 "version" 参数
    if matches.get_flag("version") {
        // 如果指定了 "version" 参数，调用 print_version 函数打印版本信息
        version::print();
        // 打印版本信息后返回，结束函数执行
        return;
    }
    // 再次调用 command 函数获取命令行解析器，并解析命令行参数，检查是否有子命令
    match command().get_matches().subcommand() {
        // 如果子命令是 "version"
        Some(("version", _)) => version::print(),
        // 如果子命令是 "list"
        Some(("list", _)) => list::handler(),
        // 如果不是子命令，则程序进入无法到达的状态
        _ => unreachable!(),
    }
}
