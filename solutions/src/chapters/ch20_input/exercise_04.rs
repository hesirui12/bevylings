//! # 练习 20.04 —— 手柄：按钮与摇杆
//!
//! 出处：https://bevy.org/examples-webgpu/input/gamepad-input/
//!
//! ## 概念
//! 手柄（Gamepad）也是普通实体：每插上一个手柄，Bevy 就给世界添加一个带
//! `Gamepad` 组件的实体。系统用查询拿到它，然后：
//! - `gamepad.just_pressed(GamepadButton::South)`：本帧是否刚按下某个按钮
//!   （South 是手柄正面下方的那个大按钮，Xbox 手柄上就是 A）。
//! - `gamepad.get(GamepadAxis::LeftStickX)`：左摇杆左右方向推了多少，
//!   返回值在 -1.0（推到底）到 1.0（推到底）之间。
//!
//! 摇杆有点小偏差很正常（手柄用久了会"漂移"），
//! 所以一般只关心"推得足够远"：**绝对值**超过一个阈值才算有效输入。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 2004` 查看现象，改正后运行 `bevylings test 2004` 让测试通过。
//!
//! 小贴士：推杆的方向无所谓，判断"推没推动"用绝对值 `abs()`。

use bevy::prelude::*;

/// 摇杆（或扳机）值有没有超过阈值？超过才算"真的在推"。
fn stick_is_active(value: f32, threshold: f32) -> bool {
    value.abs() > threshold
}

/// 描述一个手柄按钮这一帧的状态。
fn button_report(just_pressed: bool, just_released: bool) -> &'static str {
    if just_pressed {
        "刚按下"
    } else if just_released {
        "刚松开"
    } else {
        "无变化"
    }
}

/// 手柄系统：报告 South 按钮和左摇杆的状态。
fn gamepad_system(gamepads: Query<&Gamepad>) {
    for gamepad in &gamepads {
        let report = button_report(
            gamepad.just_pressed(GamepadButton::South),
            gamepad.just_released(GamepadButton::South),
        );
        if report != "无变化" {
            info!("South 按钮状态：{}", report);
        }
        if let Some(x) = gamepad.get(GamepadAxis::LeftStickX) {
            if stick_is_active(x, 0.01) {
                info!("左摇杆 X 轴推了：{}", x);
            }
        }
    }
}

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, gamepad_system)
        .run();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stick_active_past_threshold_in_both_directions() {
        assert!(stick_is_active(0.5, 0.1), "向右推了一半，应该算有效");
        assert!(stick_is_active(-0.8, 0.1), "向左推到底，绝对值够大，也算有效");
        assert!(!stick_is_active(0.05, 0.1), "轻微漂移不该算输入");
    }

    #[test]
    fn button_report_distinguishes_press_and_release() {
        assert_eq!(button_report(true, false), "刚按下");
        assert_eq!(button_report(false, true), "刚松开");
        assert_eq!(button_report(false, false), "无变化");
    }
}

// 提示：
// 1. `abs()` 取绝对值：-0.8 的绝对值是 0.8，足够大。
// 2. "超过阈值"用 `>`，"没超过"才是 `<`，想想哪个才是"真的在推"。
// 3. 修改后运行 `bevylings test 2004`，第一个测试会失败提醒你。
