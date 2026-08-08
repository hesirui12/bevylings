//! # 练习 21.03 —— Flex 布局：方向、主轴与交叉轴
//!
//! 出处：https://bevy.org/examples-webgpu/ui/flex_layout/
//!
//! ## 概念
//! Bevy 的 UI 用 Flex 布局排列子节点：`flex_direction` 决定排布方向
//! （`Row` 从左到右，`Column` 从上到下）。沿排布方向的那条轴叫**主轴**，
//! 用 `justify_content` 控制；垂直它的轴叫**交叉轴**，用 `align_items` 控制。
//! 两个都设成 `Center`，内容就会在对应方向上居中。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 2103` 观察面板方向，改正后运行
//! `bevylings test 2103` 让测试通过。
//!
//! 小贴士：`Node` 是个普通结构体，可以直接在测试里构造并检查它的字段，
//! 不一定要启动整个 Bevy 应用。

use bevy::prelude::*;

/// 生成一个铺满全屏的面板容器：横排还是竖排由参数决定
fn make_panel(is_vertical: bool) -> Node {
    Node {
        width: percent(100),
        height: percent(100),
        flex_direction: if is_vertical {
            FlexDirection::Column
        } else {
            FlexDirection::Row
        },
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    }
}

/// 启动时生成相机和一个垂直面板，里面放两个彩色方块
fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands
        .spawn((make_panel(true), BackgroundColor(Color::BLACK)))
        .with_children(|parent| {
            parent.spawn((
                Node {
                    width: px(100),
                    height: px(50),
                    ..default()
                },
                BackgroundColor(Color::srgb(1.0, 0.0, 0.0)),
            ));
            parent.spawn((
                Node {
                    width: px(100),
                    height: px(50),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.0, 0.0, 1.0)),
            ));
        });
}

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vertical_panel_uses_column() {
        let node = make_panel(true);
        assert_eq!(node.flex_direction, FlexDirection::Column);
        assert_eq!(node.justify_content, JustifyContent::Center);
        assert_eq!(node.align_items, AlignItems::Center);
    }

    #[test]
    fn horizontal_panel_uses_row_and_fills_screen() {
        let node = make_panel(false);
        assert_eq!(node.flex_direction, FlexDirection::Row);
        assert_eq!(node.width, percent(100), "面板应铺满整个窗口宽度");
        assert_eq!(node.height, percent(100), "面板应铺满整个窗口高度");
    }
}

// 提示：
// 1. 先运行 `bevylings run 2103`：两个方块本来是上下叠（竖排），现在排错了方向。
// 2. `is_vertical` 为 true 时应该用 `FlexDirection::Column`，看看是不是反了。
// 3. 改好后运行 `bevylings test 2103`，两个测试都通过就过关了。
