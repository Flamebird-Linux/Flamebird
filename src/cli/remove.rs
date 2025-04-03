// src/cli/remove.rs

// 引入 clap 库中的 Command 结构体，用于构建命令行接口
use clap::Command;

// 定义一个公共函数 remove_command Command 实例
// 该函数的作用是创建一个名为 "remove" 的命令行子命令
pub fn remove_command() -> Command {
    // 使用 clap 的 Command::new 方法创建一个新的命令，命令名称为 "remove"
    Command::new("remove")
        // 设置该命令的描述信息，说明该命令的作用是删除一个软件包
        // 这里的 about 方法可以提供简短的描述信息
        // 例如删除软件包的名称、版本等
        // 该描述信息可以帮助用户快速了解命令的功能
        // 该描述信息通常会在帮助信息中显示
        .about("Remove a package")
        // 设置该命令的长描述信息，提供更多的上下文信息
        // 说明该命令的作用是从本地系统删除一个软件包
        // 这里的 long_about 方法可以提供更详细的描述信息
        // 例如删除软件包的来源、删除方式等
        // 该描述信息可以帮助用户更好地理解命令的功能和用法
        // 该描述信息通常会在帮助信息中显示
        .long_about("Remove a package from the local system")
}