//! # 练习 06.03 —— 球撞墙反弹
//!
//! 出处：https://bevy.org/examples/games/breakout/
//!
//! ## 概念
//! 场地四周有墙。球的边缘一旦越过墙线，就该**反弹**：
//! 撞左右墙 → 翻转 x 方向速度；撞上下墙 → 翻转 y 方向速度。
//! "翻转"就是取反：`v.x = -v.x`。球心到墙的距离是否小于球的半径，
//! 用这个来判断"是否碰到墙"。这比官方示例里完整的圆形-矩形碰撞简单，
//! 但思路一致：判断碰没碰，碰了就反转对应轴的速度。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 0603` 观察现象，改正后运行 `bevylings test 0603` 让测试通过。
//!
//! 小贴士：左右墙影响的是 **x** 速度，上下墙影响的是 **y** 速度。

use bevy::prelude::*;

/// 四面墙的位置（场地边界）。
const LEFT_WALL: f32 = -450.0;
const RIGHT_WALL: f32 = 450.0;
const TOP_WALL: f32 = 300.0;
const BOTTOM_WALL: f32 = -300.0;
/// 球的直径（用来算半径）。
const BALL_DIAMETER: f32 = 30.0;

/// 球：标记组件。
#[derive(Component)]
struct Ball;

/// 速度：每秒在 x、y 方向各移动多少像素。
#[derive(Component)]
struct Velocity(Vec2);

/// 检查四面墙，返回反弹后的速度。
fn bounce(position: Vec2, velocity: Vec2) -> Vec2 {
    let mut v = velocity;
    let radius = BALL_DIAMETER / 2.0;

    // 撞左右墙
    if position.x - radius < LEFT_WALL || position.x + radius > RIGHT_WALL {
        v.x = -v.x;
    }

    // 撞上下墙
    if position.y + radius > TOP_WALL || position.y - radius < BOTTOM_WALL {
        v.y = -v.y;
    }

    v
}

/// 每帧检查球是否撞墙。
fn bounce_off_walls(mut query: Query<(&mut Transform, &mut Velocity), With<Ball>>) {
    for (mut transform, mut velocity) in &mut query {
        velocity.0 = bounce(transform.translation.truncate(), velocity.0);
    }
}

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, bounce_off_walls)
        .run();
}

/// 生成相机和球。
fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn((
        Ball,
        Velocity(Vec2::new(150.0, 200.0)),
        Transform::from_xyz(0.0, 0.0, 1.0),
    ));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bounces_off_left_wall() {
        let v = bounce(Vec2::new(LEFT_WALL + 5.0, 0.0), Vec2::new(-100.0, 50.0));
        assert_eq!(v.x, 100.0, "撞左墙后 x 速度应反向");
        assert_eq!(v.y, 50.0, "y 速度不应受影响");
    }

    #[test]
    fn bounces_off_top_wall() {
        let v = bounce(Vec2::new(0.0, TOP_WALL - 5.0), Vec2::new(100.0, 80.0));
        assert_eq!(v.y, -80.0, "撞顶墙后 y 速度应反向");
        assert_eq!(v.x, 100.0, "x 速度不应受影响");
    }

    #[test]
    fn flies_freely_inside_arena() {
        let v = bounce(Vec2::ZERO, Vec2::new(30.0, -20.0));
        assert_eq!(v, Vec2::new(30.0, -20.0), "在场地内速度保持不变");
    }
}

// 提示：
// 1. 想一想：撞"左右"墙，需要反转哪个方向的速度？
// 2. 把 BUG 行里的 y 改成正确的轴，跑 `bevylings test 0603`。
// 3. 检查每个测试的断言消息，它们会告诉你要修哪条轴。
