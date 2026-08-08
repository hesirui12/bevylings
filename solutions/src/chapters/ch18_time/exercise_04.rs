//! # 练习 18.04 —— 虚拟时间：Time<Virtual> 与速度缩放
//!
//! 出处：https://bevy.org/examples-webgpu/time/virtual_time/
//!
//! ## 概念
//! 默认的 `Time` 就是**虚拟时间** `Time<Virtual>`：它可以给时间"加速/减速"，
//! 实现子弹时间、慢动作。`set_relative_speed(2.0)` 让时间流速翻倍，
//! `relative_speed()` 读当前倍率。真实时间 `Time<Real>` 永远不受影响。
//!
//! 本练习里物体按虚拟时间移动：流速 2 倍，物体就移动 2 倍远。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 1804` 查看现象，改正后运行 `bevylings test 1804` 让测试通过。
//!
//! 小贴士：虚拟时间里走过的路程 = 真实秒数 × 倍率。

use bevy::prelude::*;

/// 标记：这个实体跟随虚拟时间。
#[derive(Component)]
struct VirtualMover;

/// 虚拟时间下物体应处的位置：真实秒数 × 流速倍率。
fn virtual_position(elapsed_secs: f32, relative_speed: f32) -> f32 {
    elapsed_secs * relative_speed
}

/// 每帧按虚拟时间更新位置。
fn move_virtual(mut query: Query<&mut Transform, With<VirtualMover>>, time: Res<Time<Virtual>>) {
    for mut transform in &mut query {
        transform.translation.x = virtual_position(time.elapsed_secs(), time.relative_speed());
    }
}

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, move_virtual)
        .run();
}

/// 生成相机和一个跟随虚拟时间的方块。
fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn((
        VirtualMover,
        Sprite::from_color(Color::srgb(0.9, 0.6, 0.2), Vec2::new(40.0, 40.0)),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn double_speed_moves_twice_as_far() {
        assert_eq!(virtual_position(10.0, 2.0), 20.0, "10 秒 × 2 倍速 = 20");
    }

    #[test]
    fn half_speed_moves_half_as_far() {
        assert_eq!(virtual_position(10.0, 0.5), 5.0, "10 秒 × 0.5 倍速 = 5");
    }

    #[test]
    fn normal_speed_moves_one_to_one() {
        assert_eq!(virtual_position(10.0, 1.0), 10.0, "正常流速一一对应");
    }
}

// 提示：
// 1. 先运行 `bevylings run 1804`，观察方块移动的速度是否与"倍率"矛盾。
// 2. 倍率大于 1 表示"更快"，路程应该更大 —— 乘法还是除法？
// 3. 把 `/` 改成 `*` 再运行 `bevylings test 1804`。
