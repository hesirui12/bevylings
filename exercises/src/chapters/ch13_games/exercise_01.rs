//! # 练习 13.01 —— 弹跳球：重力与反弹
//!
//! 出处：https://bevy.org/examples/games/contributors/
//!
//! ## 概念
//! 官方示例 contributors 里，一堆小精灵受重力下落、撞地反弹。
//! 我们把它简化成一颗二维弹跳球，物理只有三步，每帧执行一次：
//! ① 重力让 y 速度减小（`v.y -= GRAVITY × 帧时间`）；
//! ② 位置加上速度（`p += v × 帧时间`）；
//! ③ 超出地面/天花板就停在边界上并反转速度。
//! 位置和速度都是 `Vec2` —— 二维向量只有 x、y 两个分量。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 1301` 查看现象（目前会编译报错），
//! 改正后运行 `bevylings test 1301` 让测试通过。
//!
//! 小贴士：`Vec2` 只有 `x` 和 `y` 两个字段；`Vec3` 才有 `z`。

// I AM NOT DONE

use bevy::prelude::*;

/// 重力加速度：每秒让 y 速度减小这么多像素/秒。
const GRAVITY: f32 = 980.0;
/// 地面与天花板的 y 坐标。
const GROUND_Y: f32 = -300.0;
const CEILING_Y: f32 = 300.0;
/// 落地后速度保留的比例（模拟能量损失）。
const BOUNCE_REDUCTION: f32 = 0.8;

/// 球：标记组件。
#[derive(Component)]
struct Ball;

/// 速度：每秒在 x、y 方向移动的像素数。
#[derive(Component)]
struct Velocity(Vec2);

/// 一帧物理：先重力加速，再移动，最后处理边界反弹。
/// 返回 (新位置, 新速度)。
fn physics_step(pos: Vec2, vel: Vec2, delta: f32) -> (Vec2, Vec2) {
    let mut v = vel;
    v.y -= GRAVITY * delta;

    let mut p = pos + v * delta;

    // 落到地面：停在边界并向上反弹（损失一点速度）
    if p.y < GROUND_Y {
        p.y = GROUND_Y;
        v.y = -v.y * BOUNCE_REDUCTION;
    }

    // 撞到天花板：向下弹
    if p.y > CEILING_Y {
        // BUG: 位置是二维向量 `Vec2`，只有 x、y 两个分量，
        // 这里却写了一个不存在的字段，编译会报 "no field" 错误。
        p.z = CEILING_Y;
        v.y = -v.y;
    }

    (p, v)
}

/// 每帧对球做一次物理模拟。
fn apply_physics(time: Res<Time>, mut query: Query<(&mut Transform, &mut Velocity)>) {
    for (mut transform, mut velocity) in &mut query {
        let (pos, vel) = physics_step(
            transform.translation.truncate(),
            velocity.0,
            time.delta_secs(),
        );
        transform.translation.x = pos.x;
        transform.translation.y = pos.y;
        velocity.0 = vel;
    }
}

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, apply_physics)
        .run();
}

/// 生成相机和球。
fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn((
        Ball,
        Velocity(Vec2::new(100.0, 200.0)),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gravity_pulls_ball_downward() {
        let (_, vel) = physics_step(Vec2::ZERO, Vec2::ZERO, 0.1);
        assert!(vel.y < 0.0, "重力应让 y 速度变成负数（向下）");
    }

    #[test]
    fn ball_rests_on_ground_and_bounces() {
        let (pos, vel) = physics_step(
            Vec2::new(50.0, GROUND_Y - 5.0),
            Vec2::new(30.0, -100.0),
            0.1,
        );
        assert_eq!(pos.y, GROUND_Y, "落地后位置应停在底面");
        assert_eq!(vel.x, 30.0, "水平速度不应受地面影响");
        assert!(vel.y > 0.0, "落地后应向上反弹");
    }

    #[test]
    fn ceiling_bounces_ball_down() {
        let (pos, vel) = physics_step(Vec2::new(0.0, CEILING_Y + 5.0), Vec2::new(0.0, 500.0), 0.1);
        assert_eq!(pos.y, CEILING_Y, "撞天花板后位置应停在边界");
        assert!(vel.y < 0.0, "撞天花板后应向下弹");
    }
}

// 提示：
// 1. 运行 `bevylings run 1301` 读编译错误：`no field 'z' on type 'Vec2'`。
// 2. 2D 里"上下"是 y 轴，把 z 改成正确字段即可。
// 3. 改完跑 `bevylings test 1301`，三个测试全绿就过关。
