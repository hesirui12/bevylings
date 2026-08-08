//! # 练习 15.04 —— 边界与相交：AABB 和圆的碰撞判定
//!
//! 出处：https://bevy.org/examples-webgpu/math/bounding-2d/
//!
//! ## 概念
//! 判断两个形状是否"撞上"是游戏里最常用的数学。Bevy 提供两种简单的
//! **边界体积（bounding volume）**：
//! - `Aabb2d`：轴对齐包围盒，由中心和半尺寸定义；
//! - `BoundingCircle`：包围圆，由圆心和半径定义。
//!
//! 用 `IntersectsVolume` 特征提供的 `intersects` 方法判断两个体积
//! 是否相交：`a.intersects(&b)`。注意它接收的参数是引用。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 1504` 观察现象，改正后运行 `bevylings test 1504` 让测试通过。
//!
//! 小贴士：`intersects` 的参数类型是 `&Aabb2d`，别漏写引用符号 `&`。

use bevy::math::bounding::{Aabb2d, BoundingCircle, IntersectsVolume};
use bevy::prelude::*;

/// 两个轴对齐包围盒是否相交。
fn aabbs_overlap(a: Aabb2d, b: Aabb2d) -> bool {
    a.intersects(&b)
}

/// 点 p 是否在圆内（圆心 circle.center，半径 circle.radius()）。
fn point_in_circle(p: Vec2, circle: &BoundingCircle) -> bool {
    p.distance(circle.center) <= circle.radius()
}

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    // 演示：两个相距不远的包围盒是否相交，以及点是否在圆内
    let a = Aabb2d::new(Vec2::ZERO, Vec2::splat(50.0));
    let b = Aabb2d::new(Vec2::new(60.0, 0.0), Vec2::splat(50.0));
    info!("两个包围盒相交: {}", aabbs_overlap(a, b));
    let circle = BoundingCircle::new(Vec2::ZERO, 50.0);
    info!(
        "点 (30,40) 在圆内: {}",
        point_in_circle(Vec2::new(30.0, 40.0), &circle)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn overlapping_aabbs() {
        let a = Aabb2d::new(Vec2::ZERO, Vec2::splat(10.0));
        let b = Aabb2d::new(Vec2::new(5.0, 0.0), Vec2::splat(10.0));
        assert!(aabbs_overlap(a, b), "两个盒子中心只差 5，应该相交");
    }

    #[test]
    fn separated_aabbs() {
        let a = Aabb2d::new(Vec2::ZERO, Vec2::splat(10.0));
        let b = Aabb2d::new(Vec2::new(100.0, 0.0), Vec2::splat(10.0));
        assert!(!aabbs_overlap(a, b), "两个盒子相距很远，不该相交");
    }

    #[test]
    fn point_inside_and_outside_circle() {
        let circle = BoundingCircle::new(Vec2::ZERO, 10.0);
        assert!(point_in_circle(Vec2::new(3.0, 4.0), &circle), "距离 5 ≤ 半径 10");
        assert!(
            !point_in_circle(Vec2::new(10.1, 0.0), &circle),
            "距离 10.1 > 半径 10"
        );
    }
}

// 提示：
// 1. 先读编译错误，看 intersects 的参数需要什么类型。
// 2. `&` 表示引用：`intersects(&b)` 是把 b 的引用传进去，而不是把 b 本身传进去。
// 3. 修改后运行 `bevylings test 1504`，测试全绿就算过关。
