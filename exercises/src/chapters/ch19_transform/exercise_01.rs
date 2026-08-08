//! # 练习 19.01 —— 平移：让物体动起来
//!
//! 出处：https://bevy.org/examples-webgpu/transforms/translation/
//!
//! ## 概念
//! 每个可见实体都有 `Transform`（变换），由三部分组成：
//! `translation`（位置）、`rotation`（旋转）、`scale`（缩放）。
//! 本练习用 `Transform::from_translation(...)` 设定初始位置，
//! 然后每帧让物体沿自己的局部 X 轴移动：
//! `translation += 方向 × 速度 × 帧时长`，帧时长（delta）保证不同帧率下速度一致。
//!
//! 物体不能走太远：离出生点超过 `max_distance` 就掉头。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 1901` 查看现象，改正后运行 `bevylings test 1901` 让测试通过。
//!
//! 小贴士：`(spawn - translation).length()` 表示"离出生点有多远"。

// I AM NOT DONE

use bevy::prelude::*;

/// 可移动物体：记录出生点、最大移动距离和速度。
#[derive(Component)]
struct Movable {
    spawn: Vec3,
    max_distance: f32,
    speed: f32,
}

/// 判断"离出生点太远，需要掉头"。
fn should_flip(distance: f32, max_distance: f32) -> bool {
    // BUG: 比较符号写反了：物体刚出发（距离很小）就被判定"超距"掉头，
    // 结果在出生点附近来回抖动，永远走不远。
    distance < max_distance
}

/// 每帧把物体沿局部 X 轴推一步，超距就掉头。
fn move_cube(time: Res<Time>, mut query: Query<(&mut Transform, &mut Movable)>) {
    for (mut transform, mut cube) in &mut query {
        let distance = (cube.spawn - transform.translation).length();
        if should_flip(distance, cube.max_distance) {
            cube.speed *= -1.0;
        }
        let step = transform.local_x() * cube.speed * time.delta_secs();

        transform.translation += step;
    }
}

/// 生成一个可移动的立方体、相机和灯光。
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let spawn = Vec3::ZERO;
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_translation(spawn),
        Movable {
            spawn,
            max_distance: 5.0,
            speed: 2.0,
        },
    ));

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 10.0, 20.0).looking_at(spawn, Vec3::Y),
    ));
    commands.spawn((
        DirectionalLight::default(),
        Transform::from_xyz(3.0, 3.0, 3.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, move_cube)
        .run();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flips_when_past_the_limit() {
        assert_eq!(should_flip(6.0, 5.0), true, "6 > 5，应该掉头");
        assert_eq!(should_flip(10.0, 5.0), true);
    }

    #[test]
    fn keeps_direction_inside_the_limit() {
        assert_eq!(should_flip(4.9, 5.0), false, "还没超距，不该掉头");
        assert_eq!(should_flip(5.0, 5.0), false, "正好在边界上也算没超");
    }
}

// 提示：
// 1. 想一想"距离超过最大值"应该用大于还是小于。
// 2. 掉头判断只在 `distance` 和 `max_distance` 之间比较，别的行都不用动。
// 3. 改完运行 `bevylings test 1901`，测试全绿就过关。
