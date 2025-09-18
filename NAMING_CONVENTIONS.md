# 命名规范

# 1.项目名称

* 当使用 cargo new 创建项目时，遵循 kebab-case，Cargo 推荐使用连字符（-）分隔单词。例如：
```
cargo new my-rust-project
```
* 为什么用连字符？

    Cargo 和 Rust 生态系统约定使用 kebab-case（连字符分隔的小写命名）作为项目名称的命名规范。

    项目名称会用作 crate 名称，发布到 crates.io 时要求使用连字符。

    如果使用下划线（_），Cargo 会自动将下划线转换为连字符。例如，my_project 在发布时会被视为 my-project。

# 2.文件命名
* Rust 源文件（.rs）通常也遵循 kebab-case，即使用连字符。例如：
```
my-module.rs
```
* 原因：

    Rust 文件名通常与模块名对应，而模块名在代码中需要使用连字符（-）引用。例如，文件 my-module.rs 对应模块 my_module，但文件名本身使用连字符以保持一致性。
    Cargo 和 Rust 工具（如 cargo fmt 和 clippy）都能很好地处理连字符命名。

# 3.代码中的标识符
* 在 Rust 代码内部，标识符（变量名、函数名等）使用 snake_case（下划线分隔的小写命名）。例如
```
fn my_function_name() {
    let my_variable = 42;
}
```

* **注意**：代码中的模块名在引用时会将文件名中的连字符（-）转换为下划线（_）。例如，文件 my-module.rs 在代码中通过 mod my_module; 引入。