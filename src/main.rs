// 导入 flameforge 包中 cli 模块的 process 函数
// 这个 process 函数用于处理命令行参数和执行相应的操作
use flameforge::cli::process;

// 定义程序的入口函数，当程序启动时会首先执行这个函数
fn main() {
    // 调用之前导入的 process 函数
    // 该函数负责解析命令行参数，并根据参数执行相应的逻辑，例如打印版本信息等
    process();
}