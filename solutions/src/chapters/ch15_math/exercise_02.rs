//! # 练习 15.02 —— 样条曲线：CubicHermite 位置插值
//!
//! 出处：https://bevy.org/examples-webgpu/math/cubic-splines/
//!
//! ## 概念
//! 样条（spline）是一种"曲线插值"工具：给定几个点，它能生成一条
//! 平滑穿过这些点的曲线。`CubicHermite` 是其中一种，除了每个点的
//! **位置**，还需要每个点的**切线**（tangent）——切线决定曲线离开
//! 这个点时往哪个方向弯，就像提前"掰好"了弯曲的方向。
//!
//! 构造步骤：`CubicHermite::new(点列表, 切线列表)` 生成样条，
//! `.to_curve()` 得到可采样的曲线，`.position(t)` 取 t 时刻的位置
//! （t=0 是起点，t=1 是终点）。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 1502` 观察现象，改正后运行 `bevylings test 1502` 让测试通过。
//!
//! 小贴士：Hermite 样条需要两样东西：位置和切线，缺一不可。

use bevy::prelude::*;

/// 构建一条 Hermite 样条：从 (0,0) 出发、经过 (100,0) 的平滑曲线。
fn build_curve() -> CubicCurve<Vec2> {
    let points = [Vec2::new(0.0, 0.0), Vec2::new(100.0, 0.0)];
    let tangents = [Vec2::new(0.0, 50.0), Vec2::new(0.0, -50.0)];

    CubicHermite::new(points, tangents)
        .to_curve()
        .expect("至少需要两个控制点")
}

/// 取曲线上 t 时刻的位置。
fn spline_position(t: f32) -> Vec2 {
    build_curve().position(t)
}

/// 把曲线画出来：拆成 100 小段用 gizmos 画折线，
/// 再画一个进度点（t=0.75）演示 position 采样。
fn draw_spline(mut gizmos: Gizmos) {
    gizmos.linestrip_2d(build_curve().iter_positions(100), Color::WHITE);
    gizmos.circle_2d(spline_position(0.75), 4.0, Color::WHITE);
}

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, draw_spline)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn starts_and_ends_at_points() {
        assert_eq!(spline_position(0.0), Vec2::ZERO, "t=0 应在起点");
        assert_eq!(spline_position(1.0), Vec2::new(100.0, 0.0), "t=1 应在终点");
    }

    #[test]
    fn midpoint_is_halfway() {
        let mid = spline_position(0.5);
        assert!(
            (mid.x - 50.0).abs() < 1e-3,
            "t=0.5 时 x 应在中间，实际 {mid:?}"
        );
        assert!(
            (mid.y - 12.5).abs() < 1e-3,
            "y 由切线控制，实际 {mid:?}"
        );
    }
}

// 提示：
// 1. 先读编译错误，看 CubicHermite::new 期望几个参数。
// 2. 概念说明里写了：Hermite 需要"点列表 + 切线列表"，把漏掉的补上。
// 3. 修改后运行 `bevylings test 1502`，测试全绿就算过关。
