//! # 练习 07.07 —— 2D 文字（Text2d 与 TextFont）
//!
//! 出处：https://bevy.org/examples-webgpu/2d/text2d/
//!
//! ## 概念
//! `Text2d` 可以把文字直接放进 2D 世界（像其他精灵一样有坐标、会旋转缩放），
//! 不同于贴在屏幕角落的 UI 文字。`Text2d::new("内容")` 创建文字，
//! `TextFont` 组件负责字号等排版设置。注意在 Bevy 0.19 里，
//! 字号要用 `FontSize::Px(50.0)` 包装一下，不能直接填一个裸的 f32。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 0707` 观察现象，改正后运行 `bevylings test 0707` 让测试通过。
//!
//! 小贴士：`FontSize` 是 Bevy 专门表示"字号"的类型，`FontSize::Px(50.0)`
//! 表示 50 像素高。

// I AM NOT DONE

use bevy::prelude::*;

/// 把"像素字号"包装成 Bevy 的 `FontSize` 类型。
fn make_font_size(pixels: f32) -> FontSize {
    // BUG: `font_size` 字段在 Bevy 0.19 里不是裸的 f32，
    // 需要用 `FontSize::Px(..)` 把它包装起来。
    pixels
}

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    // 一行 50 像素大的中文文字
    commands.spawn((
        Text2d::new("你好，Bevy！"),
        TextFont {
            font_size: make_font_size(50.0),
            ..default()
        },
        Transform::from_xyz(0.0, 100.0, 0.0),
    ));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn font_size_wraps_pixels() {
        assert!(matches!(make_font_size(50.0), FontSize::Px(px) if (px - 50.0).abs() < 1e-5));
    }

    #[test]
    fn font_size_keeps_different_values() {
        assert!(matches!(make_font_size(24.0), FontSize::Px(px) if (px - 24.0).abs() < 1e-5));
    }

    #[test]
    fn text2d_carries_its_content() {
        let text = Text2d::new("hello");
        assert_eq!(text.0.as_str(), "hello");
    }
}

// 提示：
// 1. 先运行 `bevylings run 0707`，看看编译器说"预期 FontSize，找到 f32"。
// 2. 官方示例里怎么写字号？搜一下 `FontSize::Px`。
// 3. 改好后运行 `bevylings test 0707`，三个测试全绿就过关了。
