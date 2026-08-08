//! # 练习 10.01 —— 播放音频：AudioPlayer 与资产加载（AssetServer）
//!
//! 出处：https://bevy.org/examples-webgpu/audio/audio/
//!
//! ## 概念
//! 播放声音需要两样东西：音频文件（资产）和“播放器”。
//! 资产由 `AssetServer` 统一管理，`asset_server.load("路径")` 会返回一个
//! `Handle<AudioSource>`（音频句柄，可以理解为“文件的凭证”）。
//! 只要把 `AudioPlayer::new(句柄)` 生成到世界里，Bevy 就会开始播放这段音频。
//! 我们把“拼音频文件路径”抽成了纯函数 `sound_path`，方便在测试里验证。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 1001` 观察现象，改正后运行 `bevylings test 1001` 让测试通过。
//!
//! 小贴士：`format!` 宏可以把变量拼进字符串；`concat!` 只能拼编译期就知道的字面量。

// I AM NOT DONE

use bevy::prelude::*;

/// 把文件名拼成 sounds 目录下的完整路径，例如 `sound_path("BGM")`
/// 会得到 `"sounds/BGM.ogg"`。
fn sound_path(name: &str) -> String {
    // BUG: 这里想“把文件名拼进路径”，但 concat! 只能拼接编译期的字面量，
    // 而 name 是运行时的变量，这样写编译不过。想一想该换哪个宏。
    concat!("sounds/", name, ".ogg")
}

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

/// 启动时加载并播放一首音乐。
fn setup(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.spawn(AudioPlayer::new(asset_server.load(sound_path("Windless Slopes"))));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn path_points_into_sounds_folder() {
        let path = sound_path("BGM");
        assert!(path.starts_with("sounds/"), "路径应该以 sounds/ 开头，实际是 {path}");
    }

    #[test]
    fn path_ends_with_ogg_extension() {
        let path = sound_path("BGM");
        assert!(path.ends_with(".ogg"), "路径应该以 .ogg 结尾，实际是 {path}");
        assert_eq!(path, "sounds/BGM.ogg", "拼接结果应该符合预期");
    }
}

// 提示：
// 1. 运行 `bevylings run 1001`，看看编译器报的错在哪个宏上。
// 2. `concat!` 只能处理字符串字面量；把变量拼进字符串要用 `format!`。
// 3. 改好后运行 `bevylings test 1001`，测试全绿就过关了。
