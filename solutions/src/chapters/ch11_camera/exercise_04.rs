//! # 练习 11.04 —— 平滑跟随：让物体滑向目标（smooth_nudge）
//!
//! 出处：https://bevy.org/examples-webgpu/movement/smooth_follow/
//!
//! ## 概念
//! 让一个物体“追”另一个物体，直接瞬移太生硬。`smooth_nudge` 提供帧率无关的
//! 平滑插值：`位置 += (目标 - 位置) × (1 - e^(-衰减率 × 帧时长))`，
//! 衰减率越大追得越快。注意它的签名：
//! `smooth_nudge(&mut self, target: &Self, decay_rate: f32, delta_time: f32)`
//! —— 目标要以**引用**的形式传进去。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 1104` 观察现象，改正后运行 `bevylings test 1104` 让测试通过。
//!
//! 小贴士：Bevy 里“位置”是 `Vec3`，它实现了 `smooth_nudge` 这个平滑方法。

use bevy::prelude::*;

#[derive(Component)]
struct TargetSphere;

#[derive(Component)]
struct FollowingSphere;

/// 追赶的衰减率。
#[derive(Resource)]
struct DecayRate(f32);

/// 目标每帧沿圆周移动（假装它自己在跑）。
fn move_target(mut target: Query<&mut Transform, With<TargetSphere>>, time: Res<Time>) {
    let Ok(mut target) = target.single_mut() else {
        return;
    };
    let t = time.elapsed_secs();
    target.translation = Vec3::new(t.sin() * 3.0, t.cos() * 3.0, 0.0);
}

/// 让跟随球平滑地滑向目标球。
fn move_follower(
    mut following: Query<&mut Transform, With<FollowingSphere>>,
    target: Query<&Transform, (With<TargetSphere>, Without<FollowingSphere>)>,
    decay_rate: Res<DecayRate>,
    time: Res<Time>,
) {
    let Ok(mut following) = following.single_mut() else {
        return;
    };
    let Ok(target) = target.single() else {
        return;
    };

    following
        .translation
        .smooth_nudge(&target.translation, decay_rate.0, time.delta_secs());
}

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(DecayRate(2.0))
        .add_systems(Startup, setup)
        .add_systems(Update, (move_target, move_follower).chain())
        .run();
}

/// 生成目标球、跟随球、相机和一盏灯。
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let sphere = meshes.add(Sphere::new(0.3));
    let blue = materials.add(Color::srgb(0.3, 0.15, 0.9));
    let red = materials.add(Color::srgb(0.9, 0.3, 0.3));

    commands.spawn((TargetSphere, Mesh3d(sphere.clone()), MeshMaterial3d(blue)));
    commands.spawn((
        FollowingSphere,
        Mesh3d(sphere.clone()),
        MeshMaterial3d(red),
        Transform::from_translation(Vec3::new(0.0, -2.0, 0.0)),
    ));
    commands.spawn((
        PointLight::default(),
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.0, 3.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nudge_moves_partway_not_all_the_way() {
        let mut pos = Vec3::ZERO;
        pos.smooth_nudge(&Vec3::new(10.0, 0.0, 0.0), 2.0, 0.5);
        assert!(pos.x > 0.0 && pos.x < 10.0, "应该移动一部分而不是全部，实际 {}", pos.x);
    }

    #[test]
    fn higher_decay_follows_faster() {
        let mut slow = Vec3::ZERO;
        let mut fast = Vec3::ZERO;
        slow.smooth_nudge(&Vec3::new(10.0, 0.0, 0.0), 1.0, 1.0);
        fast.smooth_nudge(&Vec3::new(10.0, 0.0, 0.0), 4.0, 1.0);
        assert!(fast.x > slow.x, "衰减率越大，单帧移动越多");
        assert!(fast.x < 10.0, "一帧之内不会超过目标");
    }
}

// 提示：
// 1. 运行 `bevylings run 1104`，看看编译器说“expected `&Vec3`, found `Vec3`”在哪。
// 2. `smooth_nudge` 的第一个参数是“目标的引用”，给它加上 `&` 就行。
// 3. 改好后运行 `bevylings test 1104`，测试全绿就过关了。
