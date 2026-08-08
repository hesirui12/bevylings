//! # 练习 11.02 —— 相机环绕：用鼠标绕目标旋转（Orbit Camera）
//!
//! 出处：https://bevy.org/examples-webgpu/camera/camera_orbit/
//!
//! ## 概念
//! 环绕相机（orbit camera）像“绕着地球转的卫星”：相机始终朝向目标点，
//! 位置由“距离 + 朝向”决定：`位置 = 目标 - 朝向 × 距离`。
//! 鼠标上下移动改变俯仰角（pitch），左右移动改变偏航角（yaw），
//! 再用 `Quat::from_euler` 把角度合成旋转四元数赋给相机。
//! 俯仰角要限制在 ±90° 附近，否则相机会翻过头。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 1102` 观察现象，改正后运行 `bevylings test 1102` 让测试通过。
//!
//! 小贴士：`Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll)` 需要四个参数；
//! 我们不使用滚转，第三个角填 `0.0` 即可。

use bevy::{input::mouse::AccumulatedMouseMotion, prelude::*};

/// 相机与目标的距离。
const ORBIT_DISTANCE: f32 = 20.0;
/// 鼠标灵敏度。
const PITCH_SPEED: f32 = 0.003;
const YAW_SPEED: f32 = 0.004;
/// 俯仰角上限（接近 90°，防止相机翻过头）。
const PITCH_LIMIT: f32 = std::f32::consts::FRAC_PI_2 - 0.01;

/// 把俯仰角限制在 ±PITCH_LIMIT 之间。
fn clamp_pitch(pitch: f32) -> f32 {
    pitch.clamp(-PITCH_LIMIT, PITCH_LIMIT)
}

/// 根据鼠标移动更新相机的朝向和位置。
fn orbit(
    mut camera: Query<&mut Transform, With<Camera3d>>,
    mouse_motion: Res<AccumulatedMouseMotion>,
) {
    let Ok(mut camera) = camera.single_mut() else {
        return;
    };

    let delta = mouse_motion.delta;
    let delta_pitch = delta.y * PITCH_SPEED;
    let delta_yaw = delta.x * YAW_SPEED;

    let (yaw, pitch, _) = camera.rotation.to_euler(EulerRot::YXZ);
    let pitch = clamp_pitch(pitch + delta_pitch);
    let yaw = yaw + delta_yaw;

    camera.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, 0.0);

    // 让相机始终“面向目标”，并保持固定距离。
    camera.translation = Vec3::ZERO - camera.forward() * ORBIT_DISTANCE;
}

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, orbit)
        .run();
}

/// 生成相机、一个方块和一盏灯（环绕的目标是世界原点）。
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.8, 0.7, 0.6))),
        Transform::from_xyz(1.5, 0.5, 1.5),
    ));
    commands.spawn((
        PointLight::default(),
        Transform::from_xyz(3.0, 8.0, 5.0),
    ));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pitch_is_clamped() {
        assert_eq!(clamp_pitch(3.0), PITCH_LIMIT, "过大的俯仰角应被压到上限");
        assert_eq!(clamp_pitch(-3.0), -PITCH_LIMIT, "过小的俯仰角应被压到下限");
    }

    #[test]
    fn pitch_passes_through_when_in_range() {
        assert_eq!(clamp_pitch(0.0), 0.0, "范围内的角度保持不变");
        assert_eq!(clamp_pitch(0.5), 0.5);
        assert_eq!(clamp_pitch(PITCH_LIMIT), PITCH_LIMIT, "等于上限时保持不变");
    }
}

// 提示：
// 1. 运行 `bevylings run 1102`，看看编译器说“expected 4 arguments”在哪一行。
// 2. `Quat::from_euler` 的三个角是 yaw / pitch / roll，我们不用 roll，填 `0.0`。
// 3. 改好后运行 `bevylings test 1102`，测试全绿就过关了。
