//! # 练习 14.02 —— 3D Gizmos：球、射线、立方体
//!
//! 出处：https://bevy.org/examples-webgpu/gizmos/3d-gizmos/
//!
//! ## 概念
//! 3D 调试绘制和 2D 用法几乎一样，只是方法换了名字：
//! - `sphere(圆心, 半径, 颜色)`：球体线框；
//! - `ray(起点, 方向向量, 颜色)`：一条从起点沿方向发出的射线；
//! - `cube(变换, 颜色)`：立方体线框；
//! - `arrow(起点, 终点, 颜色)`：带箭头的线。
//!
//! `Gizmos` 系统参数在每帧结束时会清空重画，所以叫"即时绘制"，
//! 非常适合调试时随手画几笔。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 1402` 观察现象，改正后运行 `bevylings test 1402` 让测试通过。
//!
//! 小贴士：`sphere` 需要三个参数（圆心、半径、颜色），检查有没有漏参数。

// I AM NOT DONE

use bevy::prelude::*;

/// 射线终点：从 origin 出发，沿 direction 方向走 length 的距离。
fn ray_end(origin: Vec3, direction: Vec3, length: f32) -> Vec3 {
    origin + direction * length
}

/// 画一组 3D 调试图形。
fn draw_debug(mut gizmos: Gizmos) {
    let origin = Vec3::new(1.0, 0.5, 0.0);

    // 球体线框：圆心、半径、颜色
    // BUG: sphere 需要三个参数（圆心、半径、颜色），这里漏掉了颜色参数，
    // 编译会报"参数个数不对"。
    gizmos.sphere(origin, 1.0);

    // 射线：从 origin 指向 x 正方向
    let end = ray_end(origin, Vec3::X, 2.0);
    gizmos.ray(origin, end - origin, Color::WHITE);

    // 立方体线框
    gizmos.cube(
        Transform::from_translation(Vec3::new(-2.0, 0.0, 0.0)),
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
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 2.0, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ray_endpoint_moves_along_direction() {
        assert_eq!(
            ray_end(Vec3::ZERO, Vec3::X, 5.0),
            Vec3::new(5.0, 0.0, 0.0)
        );
        assert_eq!(
            ray_end(Vec3::new(1.0, 1.0, 1.0), Vec3::Y, 2.0),
            Vec3::new(1.0, 3.0, 1.0)
        );
    }

    #[test]
    fn ray_length_matches_distance() {
        let origin = Vec3::new(-3.0, 0.0, 0.0);
        let end = ray_end(origin, Vec3::X, 3.0);
        assert_eq!(end, Vec3::ZERO);
        assert!((origin.distance(end) - 3.0).abs() < 1e-5);
    }
}

// 提示：
// 1. 先读编译错误，它告诉你在 gizmos.sphere(...) 这行少了什么。
// 2. 对照概念说明里 sphere 的三个参数：圆心、半径、颜色。
// 3. 修改后运行 `bevylings test 1402`，测试全绿就算过关。
