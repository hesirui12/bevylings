//! # 练习 10.04 —— 空间音频：让“听者”动起来（SpatialListener）
//!
//! 出处：https://bevy.org/examples-webgpu/audio/spatial_audio_3d/
//!
//! ## 概念
//! 空间音频让声音听起来“有方向”：给发声体加上
//! `PlaybackSettings::LOOP.with_spatial(true)`，再在世界里放一个
//! `SpatialListener`（相当于两只耳朵），声音就会根据听者与发声体的
//! 相对位置自动调整左右声道和音量。
//! 移动 `SpatialListener` 所在实体的位置，就能模拟“人走来走去”的效果。
//! 我们把“按键 → 位移”抽成了纯函数 `movement`，方便在测试里验证。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 1004` 观察现象，改正后运行 `bevylings test 1004` 让测试通过。
//!
//! 小贴士：方向键和坐标轴要一致：`ArrowRight` 是 x 变大，`ArrowLeft` 是 x 变小。

use bevy::prelude::*;

/// 听者的移动速度（世界单位/秒）。
const LISTENER_SPEED: f32 = 2.0;

/// 把方向键状态翻译成本帧位移：返回值 x 表示左右移动，y 表示前后移动。
fn movement(keys: &ButtonInput<KeyCode>, speed: f32, delta_secs: f32) -> Vec2 {
    let mut delta = Vec2::ZERO;
    if keys.pressed(KeyCode::ArrowRight) {
        delta.x += speed * delta_secs;
    }
    if keys.pressed(KeyCode::ArrowLeft) {
        delta.x -= speed * delta_secs;
    }
    if keys.pressed(KeyCode::ArrowDown) {
        delta.y += speed * delta_secs;
    }
    if keys.pressed(KeyCode::ArrowUp) {
        delta.y -= speed * delta_secs;
    }
    delta
}

/// 根据方向键移动听者（带 SpatialListener 的实体）。
fn update_listener(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut listener: Query<&mut Transform, With<SpatialListener>>,
) {
    let Ok(mut listener) = listener.single_mut() else {
        return;
    };
    let delta = movement(&keyboard, LISTENER_SPEED, time.delta_secs());
    listener.translation.x += delta.x;
    listener.translation.z += delta.y;
}

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, update_listener)
        .run();
}

/// 生成一个带空间音频的发声体和一只“听者”。
fn setup(asset_server: Res<AssetServer>, mut commands: Commands) {
    // 发声体：循环播放 + 开启空间音频。
    commands.spawn((
        AudioPlayer::new(asset_server.load("sounds/Windless Slopes.ogg")),
        PlaybackSettings::LOOP.with_spatial(true),
        Transform::from_xyz(3.0, 0.0, 0.0),
    ));

    // 听者：两只耳朵相隔 4 个世界单位。
    commands.spawn((
        SpatialListener::new(4.0),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    commands.spawn(Camera3d::default());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn right_key_moves_right() {
        let mut keys = ButtonInput::<KeyCode>::default();
        keys.press(KeyCode::ArrowRight);
        let delta = movement(&keys, 2.0, 0.5);
        assert_eq!(delta.x, 1.0, "按右键应该向右移动 1 个单位，实际 {}", delta.x);
        assert_eq!(delta.y, 0.0, "左右键不该影响前后");
    }

    #[test]
    fn left_key_moves_left() {
        let mut keys = ButtonInput::<KeyCode>::default();
        keys.press(KeyCode::ArrowLeft);
        let delta = movement(&keys, 2.0, 0.5);
        assert_eq!(delta.x, -1.0, "按左键应该向左移动 1 个单位，实际 {}", delta.x);
    }

    #[test]
    fn no_keys_no_movement() {
        let keys = ButtonInput::<KeyCode>::default();
        assert_eq!(movement(&keys, 2.0, 0.5), Vec2::ZERO, "不按键就不动");
    }
}

// 提示：
// 1. 先运行 `bevylings run 1004`，按 → 键听听声音往哪边偏。
// 2. 对比 ArrowRight 和 ArrowLeft 两个分支：它们的作用应该正好相反。
// 3. 改好后运行 `bevylings test 1004`，测试全绿就过关了。
