// flamebird/src/headers/v1.rs

// 定义一个常量 INTEGRITY_CHECK，它是一个包含 21 个 u8 类型元素的数组
// 这个数组可能用于数据的完整性检查，比如在文件读取或传输过程中，
// 通过对比这个数组来验证数据是否完整或者是否被篡改
const INTEGRITY_CHECK: [u8; 21] = [
    0, 0, 1, 0, 0, 2, 0, 0, 3, 0, 0, 4, 0, 0, 5, 0, 0, 6, 0, 0, 7,
];

///
/// 定义一个公共的枚举类型 FileType，用于表示 v1 版本的 flamebird 容器中已知的文件类型
/// 使用 #[repr(u8)] 注解，表明该枚举类型在内存中以 u8 类型表示，即每个枚举成员对应一个 u8 值
#[repr(u8)]
pub enum FileType {
    /// 表示未知的容器类型，通常作为默认或者无法识别时的情况
    /// 可用于在解析文件时，如果无法确定具体类型，就将其标记为 Unknown
    /// Sanity: Unknown container type
    Unknown,

    /// 表示二进制包文件类型，可能包含预编译好的二进制代码
    /// 这种类型的文件可以直接在目标系统上运行
    /// Binary package
    Binary,

    /// 表示增量包文件类型，通常用于更新操作
    /// 只包含与旧版本之间的差异部分，可减少更新时的数据传输量
    /// Delta package
    Delta,

    /// 表示（旧版）仓库索引文件类型
    /// 用于记录仓库中可用的包信息，帮助管理和查找包
    /// (Legacy) repository index
    Repository,

    /// 表示（旧版）构建清单文件类型
    /// 包含了构建某个软件包所需的信息，如依赖项、构建步骤等
    /// (Legacy) build manifest
    BuildManifest,
}
