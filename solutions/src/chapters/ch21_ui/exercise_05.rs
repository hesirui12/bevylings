//! # 练习 21.05 —— 边框与背景：BorderColor 与 BackgroundColor
//!
//! 出处：https://bevy.org/examples-webgpu/ui/borders/
//!
//! ## 概念
//! 一个 UI 节点可以同时有背景色和边框：`BackgroundColor` 决定节点的
//! 填充颜色，`BorderColor` 决定边框颜色。要让四边都有边框，
//! 还得先在 `Node` 里设置 `border: UiRect::all(px(4))` 指定边框宽度。
//! `BorderColor::all(颜色)` 是一个便捷函数，一次性把四条边设成同一种颜色。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 2105` 观察颜色，改正后运行
//! `bevylings test 2105` 让测试通过。
//!
//! 小贴士：`BorderColor` 有 `top` / `bottom` / `left` / `right` 四个公开字段，
//! 测试里可以直接比较单个边。

use bevy::prelude::*;

const ACTIVE_BORDER: Color = Color::srgb(0.1, 0.9, 0.1);
const INACTIVE_BORDER: Color = Color::srgb(0.3, 0.3, 0.3);
const ACTIVE_BG: Color = Color::srgb(0.9, 0.9, 0.2);
const INACTIVE_BG: Color = Color::srgb(0.2, 0.2, 0.2);

/// 根据"选中"状态返回边框颜色
fn border_for(active: bool) -> BorderColor {
    if active {
        BorderColor::all(ACTIVE_BORDER)
    } else {
        BorderColor::all(INACTIVE_BORDER)
    }
}

/// 根据"选中"状态返回背景颜色
fn background_for(active: bool) -> BackgroundColor {
    if active {
        BackgroundColor(ACTIVE_BG)
    } else {
        BackgroundColor(INACTIVE_BG)
    }
}

/// 启动时生成相机和一个"选中"状态的卡片
fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn((
        Node {
            width: px(200),
            height: px(80),
            border: UiRect::all(px(4)),
            ..default()
        },
        border_for(true),
        background_for(true),
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
    fn active_panel_uses_active_colors() {
        assert_eq!(border_for(true).top, ACTIVE_BORDER, "选中时边框应是亮绿色");
        assert_eq!(border_for(true).right, ACTIVE_BORDER);
        assert_eq!(background_for(true).0, ACTIVE_BG, "选中时背景应是亮黄色");
    }

    #[test]
    fn inactive_panel_uses_inactive_colors() {
        assert_eq!(border_for(false).top, INACTIVE_BORDER);
        assert_eq!(border_for(false).bottom, INACTIVE_BORDER);
        assert_eq!(background_for(false).0, INACTIVE_BG);
    }
}

// 提示：
// 1. 先运行 `bevylings run 2105`，看看"选中"的卡片边框是什么颜色。
// 2. `border_for(active)` 里两个分支是不是接反了？
// 3. 改好后运行 `bevylings test 2105`，两个测试都通过就过关了。
