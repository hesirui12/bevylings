//! # 练习 23.01 —— 平滑跟随（插值缓动）
//!
//! 出处：https://bevy.org/examples-webgpu/movement/smooth_follow/
//!
//! ## 概念
//! 让一个物体"平滑地追着"另一个物体，最常用的办法是**插值缓动**：
//! 每一帧都朝目标方向走"剩余距离的一定比例"。离得远时走得快，
//! 离得近时走得慢，永远不会突然瞬移过去，看起来就很自然。
//! 公式里 `decay`（衰减率）越大追得越快；`delta` 是这一帧经过的秒数。
//! 数学上它等于 `当前 + (目标 - 当前) × (1 - e^(decay × delta))`。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 2301` 观察现象，改正后运行 `bevylings test 2301` 让测试通过。
//!
//! 小贴士：`e` 是自然常数（约 2.718），`(-2.0).exp()` 就是 e⁻²。
//! `decay × delta` 越大，`e^(-decay × delta)` 越小，移动的比例越大。

// I AM NOT DONE

use bevy::prelude::*;

/// 朝 `target` 靠近一步，返回新的位置。
/// `decay` 越大追得越快，`delta` 是这一帧经过的秒数。
fn smooth_follow(current: Vec3, target: Vec3, decay: f32, delta: f32) -> Vec3 {
    // BUG: 指数里的符号写反了。decay 越大，e 的指数应该越小，
    // 移动比例越大、跟得越快；现在却是 decay 越大动得越少甚至反向。
    current + (target - current) * (1.0 - (decay * delta).exp())
}

/// 被追的目标。
#[derive(Component)]
struct Target;

/// 追着目标跑的小方块。
#[derive(Component)]
struct Follower;

/// 追得多快（衰减率）。
#[derive(Resource)]
struct DecayRate(f32);

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(DecayRate(2.0))
        .add_systems(Startup, setup)
        .add_systems(Update, follow)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    // 目标：停在右侧
    commands.spawn((
        Sprite::from_color(Color::srgb(1.0, 0.6, 0.2), Vec2::new(40.0, 40.0)),
        Transform::from_xyz(300.0, 0.0, 0.0),
        Target,
    ));

    // 追随之物：从左侧出发
    commands.spawn((
        Sprite::from_color(Color::srgb(0.3, 0.6, 1.0), Vec2::new(40.0, 40.0)),
        Transform::from_xyz(-300.0, 0.0, 0.0),
        Follower,
    ));
}

/// 每帧让 Follower 朝 Target 平滑移动一步。
fn follow(
    target: Single<&Transform, (With<Target>, Without<Follower>)>,
    mut follower: Single<&mut Transform, With<Follower>>,
    decay: Res<DecayRate>,
    time: Res<Time>,
) {
    follower.translation =
        smooth_follow(follower.translation, target.translation, decay.0, time.delta_secs());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn already_there_means_no_movement() {
        let p = smooth_follow(Vec3::new(1.0, 2.0, 0.0), Vec3::new(1.0, 2.0, 0.0), 2.0, 0.1);
        assert!((p - Vec3::new(1.0, 2.0, 0.0)).length() < 1e-5);
    }

    #[test]
    fn moves_toward_target_not_away() {
        let p = smooth_follow(Vec3::ZERO, Vec3::new(10.0, 0.0, 0.0), 2.0, 1.0);
        // 应该前进了一段：比起点大、比目标小
        assert!(p.x > 0.0, "应该朝目标方向移动，实际 x = {}", p.x);
        assert!(p.x < 10.0);
    }

    #[test]
    fn bigger_decay_follows_faster() {
        let slow = smooth_follow(Vec3::ZERO, Vec3::new(10.0, 0.0, 0.0), 1.0, 1.0);
        let fast = smooth_follow(Vec3::ZERO, Vec3::new(10.0, 0.0, 0.0), 4.0, 1.0);
        assert!(fast.x > slow.x, "decay 越大应该跟得越快");
    }
}

// 提示：
// 1. 先运行 `bevylings run 2301`，看看小方块是不是越走越远（或原地不动）。
// 2. 想一想 e 的指数：想让"decay 越大 → 移动比例越大"，指数应该是什么符号？
// 3. 改好后运行 `bevylings test 2301`，三个测试全绿就过关了。
