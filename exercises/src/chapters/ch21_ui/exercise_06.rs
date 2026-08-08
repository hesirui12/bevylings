//! # 练习 21.06 —— 大小约束：min_width / max_width
//!
//! 出处：https://bevy.org/examples-webgpu/ui/size_constraints/
//!
//! ## 概念
//! 除了直接指定 `width` / `height`，`Node` 还可以设置**大小约束**：
//! `min_width` 和 `min_height` 规定"至少多大"，`max_width` 和
//! `max_height` 规定"至多多大"。这在做进度条、面板等"有下限"的
//! UI 时非常有用——内容再怎么缩小，节点也不会被压垮。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 2106` 会先遇到一个编译错误，改正后运行
//! `bevylings test 2106` 让测试通过。
//!
//! 小贴士：`Node` 里表示"最小宽度"的字段名是 `min_width`
//! （最小高度是 `min_height`），没有 `min_size` 这种写法。

// I AM NOT DONE

use bevy::prelude::*;

/// 生成一个进度条节点：宽度可变，但至少 100 像素宽、20 像素高
fn progress_bar(value: Val) -> Node {
    Node {
        width: value,
        // BUG: 字段名写错了：`Node` 里没有这个字段，
        // 表示"最小宽度"的正确字段名不是它。
        min_size: px(100),
        height: px(20),
        ..default()
    }
}

/// 启动时生成相机和一个进度条
fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn((
        progress_bar(px(50)),
        BackgroundColor(Color::srgb(0.2, 0.6, 1.0)),
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
    fn bar_has_minimum_width() {
        let node = progress_bar(px(50));
        assert_eq!(node.min_width, px(100), "进度条至少 100 像素宽");
        assert_eq!(node.width, px(50), "宽度取传入的值");
    }

    #[test]
    fn bar_has_fixed_height_and_free_max_width() {
        let node = progress_bar(percent(60));
        assert_eq!(node.height, px(20));
        assert_eq!(node.max_width, Val::Auto, "默认不限制最大宽度");
    }
}

// 提示：
// 1. 编译错误会提示"没有名为 `min_size` 的字段"（no field）。
// 2. 在 `Node` 里，宽度约束叫 `min_width` / `max_width`，高度约束叫 `min_height` / `max_height`。
// 3. 改好后运行 `bevylings test 2106`，两个测试都通过就过关了。
