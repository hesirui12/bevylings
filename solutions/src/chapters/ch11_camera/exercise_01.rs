//! # 练习 11.01 —— 2D 相机跟随：WASD 移动、相机平滑追赶
//!
//! 出处：https://bevy.org/examples-webgpu/camera/2d_top_down_camera/
//!
//! ## 概念
//! 2D 俯视角游戏里，玩家角色在世界里移动，摄像机一直跟着玩家。
//! 做法：玩家实体带上 `Player` 标记组件，摄像机实体用 `Camera2d`；
//! 每帧先用 WASD 更新玩家位置，再让相机向玩家位置**平滑靠拢**
//! （`smooth_nudge`：按衰减率插值，和帧率无关）。
//! 我们先把“按键 → 方向”和“方向 → 位移”拆成纯函数，方便测试。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 1101` 观察现象，改正后运行 `bevylings test 1101` 让测试通过。
//!
//! 小贴士：字母键和坐标轴一致：A 是左（x 减小），D 是右（x 增大）。

use bevy::prelude::*;

/// 玩家移动速度（像素/秒）。
const PLAYER_SPEED: f32 = 100.0;

/// 相机追赶玩家的衰减率：越大追得越快。
const CAMERA_DECAY_RATE: f32 = 2.0;

#[derive(Component)]
struct Player;

/// 把按键状态翻译成移动方向向量。
fn input_direction(keys: &ButtonInput<KeyCode>) -> Vec2 {
    let mut direction = Vec2::ZERO;
    if keys.pressed(KeyCode::KeyW) {
        direction.y += 1.0;
    }
    if keys.pressed(KeyCode::KeyS) {
        direction.y -= 1.0;
    }
    if keys.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }
    if keys.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }
    direction
}

/// 根据方向计算本帧位移：方向 × 速度 × 帧时长。
fn move_delta(direction: Vec2, speed: f32, delta_secs: f32) -> Vec3 {
    (direction.normalize_or_zero() * speed * delta_secs).extend(0.0)
}

/// 根据 WASD 更新玩家位置。
fn move_player(
    mut player: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    let Ok(mut player) = player.single_mut() else {
        return;
    };
    player.translation += move_delta(input_direction(&keys), PLAYER_SPEED, time.delta_secs());
}

/// 相机向玩家平滑靠拢（x/y 跟随玩家，z 保持相机自己的高度）。
fn follow_player(
    mut camera: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
    player: Query<&Transform, (With<Player>, Without<Camera2d>)>,
    time: Res<Time>,
) {
    let Ok(mut camera) = camera.single_mut() else {
        return;
    };
    let Ok(player) = player.single() else {
        return;
    };
    let target = Vec3::new(player.translation.x, player.translation.y, camera.translation.z);
    camera
        .translation
        .smooth_nudge(&target, CAMERA_DECAY_RATE, time.delta_secs());
}

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (move_player, follow_player).chain())
        .run();
}

/// 生成玩家和摄像机。
fn setup(mut commands: Commands) {
    commands.spawn((Player, Transform::from_xyz(0.0, 0.0, 2.0)));
    commands.spawn(Camera2d);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_key_means_left_d_key_means_right() {
        let mut keys = ButtonInput::<KeyCode>::default();
        keys.press(KeyCode::KeyA);
        assert_eq!(input_direction(&keys), Vec2::new(-1.0, 0.0), "A 应该是向左");
        keys.press(KeyCode::KeyD);
        assert_eq!(input_direction(&keys), Vec2::ZERO, "同时按 A 和 D 方向抵消");
    }

    #[test]
    fn move_delta_scales_with_speed_and_time() {
        let delta = move_delta(Vec2::new(1.0, 0.0), 100.0, 0.5);
        assert!((delta.x - 50.0).abs() < 1e-6, "位移 = 速度 × 时间");
        assert_eq!(move_delta(Vec2::ZERO, 100.0, 1.0), Vec3::ZERO, "零方向不移动");
    }

    #[test]
    fn camera_nudges_toward_player() {
        let mut camera = Vec3::new(0.0, 0.0, 2.0);
        camera.smooth_nudge(&Vec3::new(50.0, 0.0, 2.0), CAMERA_DECAY_RATE, 1.0);
        assert!(camera.x > 0.0 && camera.x < 50.0, "相机应走一部分路程，而不是瞬移");
    }
}

// 提示：
// 1. 先运行 `bevylings run 1101`，按 A 键看看玩家往哪边跑。
// 2. 对比 KeyA 和 KeyD 两个分支：它们的 x 方向应该正好相反。
// 3. 改好后运行 `bevylings test 1101`，测试全绿就过关了。
