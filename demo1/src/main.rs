// 为编写应用代码的地方。
// Rust 由工具 rustup 安装和管理。
// Cargo 有一个很棒的功能是：运行 cargo doc --open 命令来构建所有本地依赖提供的文档，并在浏览器中打开。
/* 
构建工具和包管理器 即 Cargo
cargo build 可以构建项目,(安装依赖)
  运行此命令会创建一个新文件 Cargo.lock，该文件记录了本地所用依赖库的精确版本。
cargo run 可以运行项目
cargo test 可以测试项目
cargo doc 可以为项目构建文档
cargo publish 可以将库发布到 crates.io。
cargo check 快速检查代码确保其可以编译，但并不产生可执行文件
cargo build --release  准备发布时,优化编译项目
cargo update 更新依赖，当前大版本的最后一个版本
 
要检查您是否安装了 Rust 和 Cargo，可以在终端中运行：
cargo --version 
rustc 是 Rust 编程语言的编译器
*/
// cargo new 会生成一个新的“Hello, world!”项目！我们可以进入新创建的目录中，执行下面的命令来运行此程序：cargo run
use ferris_says::say;
use std::io::{stdout, BufWriter};

fn main() {
    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}
