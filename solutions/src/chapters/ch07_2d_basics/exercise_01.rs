//! # 练习 07.01 —— 生成精灵（Sprite）
//!
//! 出处：https://bevy.org/examples-webgpu/2d/sprite/
//!
//! ## 概念
//! 游戏里的"图片 / 色块"在 Bevy 中叫**精灵（Sprite）**，它是一个组件。
//! 只要把 `Sprite` 组件挂到一个实体上，它就会被画出来。
//! 图片精灵需要从文件加载，但纯色精灵不需要任何文件：
//! `Sprite::from_color(颜色, 尺寸)` 直接就能生成一个色块。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 0701` 观察现象，改正后运行 `bevylings test 0701` 让测试通过。
//!
//! 小贴士：`Color::srgb(r, g, b)` 的三个参数都是 0.0~1.0 的小数，
//! 比如纯红色是 `Color::srgb(1.0, 0.0, 0.0)`。

use bevy::prelude::*;

/// 用纯色生成一个精灵。`size` 是精灵在屏幕上的大小（像素）。
fn make_colored_sprite(color: Color, size: Vec2) -> Sprite {
    Sprite::from_color(color, size)
}

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    // 一个 100x100 的红色方块精灵
    commands.spawn((
        make_colored_sprite(Color::srgb(1.0, 0.0, 0.0), Vec2::new(100.0, 100.0)),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sprite_keeps_requested_color() {
        let color = Color::srgb(0.2, 0.5, 0.9);
        let sprite = make_colored_sprite(color, Vec2::new(64.0, 32.0));
        assert_eq!(sprite.color, color);
    }

    #[test]
    fn sprite_uses_custom_size() {
        let sprite = make_colored_sprite(Color::srgb(1.0, 0.0, 0.0), Vec2::new(64.0, 32.0));
        assert_eq!(sprite.custom_size, Some(Vec2::new(64.0, 32.0)));
    }
}

// 提示：
// 1. 先运行 `bevylings run 0701`，看看编译器报了什么错。
// 2. `Sprite::from_color(颜色, 尺寸)` 需要两个参数，第二个是 `Vec2::new(宽, 高)`。
// 3. 改好后运行 `bevylings test 0701`，两个测试全绿就过关了。
