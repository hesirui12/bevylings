//! # 练习 21.04 —— 锚点定位：PositionType::Absolute 与边距
//!
//! 出处：https://bevy.org/examples-webgpu/ui/anchor_layout/
//!
//! ## 概念
//! 默认情况下，UI 节点会被 Flex 布局自动摆放。如果想让某个节点
//! "钉"在窗口的某个位置（比如左上角），可以把它的 `position_type`
//! 设为 `PositionType::Absolute`，然后用 `left` / `top` / `right` / `bottom`
//! 指定它相对于父节点的偏移。这样它就不再参与其它节点的排队了。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 2104` 会先遇到一个编译错误，改正后运行
//! `bevylings test 2104` 让测试通过。
//!
//! 小贴士：`Node` 有很多字段，写结构体字面量时通常要用
//! `..default()` 把没写到的字段全部补成默认值。

use bevy::prelude::*;

/// 生成一个锚定节点：位置由 left/top 决定，与父容器其它元素无关
fn anchored_node(left: Val, top: Val) -> Node {
    Node {
        position_type: PositionType::Absolute,
        left,
        top,
        ..default()
    }
}

/// 启动时生成相机和一个钉在左上角的标签
fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands
        .spawn((
            anchored_node(px(10), px(10)),
            BackgroundColor(Color::srgb(0.92, 0.14, 0.05)),
        ))
        .with_child((
            Text::new("left-top"),
            TextFont {
                font_size: FontSize::Px(20.0),
                ..default()
            },
        ));
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
    fn anchored_node_uses_absolute_positioning() {
        let node = anchored_node(px(10), px(20));
        assert_eq!(node.position_type, PositionType::Absolute);
        assert_eq!(node.left, px(10));
        assert_eq!(node.top, px(20));
    }

    #[test]
    fn anchored_node_keeps_given_edges() {
        let node = anchored_node(px(30), px(40));
        assert_eq!(node.left, px(30));
        assert_eq!(node.top, px(40));
        assert_eq!(node.flex_direction, FlexDirection::Row, "其余字段应保持默认值");
    }
}

// 提示：
// 1. 编译错误会列出一大堆"缺少字段"，这是因为结构体字面量没有写全。
// 2. `Node { ... }` 写完后加上 `..default()`，剩下的字段就自动用默认值补齐。
// 3. 改好后运行 `bevylings test 2104`，两个测试都通过就过关了。
