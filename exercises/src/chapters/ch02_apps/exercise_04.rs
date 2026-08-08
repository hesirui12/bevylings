//! # 练习 02.04 —— 默认插件配置（DefaultPlugins.set）
//!
//! 出处：https://bevy.org/examples-webgpu/application/settings/
//!
//! ## 概念
//! `DefaultPlugins` 是一个插件组，里面包含窗口、渲染、输入等一大堆插件。
//! 用 `.set(...)` 可以**替换**组里的某个插件，改成你自己的配置。
//! 比如 `WindowPlugin` 决定窗口长什么样：标题、大小、能否缩放等。
//! 配置结构体字段很多，习惯上用 `..default()` 补齐没写到的字段。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 0204` 观察现象，改正后运行 `bevylings test 0204` 让测试通过。
//!
//! 小贴士：`Window` 的字段非常多，只写想改的字段，剩下的交给 `..default()`。

// I AM NOT DONE

use bevy::prelude::*;

/// 根据标题构造一个"主窗口插件"配置。
fn build_window_plugin(title: String) -> WindowPlugin {
    WindowPlugin {
        primary_window: Some(Window {
            title,
            ..default()
        }),
        // BUG: 这个结构体初始化忘了用 `..default()` 补齐其余字段，
        // 编译器会列出缺少的字段并报错。
    }
}

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins.set(build_window_plugin("My Game".to_string())))
        .run();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn window_title_is_configured() {
        let plugin = build_window_plugin("My Game".to_string());
        let window = plugin.primary_window.expect("应该有主窗口");
        assert_eq!(window.title, "My Game", "窗口标题应该来自配置");
    }

    #[test]
    fn window_keeps_default_settings() {
        let plugin = build_window_plugin(String::new());
        let window = plugin.primary_window.unwrap();
        assert!(window.resizable, "默认窗口应该是可调整大小的");
    }
}

// 提示：
// 1. 编译器会说"missing fields ... in initializer"，列出缺的字段。
// 2. 解决办法是加上一行 `..default()`：意思是"其余字段都用默认值"。
// 3. 修改后运行 `bevylings test 0204`，两个测试都通过就过关了。
