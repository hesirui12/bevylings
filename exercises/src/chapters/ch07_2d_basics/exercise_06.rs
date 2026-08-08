//! # 练习 07.06 —— 透明度（Sprite alpha）
//!
//! 出处：https://bevy.org/examples-webgpu/2d/transparency_2d/
//!
//! ## 概念
//! 颜色有四个分量：红、绿、蓝、**透明度 alpha**（0.0 = 全透明，1.0 = 不透明）。
//! `Color::srgba(r, g, b, a)` 可以同时给出颜色和透明度。
//! 几个半透明精灵叠在一起时，Bevy 会按 Z 坐标排序，先画远处的再画近处的，
//! 这样后面的精灵就能透过前面的精灵显示出来。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 0706` 观察现象，改正后运行 `bevylings test 0706` 让测试通过。
//!
//! 小贴士：`Color::srgb(r, g, b)` 也有一个隐藏的 alpha，它恒等于 1.0（不透明）。

// I AM NOT DONE

use bevy::prelude::*;

/// 把 RGB 颜色和透明度合成一个"带 alpha 的颜色"。
fn translucent_color(r: f32, g: f32, b: f32, alpha: f32) -> Color {
    // BUG: srgb 只有红绿蓝三个通道，我们辛苦传进来的 alpha 被丢掉了，
    // 结果所有精灵都变成完全不透明，看不出叠加效果。
    Color::srgb(r, g, b)
}

/// 生成一个带透明度的彩色方块精灵。
fn make_translucent_sprite(r: f32, g: f32, b: f32, alpha: f32) -> Sprite {
    Sprite {
        color: translucent_color(r, g, b, alpha),
        custom_size: Some(Vec2::new(120.0, 120.0)),
        ..default()
    }
}

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    // 三个方块从左到右，透明度依次不同（z 坐标高的画在上面）
    commands.spawn((
        make_translucent_sprite(0.0, 0.0, 1.0, 0.7),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
    commands.spawn((
        make_translucent_sprite(0.0, 1.0, 0.0, 0.3),
        Transform::from_xyz(100.0, 0.0, 0.1),
    ));
    commands.spawn((
        make_translucent_sprite(1.0, 0.0, 0.0, 0.9),
        Transform::from_xyz(200.0, 0.0, 0.2),
    ));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alpha_is_preserved() {
        let color = translucent_color(0.0, 0.0, 1.0, 0.7);
        assert!((color.to_srgba().alpha - 0.7).abs() < 1e-5);
    }

    #[test]
    fn rgb_channels_are_kept() {
        let color = translucent_color(1.0, 0.0, 0.0, 0.3);
        let srgba = color.to_srgba();
        assert!((srgba.red - 1.0).abs() < 1e-5);
        assert!((srgba.green - 0.0).abs() < 1e-5);
    }

    #[test]
    fn opaque_color_has_alpha_one() {
        let color = translucent_color(0.0, 1.0, 0.0, 1.0);
        assert!((color.to_srgba().alpha - 1.0).abs() < 1e-5);
    }
}

// 提示：
// 1. 先运行 `bevylings run 0706`，看看三个方块是不是都"不透亮"了。
// 2. 想保留 alpha，该用 `Color` 的哪个构造方法？（sRGB 家族里还有一位成员）
// 3. 改好后运行 `bevylings test 0706`，三个测试全绿就过关了。
