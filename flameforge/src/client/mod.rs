// src/client/mod.rs

// 导入标准库中的 PathBuf 类型，用于表示文件系统路径
use std::path::PathBuf;

// 定义一个公共的结构体 client，用于表示客户端实例
// 目前该结构体包含一个字段 root，类型为 PathBuf，表示根路径
pub struct client {
    // 定义一个字段 root，类型为 PathBuf，用于存储客户端的根路径
    root: PathBuf,
}

// 为 client 结构体实现一个关联函数
impl client {
    /// 构建一个新的客户端
    // 定义一个关联函数 new_for_root，用于根据指定的根路径创建一个新的 client 实例
    fn new_for_root(root: PathBuf) -> client {
        // 创建并返回一个 client 实例，初始化其 root 字段为传入的路径
        client { root }
    }

    // 定义一个关联函数 system，用于创建一个默认的客户端实例
    // 默认情况下，根路径设置为 "/"（即系统的根目录）
    fn system() -> client {
        // 调用 new_for_root 函数，传入根路径 "/"，创建并返回一个 client 实例
        client::new_for_root(PathBuf::from("/"))
    }
}

// 为 client 结构体实现 Drop trait
// Drop trait 用于在对象被销毁时执行一些清理操作，比如释放资源等
impl Drop for client {
    // 实现 Drop trait 中的 drop 方法
    // 当 client 实例离开其作用域时，这个方法会被自动调用
    fn drop(&mut self) {
        // todo! 宏用于标记一个待完成的代码区域
        // 这里表示在 client 实例被销毁时，需要实现具体的清理逻辑
        todo!()
    }
}
