//! # 练习 20.02 —— 鼠标：ButtonInput<MouseButton> 与 AccumulatedMouseMotion
//!
//! 出处：https://bevy.org/examples-webgpu/input/mouse-input/
//!
//! ## 概念
//! 鼠标有两种输入：
//! - 按键：资源 `ButtonInput<MouseButton>`，`MouseButton::Left / Right / Middle`
//!   是三个键，用法和键盘一模一样（`pressed` / `just_pressed` / `just_released`）。
//! - 移动：资源 `AccumulatedMouseMotion` 记录"这一帧鼠标移动了多少"，
//!   它的 `delta` 字段是一个 `Vec2`：x 是水平位移，y 是垂直位移。
//!   如果这一帧鼠标没动，`delta` 就是 `Vec2::ZERO`（零向量）。
//!
//! 我们判断"鼠标这一帧有没有动过"，并检查"左键是否刚被点下"。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 2002` 查看现象，改正后运行 `bevylings test 2002` 让测试通过。
//!
//! 小贴士：`Vec2::ZERO` 是 (0, 0)；只要 x 或 y 有一个不是 0，鼠标就算动过。

// I AM NOT DONE

use bevy::{
    input::mouse::AccumulatedMouseMotion,
    prelude::*,
};

/// 这一帧鼠标动过吗？
fn mouse_moved(motion: &AccumulatedMouseMotion) -> bool {
    // BUG: 比较写反了：现在"鼠标没动（delta 是零向量）"反而返回 true。
    // "动过" 应该判断 delta **不是**零向量。
    motion.delta == Vec2::ZERO
}

/// 左键刚被点下吗？
fn left_clicked(buttons: &ButtonInput<MouseButton>) -> bool {
    buttons.just_pressed(MouseButton::Left)
}

/// 鼠标输入系统：左键刚按下时打印提示，鼠标动了时打印位移。
fn mouse_input_system(
    buttons: Res<ButtonInput<MouseButton>>,
    motion: Res<AccumulatedMouseMotion>,
) {
    if left_clicked(&buttons) {
        info!("left mouse just pressed");
    }
    if mouse_moved(&motion) {
        info!("mouse moved ({}, {})", motion.delta.x, motion.delta.y);
    }
}

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, mouse_input_system)
        .run();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn moved_delta_means_mouse_moved() {
        let moved = AccumulatedMouseMotion {
            delta: Vec2::new(1.0, -2.0),
        };
        assert!(mouse_moved(&moved), "有位移就算动过");

        let idle = AccumulatedMouseMotion { delta: Vec2::ZERO };
        assert!(!mouse_moved(&idle), "delta 是零向量，说明鼠标没动");
    }

    #[test]
    fn only_left_button_counts_as_left_click() {
        let mut buttons = ButtonInput::<MouseButton>::default();
        buttons.press(MouseButton::Right);
        assert!(!left_clicked(&buttons), "按的是右键，不算左键点击");
        buttons.press(MouseButton::Left);
        assert!(left_clicked(&buttons), "左键刚按下");
    }
}

// 提示：
// 1. `Vec2::ZERO` 表示 (0, 0)。"动过" 和 "没动过" 是一对相反的条件。
// 2. 修改后运行 `bevylings test 2002`，第一个测试会失败提醒你。
