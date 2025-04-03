// src/lib.rs

// 使用 pub mod 声明一个公共的模块
// 这里声明了一个名为 cli 的模块，意味着项目里存在一个 cli 模块
// 该模块可能包含多个相关的功能，例如命令行解析、命令行子命令的定义等
// 由于使用了 pub 关键字，这个模块可以被项目的其他部分或者外部依赖此库的项目访问

// 调用路径 src/cli
pub mod cli;
// 调用路径 src/client
pub mod client;