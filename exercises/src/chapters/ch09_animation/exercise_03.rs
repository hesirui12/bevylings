//! # 练习 09.03 —— 动画组件：AnimationClip 与 AnimationPlayer
//!
//! 出处：https://bevy.org/examples-webgpu/animation/animated-transform/
//!
//! ## 概念
//! 一段动画（`AnimationClip`）由一条或多条**曲线**组成，每条曲线描述
//! 一个属性（比如 `Transform::translation`）随时间如何变化。关键帧用
//! `UnevenSampleAutoCurve` 保存"时间 → 值"的采样点，播放时自动在
//! 相邻关键帧之间插值。`AnimationPlayer` 负责真正播放它：
//! `play(节点编号)` 开始播放，`.repeat()` 让它循环。
//!
//! 想让动画**无缝循环**，最后一个关键帧必须和第一个完全相同，
//! 否则每循环一次就会看到一次明显的跳变。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 0903` 观察现象，改正后运行 `bevylings test 0903` 让测试通过。
//!
//! 小贴士：`animated_field!` 是 Bevy 提供的宏，用来把"组件字段"
//! （如 `Transform::translation`）变成动画要驱动的目标。

// I AM NOT DONE

use bevy::animation::{animated_field, AnimatedBy, AnimationTargetId};
use bevy::prelude::*;

/// 4 秒绕一圈的位移关键帧（正方形路径）。
/// 注意：为了让动画无缝循环，最后一个关键帧要与第一个相同。
fn loop_positions() -> [Vec3; 5] {
    [
        Vec3::new(1.0, 0.0, 1.0),
        Vec3::new(-1.0, 0.0, 1.0),
        Vec3::new(-1.0, 0.0, -1.0),
        Vec3::new(1.0, 0.0, -1.0),
        // BUG: 最后一个关键帧写成了 (1,1,1)，与第一个关键帧不一致，
        // 循环播放到结尾时位置会突然跳变；它应该和第一个关键帧相同。
        Vec3::new(1.0, 1.0, 1.0),
    ]
}

/// 用关键帧构建一段 4 秒的位移动画。
fn build_loop_animation() -> AnimationClip {
    let mut clip = AnimationClip::default();
    let target = AnimationTargetId::from_name(&Name::new("cube"));
    let curve = UnevenSampleAutoCurve::new(
        [0.0, 1.0, 2.0, 3.0, 4.0]
            .into_iter()
            .zip(loop_positions()),
    )
    .expect("时间戳必须严格递增");
    clip.add_curve_to_target(
        target,
        AnimatableCurve::new(animated_field!(Transform::translation), curve),
    );
    clip
}

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut clips: ResMut<Assets<AnimationClip>>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    // 把动画放进资源仓库，生成动画图，取到节点编号
    let (graph, node_index) = AnimationGraph::from_clip(clips.add(build_loop_animation()));
    let mut player = AnimationPlayer::default();
    player.play(node_index).repeat();

    // 方块：挂上动画图句柄和播放器，再告诉系统动画目标是谁
    let cube = commands
        .spawn((
            Mesh3d(meshes.add(Cuboid::new(0.5, 0.5, 0.5))),
            MeshMaterial3d(materials.add(Color::srgb(0.3, 0.9, 0.3))),
            Name::new("cube"),
            AnimationGraphHandle(graphs.add(graph)),
            player,
        ))
        .id();
    commands
        .entity(cube)
        .insert((AnimationTargetId::from_name(&Name::new("cube")), AnimatedBy(cube)));

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_keyframe_equals_last() {
        let positions = loop_positions();
        assert_eq!(
            positions[0], positions[4],
            "无缝循环要求首尾关键帧一致，实际首={:?} 尾={:?}",
            positions[0], positions[4]
        );
    }

    #[test]
    fn clip_covers_four_seconds() {
        let clip = build_loop_animation();
        assert_eq!(clip.duration(), 4.0, "动画时长应为 4 秒");
    }

    #[test]
    fn loop_matches_at_start_and_end() {
        let curve = UnevenSampleAutoCurve::new(
            [0.0, 1.0, 2.0, 3.0, 4.0]
                .into_iter()
                .zip(loop_positions()),
        )
        .expect("构建曲线失败");
        assert_eq!(
            curve.sample(0.0).unwrap(),
            curve.sample(4.0).unwrap(),
            "t=0 与 t=4 采样到的位置应该相同"
        );
    }
}

// 提示：
// 1. 先看 `loop_positions` 里 5 个关键帧，比较第 1 个和第 5 个是否一致。
// 2. 思考：动画循环播放时，结尾位置会直接接上开头位置，两者不同就会"跳变"。
// 3. 修改后运行 `bevylings test 0903`，测试全绿就算过关。
