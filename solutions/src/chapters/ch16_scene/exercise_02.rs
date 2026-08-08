//! # 练习 16.02 —— BSN 交互：场景里的按钮与 on 观察者
//!
//! 出处：https://bevy.org/examples-webgpu/scene/bsn/
//!
//! ## 概念
//! 场景不只是"摆造型"——它还能给实体挂**观察者**（observer）。
//! 在 BSN 里用 `on(...)` 给实体绑定一个事件处理闭包：
//! `on(|_event: On<Pointer<Press>>| ...)` 表示"鼠标或手指按下这个实体时，
//! 执行闭包里的代码"。`Pointer<Press>` 就是"按下"这一事件。
//!
//! 闭包里写的是普通 Rust 代码。我们把"按下后说什么"抽成纯函数
//! `press_message`，方便用单元测试验证，场景里只管调用它。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 1602` 查看现象，改正后运行 `bevylings test 1602` 让测试通过。
//!
//! 小贴士：`on(...)` 里的闭包参数 `On<Pointer<Press>>` 是固定写法，
//! 本练习不用改它，重点在 `press_message` 的判断逻辑。

use bevy::prelude::*;

/// 整张场景：相机 + 两个可点击的按钮。
fn scene() -> impl SceneList {
    bsn_list![Camera2d, ui()]
}

/// 根 UI 容器：垂直排列两个按钮。
fn ui() -> impl Scene {
    bsn! {
        Node {
            width: percent(100),
            height: percent(100),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            row_gap: px(10),
        }
        Children [
            (
                button("确定")
                on(|_event: On<Pointer<Press>>| println!("{}", press_message("确定")))
            ),
            (
                button("取消")
                on(|_event: On<Pointer<Press>>| println!("{}", press_message("取消")))
            ),
        ]
    }
}

/// 一个标准按钮实体。
fn button(label: &str) -> impl Scene {
    bsn! {
        Button
        Node {
            width: px(150),
            height: px(65),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
        }
        BackgroundColor(Color::srgb(0.15, 0.15, 0.15))
        Children [(
            Text(label)
            TextColor(Color::srgb(0.9, 0.9, 0.9))
        )]
    }
}

/// 按下按钮后打印什么消息？"确定"确认，"取消"放弃。
fn press_message(label: &str) -> String {
    if label == "确定" {
        format!("{label}：确认！")
    } else {
        format!("{label}：已取消")
    }
}

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, scene.spawn())
        .run();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ok_button_confirms() {
        assert_eq!(press_message("确定"), "确定：确认！");
    }

    #[test]
    fn cancel_button_cancels() {
        assert_eq!(press_message("取消"), "取消：已取消");
    }

    #[test]
    fn messages_are_different() {
        assert_ne!(press_message("确定"), press_message("取消"));
    }
}

// 提示：
// 1. 两个分支分别对应"确定"和"取消"，想想判断条件应该怎么写。
// 2. `!=`（不等于）和 `==`（等于）只有一字之差，方向却完全相反。
// 3. 改好后运行 `bevylings test 1602`，三个测试全绿就过关。
