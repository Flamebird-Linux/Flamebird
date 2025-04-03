// src/cli/list.rs

// 导入 clap 库中的 Command 结构体，用于构建命令行的子命令
use clap::Command;

// 定义一个公共函数 command Command 实例
// 该函数的作用是创建一个名为 "list" 的命令行子命令
pub fn command() -> Command {
    // 创建一个新的命令，名称为 "list"
    Command::new("list")
        // 设置该命令的简要描述信息
       .about("List Package")
        // 设置该命令的长描述信息，提供更多的上下文信息
       .long_about("List packages according to a filter")
        // 要求该命令必须指定一个子命令，否则会报错
       .subcommand_required(true)

	    // 为 "list" 命令添加一个子命令 "installed"
	   .subcommand(

	       Command::new("installed")
	       		// 为该子命令添加简要描述信息
	       		.about("List all installed packages")
	       		// 为该子命令添加别名
	       		.visible_alias("li"),
	   )
        // 为 "list" 命令添加另一个子命令 "available"
		.subcommand(
	       Command::new("available")
	       		// 为该子命令添加简要描述信息
	       		.about("List all available packages")
	       		// 为该子命令添加别名
				.visible_alias("la"),
		)
}

// 定义一个公共函数 list_command_handler，用于处理 "list" 命令
pub fn handler() {
    // 打印 "Listage" 字符串，这里应该是一个占位实现，实际使用时可替换为具体的列表处理逻辑
    println!("Listage");
}