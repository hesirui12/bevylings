//! # 练习 09.02 —— 缓动函数：EaseFunction 家族
//!
//! 出处：https://bevy.org/examples-webgpu/animation/easing-functions/
//!
//! ## 概念
//! 缓动函数（easing function）回答一个问题：进度 t（0 到 1）已经走了
//! 百分之多少，动画实际上应该走到哪一步？`EaseFunction::Linear` 是
//! 匀速（t 是多少就走多少），`QuadraticIn` 是先慢后快，
//! `BounceOut` 则像皮球落地一样带着反弹，各有各的手感。
//!
//! 在 Bevy 里，`EaseFunction` 本身就可以被当作一条**曲线（Curve）**
//! 来用：曲线是一种能按参数 t 采样取值的东西，`sample(t)` 返回
//! t 处的输出（一个 0..=1 之间的 f32）。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 0902` 观察现象，改正后运行 `bevylings test 0902` 让测试通过。
//!
//! 小贴士：方法名拼写要准确，`Curve` 特征提供的是 `sample` 采样方法。

// I AM NOT DONE

use bevy::prelude::*;

/// 计算缓动函数在进度 t 处的输出。
fn ease(function: EaseFunction, t: f32) -> f32 {
    // BUG: EaseFunction 没有叫 `evaluate` 的方法，编译会报
    // "no method named `evaluate`"。它实现了 Curve<f32> 特征，
    // 应该用特征提供的采样方法取出 t 处的值。
    function.evaluate(t).unwrap()
}

/// 演示用的方块：沿 x 轴滑动，用 QuadraticIn 加速。
#[derive(Component)]
struct EasedSquare;

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, animate)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn((
        Sprite::from_color(Color::WHITE, Vec2::new(20.0, 40.0)),
        Transform::from_xyz(-300.0, 0.0, 0.0),
        EasedSquare,
    ));
}

fn animate(time: Res<Time>, mut query: Query<&mut Transform, With<EasedSquare>>) {
    let t = time.elapsed_secs().rem_euclid(2.0) / 2.0;
    let x = ease(EaseFunction::QuadraticIn, t) * 600.0 - 300.0;
    for mut transform in &mut query {
        transform.translation.x = x;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linear_is_identity() {
        assert!((ease(EaseFunction::Linear, 0.25) - 0.25).abs() < 1e-5);
        assert!((ease(EaseFunction::Linear, 0.8) - 0.8).abs() < 1e-5);
    }

    #[test]
    fn quadratic_in_starts_slow() {
        assert_eq!(ease(EaseFunction::QuadraticIn, 0.0), 0.0);
        assert!((ease(EaseFunction::QuadraticIn, 1.0) - 1.0).abs() < 1e-5);
        // t² 在 t=0.5 处是 0.25，说明前半程走得慢
        assert!((ease(EaseFunction::QuadraticIn, 0.5) - 0.25).abs() < 1e-5);
    }

    #[test]
    fn bounce_stays_in_range() {
        let y = ease(EaseFunction::BounceOut, 0.5);
        assert!(y >= 0.0 && y <= 1.0, "缓动输出应在 0..=1，实际 {y}");
    }
}

// 提示：
// 1. 先读编译错误：它在告诉你 EaseFunction 上找不到哪个方法。
// 2. EaseFunction 实现了 Curve<f32> 特征，Curve 提供了 `sample` / `sample_clamped` 等方法。
// 3. 修改后运行 `bevylings test 0902`，测试全绿就算过关。
