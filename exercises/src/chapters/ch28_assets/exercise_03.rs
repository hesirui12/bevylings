//! # 练习 28.03 —— 嵌入资产：embedded:// 路径
//!
//! 出处：https://bevy.org/examples-webgpu/asset/embedded-asset/
//!
//! ## 概念
//! 普通资产在程序启动后从磁盘加载；"嵌入资产"则直接编译进可执行文件
//! （官方用 `embedded_asset!` 宏，底层是 `include_bytes!`），适合做加载画面等
//! 必须立刻可用的资源。嵌入资产存放在一个自定义的资产源（Asset Source）里，
//! 路径写法是 `embedded://<crate名>/<文件路径>`。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 2803` 查看现象，改正后运行 `bevylings test 2803` 让测试通过。
//!
//! 小贴士：`AssetSourceId` 表示"资产从哪个源来"，`embedded` 就是那个自定义源的名字。

// I AM NOT DONE

use bevy::asset::{io::AssetSourceId, AssetPath};
use bevy::prelude::*;
use std::path::PathBuf;

/// 构造一个位于 "embedded://" 自定义源里的资产路径。
/// 真实项目中这样使用：`asset_server.load(path)`。
fn embedded_path(crate_name: &str, file: &str) -> AssetPath<'static> {
    // 注意用 / 连接，避免 Windows 下 join 产生反斜杠
    let path = PathBuf::from(format!("{crate_name}/{file}"));
    // BUG: 源的名称写错了，生成的是 files:// 而不是 embedded://。
    let source = AssetSourceId::from("files");
    AssetPath::from_path_buf(path).with_source(source)
}

pub fn run() {
    // 演示：把构造好的路径交给 asset_server.load() 就能加载嵌入资产。
    let path = embedded_path("my_game", "files/logo.png");
    info!("嵌入资产路径: {path}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_embedded_url() {
        let path = embedded_path("my_game", "files/logo.png");
        assert_eq!(path.to_string(), "embedded://my_game/files/logo.png");
    }

    #[test]
    fn default_source_has_no_prefix() {
        let path = AssetPath::from_path_buf(PathBuf::from("sounds/ding.ogg"));
        assert_eq!(path.to_string(), "sounds/ding.ogg");
    }
}

// 提示：
// 1. 运行 `bevylings run 2803`，日志里打印的前缀是 files:// 而不是 embedded://。
// 2. 官方示例里写的是 `AssetSourceId::from("embedded")`。
// 3. 修改后运行 `bevylings test 2803`，两个测试全绿就过关了。
