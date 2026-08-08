//! # 练习 15.03 —— 自定义数学原语：实现 Bounded2d
//!
//! 出处：https://bevy.org/examples-webgpu/math/custom-primitives/
//!
//! ## 概念
//! Bevy 的数学原语（圆形、矩形、多边形……）都能回答两个问题：
//! - **轴对齐包围盒（AABB）**：包住这个形状的最小矩形，用 `Aabb2d` 表示；
//! - **包围圆（BoundingCircle）**：包住这个形状的最小圆。
//!
//! 让一个形状能回答这两个问题的"接口"叫 `Bounded2d` 特征，实现
//! `aabb_2d` 和 `bounding_circle` 两个方法即可。游戏里做精确碰撞
//! 之前，通常先用这种简单形状做粗略判断，速度快、省计算。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 1503` 观察现象，改正后运行 `bevylings test 1503` 让测试通过。
//!
//! 小贴士：AABB 的两个角是"中心 ± 半尺寸"；外接圆半径用勾股定理。

use bevy::math::bounding::{Aabb2d, Bounded2d, BoundingCircle, BoundingVolume};
use bevy::prelude::*;

/// 一个自定义的 2D 数学原语：正方形。
#[derive(Clone, Copy, Debug)]
struct Square {
    half_size: f32,
}

impl Primitive2d for Square {}

impl Bounded2d for Square {
    /// 轴对齐包围盒：中心 ± 半尺寸。
    fn aabb_2d(&self, isometry: impl Into<Isometry2d>) -> Aabb2d {
        let isometry = isometry.into();
        let half = Vec2::splat(self.half_size);
        Aabb2d {
            min: isometry.translation - half,
            max: isometry.translation + half,
        }
    }

    /// 包围圆：正方形的外接圆（半径 = 半尺寸 × √2）。
    fn bounding_circle(&self, isometry: impl Into<Isometry2d>) -> BoundingCircle {
        let isometry = isometry.into();
        let radius = self.half_size * 2.0_f32.sqrt();
        BoundingCircle::new(isometry.translation, radius)
    }
}

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, draw_bounds)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

/// 画一个正方形以及它的包围盒与包围圆。
fn draw_bounds(mut gizmos: Gizmos) {
    let square = Square { half_size: 50.0 };
    let isometry = Isometry2d::IDENTITY;

    let aabb = square.aabb_2d(isometry);
    gizmos.rect_2d(
        Isometry2d::from_translation(aabb.center()),
        aabb.half_size() * 2.0,
        Color::WHITE,
    );

    let circle = square.bounding_circle(isometry);
    gizmos.circle_2d(circle.center, circle.radius(), Color::WHITE);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aabb_is_centered() {
        let square = Square { half_size: 2.0 };
        let aabb = square.aabb_2d(Isometry2d::IDENTITY);
        assert_eq!(aabb.min, Vec2::new(-2.0, -2.0));
        assert_eq!(aabb.max, Vec2::new(2.0, 2.0));
    }

    #[test]
    fn aabb_follows_translation() {
        let square = Square { half_size: 1.0 };
        let iso = Isometry2d::from_translation(Vec2::new(10.0, -5.0));
        let aabb = square.aabb_2d(iso);
        assert_eq!(aabb.min, Vec2::new(9.0, -6.0));
        assert_eq!(aabb.max, Vec2::new(11.0, -4.0));
    }

    #[test]
    fn bounding_circle_covers_corners() {
        let square = Square { half_size: 2.0 };
        let circle = square.bounding_circle(Isometry2d::IDENTITY);
        assert_eq!(circle.center, Vec2::ZERO);
        assert!(
            (circle.radius() - 2.0 * 2.0_f32.sqrt()).abs() < 1e-5,
            "外接圆半径应为 2√2，实际 {}",
            circle.radius()
        );
    }
}

// 提示：
// 1. 先运行 `bevylings run 1503`，看看包围盒是不是反着画的。
// 2. AABB 的 min 角应小于 max 角：min = 中心 − 半尺寸，max = 中心 + 半尺寸。
// 3. 修改后运行 `bevylings test 1503`，测试全绿就算过关。
