//! # 练习 14.01 —— 2D Gizmos：线、矩形、圆
//!
//! 出处：https://bevy.org/examples-webgpu/gizmos/2d-gizmos/
//!
//! ## 概念
//! Gizmos 是 Bevy 的"即时绘制"调试工具：把 `Gizmos` 作为系统参数，
//! 就能在屏幕上画线、矩形、圆等图形，每帧重画一次，非常适合画
//! 碰撞盒、瞄准线之类的调试信息，不影响游戏逻辑。
//!
//! 常用方法：
//! - `line_2d(起点, 终点, 颜色)`：画一条线段；
//! - `rect_2d(位置, 尺寸, 颜色)`：画矩形，尺寸是**完整**宽高；
//! - `circle_2d(圆心, 半径, 颜色)`：画圆。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 1401` 观察现象，改正后运行 `bevylings test 1401` 让测试通过。
//!
//! 小贴士：矩形给定的 size 是完整宽高，包围范围要由"中心 ± 尺寸的一半"得出。

use bevy::prelude::*;

/// 计算矩形的包围范围：(最小点, 最大点)。
/// `size` 是矩形的完整宽高，中心在 `center`。
fn rect_bounds(center: Vec2, size: Vec2) -> (Vec2, Vec2) {
    (center - size / 2.0, center + size / 2.0)
}

/// 画调试图形：十字线、圆和矩形。
fn draw_debug(mut gizmos: Gizmos) {
    let center = Vec2::ZERO;

    // 十字线：两条交叉的线段
    gizmos.line_2d(
        center - Vec2::X * 100.0,
        center + Vec2::X * 100.0,
        Color::WHITE,
    );
    gizmos.line_2d(
        center - Vec2::Y * 100.0,
        center + Vec2::Y * 100.0,
        Color::WHITE,
    );

    // 圆：圆心在原点，半径 40
    gizmos.circle_2d(center, 40.0, Color::WHITE);

    // 矩形：先算出包围范围，再用它确定位置和大小
    let size = Vec2::new(200.0, 100.0);
    let (min, max) = rect_bounds(center, size);
    gizmos.rect_2d(
        Isometry2d::from_translation((min + max) / 2.0),
        max - min,
        Color::WHITE,
    );
}

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, draw_debug)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bounds_centered_rect() {
        let (min, max) = rect_bounds(Vec2::ZERO, Vec2::splat(100.0));
        assert_eq!(min, Vec2::splat(-50.0), "最小点应在中心左下方");
        assert_eq!(max, Vec2::splat(50.0), "最大点应在中心右上方");
    }

    #[test]
    fn bounds_off_center_rect() {
        let (min, max) = rect_bounds(Vec2::new(10.0, 20.0), Vec2::new(80.0, 40.0));
        assert_eq!(min, Vec2::new(-30.0, 0.0));
        assert_eq!(max, Vec2::new(50.0, 40.0));
    }
}

// 提示：
// 1. 先运行 `bevylings run 1401`，看看画出来的矩形比圆和十字线大了多少。
// 2. "尺寸的一半"怎么表示？`Vec2` 可以做除法运算（`size / 2.0`）。
// 3. 修改后运行 `bevylings test 1401`，测试全绿就算过关。
