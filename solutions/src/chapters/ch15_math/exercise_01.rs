//! # 练习 15.01 —— 向量运算：距离与方向
//!
//! 出处：https://bevy.org/examples-webgpu/math/bounding-2d/
//!
//! ## 概念
//! `Vec2` 是一个二维向量，既可以表示"位置"也可以表示"位移/方向"。
//! 常用运算：
//! - 加减：`a + b` 把两个位移叠加；`b - a` 是"从 a 到 b 的位移"；
//! - `length()`：向量的长度（勾股定理）；
//! - `normalize()`：把向量缩放到长度 1，只保留方向；
//! - `distance(a, b)`：两点之间的距离。
//!
//! 判断"点是否落在圆里"用的就是这套数学：点到圆心的距离 ≤ 半径。
//! 这也是 Bevy 碰撞检测等功能的底层基础。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 1501` 观察现象，改正后运行 `bevylings test 1501` 让测试通过。
//!
//! 小贴士："从 a 到 b 的位移"是 `b - a`，先想清楚减号方向再动手。

use bevy::prelude::*;

/// 两点之间的直线距离。
fn distance(a: Vec2, b: Vec2) -> f32 {
    (b - a).length()
}

/// 从 from 指向 to 的单位方向向量。
fn direction(from: Vec2, to: Vec2) -> Vec2 {
    (to - from).normalize()
}

/// 判断点 p 是否落在圆心 target、半径 radius 的圆内。
fn point_in_circle(p: Vec2, target: Vec2, radius: f32) -> bool {
    distance(p, target) <= radius
}

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    // 演示：打印方向向量，以及两个点是否在圆内
    let dir = direction(Vec2::ZERO, Vec2::new(3.0, 4.0));
    info!("从原点到 (3,4) 的单位方向: {dir:?}");
    for p in [Vec2::new(30.0, 40.0), Vec2::new(200.0, 0.0)] {
        info!(
            "点 {p:?} 在半径 50 的圆内: {}",
            point_in_circle(p, Vec2::ZERO, 50.0)
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distance_is_pythagorean() {
        assert!((distance(Vec2::ZERO, Vec2::new(3.0, 4.0)) - 5.0).abs() < 1e-5);
        assert!(
            (distance(Vec2::new(1.0, 1.0), Vec2::new(4.0, 5.0)) - 5.0).abs() < 1e-5,
            "Δx=3, Δy=4，距离应为 5"
        );
    }

    #[test]
    fn direction_is_unit_vector() {
        assert_eq!(direction(Vec2::ZERO, Vec2::new(0.0, 5.0)), Vec2::Y);
        let d = direction(Vec2::ZERO, Vec2::new(3.0, 4.0));
        assert!(
            (d.x - 0.6).abs() < 1e-5 && (d.y - 0.8).abs() < 1e-5,
            "方向向量应归一化，实际 {d:?}"
        );
    }

    #[test]
    fn point_in_circle_check() {
        // 真实距离 ≈ 44.7 ≤ 50，应在圆内
        assert!(point_in_circle(Vec2::new(30.0, 40.0), Vec2::new(10.0, 0.0), 50.0));
        // 明显在圆外
        assert!(!point_in_circle(Vec2::new(200.0, 0.0), Vec2::ZERO, 50.0));
    }
}

// 提示：
// 1. 先运行 `bevylings run 1501`，看看日志里两个点的判定结果是不是反了。
// 2. "两点距离"可以拆成两步：先求位移向量（终点减起点），再对它求长度。
// 3. 修改后运行 `bevylings test 1501`，测试全绿就算过关。
