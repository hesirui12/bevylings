//! # 练习 22.01 —— 窗口设置：标题、分辨率、大小
//!
//! 出处：https://bevy.org/examples-webgpu/window/window-settings/
//!
//! ## 概念
//! Bevy 的窗口长什么样，由 `WindowPlugin` 里的 `primary_window` 决定：
//! 它是 `Some(Window { ... })`，里面可以设置：
//! - `title`：窗口标题栏上的文字（一个 String）。
//! - `resolution`：窗口大小，类型是 `WindowResolution`。常用的简便写法是
//!   `(宽, 高).into()`，比如 `(500, 300).into()` 表示 500x300。
//! - `resizable`：用户能不能用鼠标拖拽窗口边缘改变大小。
//!
//! 我们写一个函数 `make_window`，返回一个设置好的 `Window`。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 2201` 查看现象（目前编译会报错），
//! 改正后运行 `bevylings test 2201` 让测试通过。
//!
//! 小贴士：`resolution` 字段的类型不是元组 `(500, 300)`，需要用 `.into()` 转换。

use bevy::prelude::*;

/// 创建一个配置好的主窗口。
fn make_window() -> Window {
    Window {
        title: "I am a window!".to_string(),
        resolution: (500, 300).into(),
        resizable: true,
        ..default()
    }
}

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(make_window()),
            ..default()
        }))
        .run();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn window_has_expected_title_and_size() {
        let window = make_window();
        assert_eq!(window.title, "I am a window!");
        assert_eq!(window.resolution.width(), 500.0, "窗口宽度应该是 500");
        assert_eq!(window.resolution.height(), 300.0, "窗口高度应该是 300");
    }

    #[test]
    fn window_is_resizable() {
        let window = make_window();
        assert!(window.resizable, "窗口应该允许用户拖拽改大小");
    }
}

// 提示：
// 1. 先看 `// BUG:` 那一行：`resolution` 字段的类型是什么？
// 2. `(500, 300).into()` 能把元组转换成 WindowResolution（官方示例就是这么写的）。
// 3. 修改后运行 `bevylings test 2201`，两个测试都通过就过关了。
