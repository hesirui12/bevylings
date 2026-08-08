//! # 练习 11.03 —— 投影缩放：用滚轮缩放正交相机
//!
//! 出处：https://bevy.org/examples-webgpu/camera/projection_zoom/
//!
//! ## 概念
//! 相机的投影方式决定 3D 场景怎么被“压”到屏幕上。
//! `Projection::Orthographic`（正交投影）没有近大远小，适合俯视角/UI；
//! 它通过 `OrthographicProjection.scale` 控制缩放：scale 越小画面越“近”（放大），
//! 越大越“远”（缩小）。缩放是**乘法**的：`scale × (1 + 增量)`。
//! 滚轮向上（`delta.y > 0`）应该放大，所以增量要取负号。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 1103` 观察现象，改正后运行 `bevylings test 1103` 让测试通过。
//!
//! 小贴士：滚轮向上是 `delta.y > 0`，想要“放大”就得让 scale 变小。

use bevy::{camera::ScalingMode, input::mouse::AccumulatedMouseScroll, prelude::*};

/// 滚轮灵敏度。
const ZOOM_SPEED: f32 = 0.2;
/// scale 允许的范围。
const MIN_SCALE: f32 = 0.1;
const MAX_SCALE: f32 = 10.0;

/// 把滚轮增量换算成新的 scale，并限制在范围内。
fn new_scale(scale: f32, wheel_delta_y: f32, speed: f32, min: f32, max: f32) -> f32 {
    let delta_zoom = -wheel_delta_y * speed;
    (scale * (1.0 + delta_zoom)).clamp(min, max)
}

/// 每帧读取滚轮输入并缩放相机。
fn zoom(
    mut camera: Query<&mut Projection, With<Camera3d>>,
    mouse_wheel: Res<AccumulatedMouseScroll>,
) {
    let Ok(mut camera) = camera.single_mut() else {
        return;
    };
    match *camera.into_inner() {
        Projection::Orthographic(ref mut ortho) => {
            ortho.scale = new_scale(ortho.scale, mouse_wheel.delta.y, ZOOM_SPEED, MIN_SCALE, MAX_SCALE);
        }
        _ => {}
    }
}

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, zoom)
        .run();
}

/// 生成一个正交投影相机。
fn setup(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Projection::from(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: 5.0,
            },
            scale: 1.0,
            ..OrthographicProjection::default_3d()
        }),
        Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scroll_up_zooms_in() {
        let s = new_scale(1.0, 1.0, ZOOM_SPEED, MIN_SCALE, MAX_SCALE);
        assert!(s < 1.0, "向上滚应该缩小 scale（放大画面），实际 {s}");
    }

    #[test]
    fn scroll_down_zooms_out() {
        let s = new_scale(1.0, -1.0, ZOOM_SPEED, MIN_SCALE, MAX_SCALE);
        assert!(s > 1.0, "向下滚应该增大 scale（缩小画面），实际 {s}");
    }

    #[test]
    fn scale_is_clamped() {
        let small = new_scale(0.05, 1.0, ZOOM_SPEED, MIN_SCALE, MAX_SCALE);
        assert_eq!(small, MIN_SCALE, "低于下限应被钳制");
        let big = new_scale(100.0, -1.0, ZOOM_SPEED, MIN_SCALE, MAX_SCALE);
        assert_eq!(big, MAX_SCALE, "超过上限应被钳制");
    }
}

// 提示：
// 1. 先运行 `bevylings run 1103`，向上滚轮看看画面是放大还是缩小。
// 2. “滚轮向上要放大”意味着：delta_y > 0 时 scale 要减小，所以增量带负号。
// 3. 改好后运行 `bevylings test 1103`，测试全绿就过关了。
