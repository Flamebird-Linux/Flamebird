// src/cli/list.rs

// 导入 clap 库中的 Command 结构体，用于构建命令行的子命令
use clap::Command;

// 定义一个公共函数 command，返回一个 Command 实例
// 该函数的作用是创建一个名为 "list" 的命令行子命令
pub fn command() -> Command {
	// 创建一个新的命令，名称为 "list"
	Command::new("list")
	// 设置该命令的简要描述信息，用户在使用帮助时会看到
	.about("List Package")
	// 设置该命令的长描述信息，提供更多的上下文信息
	.long_about("List packages according to a filter")
	// 要求该命令必须指定一个子命令，否则会报错
	.subcommand_required(true)

	// 为 "list" 命令添加一个子命令 "installed"
	.subcommand(
		// 创建一个新的子命令，名称为 "installed"
		Command::new("installed")
		// 为该子命令添加简要描述信息，说明其功能
		.about("List all installed packages")
		// 为该子命令添加别名 "li"，用户可以通过 "li" 调用该子命令
		.visible_alias("li"),
	)
	// 为 "list" 命令添加另一个子命令 "available"
	.subcommand(
		// 创建一个新的子命令，名称为 "available"
		Command::new("available")
		// 为该子命令添加简要描述信息，说明其功能
		.about("List all available packages")
		// 为该子命令添加别名 "la"，用户可以通过 "la" 调用该子命令
		.visible_alias("la"),
	)
}

// 定义一个公共函数 handle，用于处理 "list" 命令的参数
// 参数 args 是一个 ArgMatches 类型的引用，表示解析后的命令行参数
pub fn handle(args: &ArgMatches) {
	// 使用 match 表达式匹配子命令
	match args.subcommand() {
		// 如果子命令是 "available"，执行未实现的逻辑
		Some(("available", _)) => unimplemented!(),
		// 如果子命令是 "installed"，执行未实现的逻辑
		Some(("installed", _)) => unimplemented!(),
		// 如果没有匹配到任何子命令，理论上不会发生（因为 subcommand_required 为 true）
		_ => unreachable!(),
	}
}
