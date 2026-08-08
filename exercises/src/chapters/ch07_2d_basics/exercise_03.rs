//! # 练习 07.03 —— 用时间移动精灵（Transform.translation）
//!
//! 出处：https://bevy.org/examples-webgpu/2d/move_sprite/
//!
//! ## 概念
//! 游戏是一帧一帧跑的。想让精灵"每秒移动 150 像素"，不能每帧固定加 150，
//! 而要乘上"这一帧过去了多久"：`位移 = 速度 × 时间`。
//! Bevy 里这个时间由 `Time` 资源提供，`time.delta_secs()` 返回**这一帧的秒数**，
//! 这样不管电脑快慢（帧率高低），精灵的速度都保持一致。
//! 我们还用一个组件 `Direction` 标记精灵当前该往哪边走。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 0703` 观察现象，改正后运行 `bevylings test 0703` 让测试通过。
//!
//! 小贴士：别把 `delta()` 和 `delta_secs()` 搞混：
//! 一个是"一段时间"（Duration），一个是"秒数"（f32）。

// I AM NOT DONE

use bevy::prelude::*;

/// 精灵当前的前进方向。
#[derive(Component, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

/// 每秒移动多少像素（向左是负数，向右是正数）。
fn move_speed(direction: Direction) -> f32 {
    match direction {
        Direction::Right => 150.0,
        Direction::Left => -150.0,
    }
}

/// 每帧根据经过的时间移动精灵。
fn sprite_movement(time: Res<Time>, mut query: Query<(&Direction, &mut Transform)>) {
    for (direction, mut transform) in &mut query {
        // BUG: `delta()` 返回的是 Duration（一段时长），不能直接和速度相乘。
        // 这里应该用"返回秒数"的那个方法，才能得到 f32。
        transform.translation.x += move_speed(*direction) * time.delta();
    }
}

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, sprite_movement)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    // 一个从原点出发、向右移动的精灵
    commands.spawn((
        Sprite::from_color(Color::srgb(0.9, 0.5, 0.2), Vec2::new(60.0, 60.0)),
        Transform::from_xyz(0.0, 0.0, 0.0),
        Direction::Right,
    ));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn right_moves_positive() {
        assert_eq!(move_speed(Direction::Right), 150.0);
    }

    #[test]
    fn left_moves_negative() {
        assert_eq!(move_speed(Direction::Left), -150.0);
    }

    #[test]
    fn directions_are_opposite() {
        assert_ne!(move_speed(Direction::Left), move_speed(Direction::Right));
    }
}

// 提示：
// 1. 先运行 `bevylings run 0703`，看编译器对 `time.delta()` 报什么错。
// 2. 想一想：`Time` 上有哪两个方法返回"时间"？哪个是 f32 秒数？
// 3. 改好后运行 `bevylings test 0703`，三个测试全绿就过关了。
