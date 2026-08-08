//! # 练习 23.03 —— 转向鼠标（rotate to cursor）
//!
//! 出处：https://bevy.org/examples-webgpu/2d/rotate_to_cursor/
//!
//! ## 概念
//! 让精灵"看向鼠标"是射击游戏最常见的需求之一，分三步：
//! 1. 用 `window.cursor_position()` 拿到鼠标的屏幕坐标（左上角为原点）；
//! 2. 用 `camera.viewport_to_world_2d(相机变换, 屏幕坐标)` 把屏幕坐标
//!    换算成世界坐标（世界原点在屏幕中心）；
//! 3. 计算"从自己到目标"的方向角：`(目标 - 自己).to_angle()`，
//!    得到弧度后赋给 `Quat::from_rotation_z(弧度)`。
//! 本练习把第 3 步抽成纯函数 `angle_to`，方便测试。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 2303` 观察现象，改正后运行 `bevylings test 2303` 让测试通过。
//!
//! 小贴士：`Vec2::to_angle()` 返回弧度：朝 +X 方向是 0，朝 +Y 方向是 π/2。

// I AM NOT DONE

use bevy::prelude::*;

/// 计算"从自己到目标"的方向角（弧度）。0 表示朝 +X，正数是逆时针。
fn angle_to(from: Vec2, to: Vec2) -> f32 {
    // BUG: 方向算反了：应该用"目标 - 自己"，现在用的是"自己 - 目标"，
    // 结果精灵会一直背对着鼠标。
    (from - to).to_angle()
}

/// 玩家（由鼠标控制朝向）。
#[derive(Component)]
struct Player;

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, player_movement_system)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn((
        Sprite::from_color(Color::srgb(1.0, 0.4, 0.2), Vec2::new(60.0, 60.0)),
        Player,
    ));
}

/// 让玩家每帧面向鼠标所在的世界坐标。
fn player_movement_system(
    mut player: Single<&mut Transform, With<Player>>,
    camera: Single<(&Camera, &GlobalTransform)>,
    window: Single<&Window>,
) {
    let (camera, camera_transform) = *camera;

    if let Some(cursor) = window.cursor_position() {
        if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor) {
        // 我们的精灵默认朝右，直接使用两点之间的方向角即可
        player.rotation = Quat::from_rotation_z(angle_to(player.translation.xy(), world_pos));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn east_target_has_zero_angle() {
        let angle = angle_to(Vec2::ZERO, Vec2::new(1.0, 0.0));
        assert!((angle - 0.0).abs() < 1e-5);
    }

    #[test]
    fn north_target_is_quarter_turn() {
        let angle = angle_to(Vec2::ZERO, Vec2::new(0.0, 1.0));
        assert!((angle - std::f32::consts::FRAC_PI_2).abs() < 1e-5);
    }

    #[test]
    fn angle_depends_on_relative_position() {
        // 自己在 (5,5)，目标在右边一格：仍然是朝右
        let angle = angle_to(Vec2::new(5.0, 5.0), Vec2::new(6.0, 5.0));
        assert!((angle - 0.0).abs() < 1e-5);
    }
}

// 提示：
// 1. 先运行 `bevylings run 2303`，看看方块是不是背对着鼠标。
// 2. `angle_to` 里的两个向量谁减谁？"从自己到目标"应该是哪个？
// 3. 改好后运行 `bevylings test 2303`，三个测试全绿就过关了。
