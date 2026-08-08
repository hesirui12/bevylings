//! # 练习 07.05 —— 缩放与翻转（transform.scale / flip）
//!
//! 出处：https://bevy.org/examples-webgpu/2d/sprite_flipping/ 与
//! https://bevy.org/examples-webgpu/2d/sprite_scale/
//!
//! ## 概念
//! 想让精灵"变大变小"，不用重新画图，改 `Transform.scale` 就行：
//! `Vec3::splat(2.0)` 表示放大 2 倍，`Vec3::splat(0.5)` 表示缩小一半。
//! 想让精灵"左右颠倒 / 上下颠倒"，则用 `Sprite` 自带的
//! `flip_x`（水平翻转）和 `flip_y`（垂直翻转）两个字段。
//! 翻转不改坐标、不改大小，只把贴图镜像一下。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 0705` 观察现象，改正后运行 `bevylings test 0705` 让测试通过。
//!
//! 小贴士：`Transform::from_xyz(..).with_scale(Vec3::splat(2.0))`
//! 可以在生成实体时直接带上缩放。

use bevy::prelude::*;

/// 生成一个绿色方块精灵。
fn make_sprite(size: Vec2, flip_x: bool, flip_y: bool) -> Sprite {
    Sprite {
        color: Color::srgb(0.2, 0.8, 0.4),
        custom_size: Some(size),
        flip_x,
        flip_y,
        ..default()
    }
}

/// 敌人需要水平翻转（设计如此），返回 true 表示"要翻转"。
fn enemy_flipped() -> bool {
    true
}

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    // 主角：不翻转，放在左边
    commands.spawn((
        make_sprite(Vec2::new(100.0, 100.0), false, false),
        Transform::from_xyz(-150.0, 0.0, 0.0),
    ));

    // 敌人：水平翻转 + 放大 2 倍，放在右边
    commands.spawn((
        make_sprite(Vec2::new(100.0, 100.0), enemy_flipped(), false),
        Transform::from_xyz(150.0, 0.0, 0.0).with_scale(Vec3::splat(2.0)),
    ));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enemy_should_be_flipped() {
        assert!(enemy_flipped(), "敌人在设计上要水平翻转，否则和主角朝向一样");
    }

    #[test]
    fn sprite_keeps_flip_flags() {
        let sprite = make_sprite(Vec2::new(100.0, 100.0), true, false);
        assert!(sprite.flip_x, "flip_x 为 true 表示水平翻转");
        assert!(!sprite.flip_y, "这里没有垂直翻转");
    }

    #[test]
    fn sprite_size_is_kept() {
        let sprite = make_sprite(Vec2::new(80.0, 40.0), false, true);
        assert_eq!(sprite.custom_size, Some(Vec2::new(80.0, 40.0)));
    }
}

// 提示：
// 1. 敌人是不是应该和主角朝向相反？现在 `enemy_flipped()` 返回了什么？
// 2. 想一想 flip_x 和 flip_y 分别翻转哪个方向：左右，还是上下？
// 3. 改好后运行 `bevylings test 0705`，三个测试全绿就过关了。
