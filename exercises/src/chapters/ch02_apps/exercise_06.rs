//! # 练习 02.06 —— 无渲染但保留逻辑（no_renderer）
//!
//! 出处：https://bevy.org/examples-webgpu/application/no_renderer/
//!
//! ## 概念
//! `DefaultPlugins` 默认包含渲染器，但有些场景（比如集成测试、CI）我们只
//! 想跑游戏逻辑、不想真正画图。把 `RenderPlugin` 的 `backends` 设为 `None`
//! 就能"关掉渲染"：窗口还在，但不再创建 GPU 渲染管线。
//! 游戏逻辑（比如这里的计算）照常运行。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 0206` 观察现象，改正后运行 `bevylings test 0206` 让测试通过。
//!
//! 小贴士：`WgpuSettings` 里的 `backends: None` 表示"不启用任何图形后端"。

// I AM NOT DONE

use bevy::prelude::*;
use bevy::render::{settings::WgpuSettings, RenderPlugin};

/// 核心"游戏逻辑"：把两个数加起来（纯函数，方便测试）。
fn compute_sum(a: u32, b: u32) -> u32 {
    // BUG: 这里把加法写成了乘法，计算结果不对。
    a * b
}

/// 每帧打印一次计算结果。
fn logic_system() {
    println!("1 + 2 = {}", compute_sum(1, 2));
}

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins.set(RenderPlugin {
            render_creation: WgpuSettings {
                backends: None,
                ..default()
            }
            .into(),
            ..default()
        }))
        .add_systems(Update, logic_system)
        .run();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn computes_correct_sum() {
        assert_eq!(compute_sum(1, 2), 3, "1 + 2 应该等于 3");
        assert_eq!(compute_sum(10, 5), 15, "10 + 5 应该等于 15");
    }

    #[test]
    fn sum_works_for_zero() {
        assert_eq!(compute_sum(0, 5), 5, "0 + 5 应该等于 5");
        assert_eq!(compute_sum(7, 8), 15, "7 + 8 应该等于 15");
    }
}

// 提示：
// 1. 函数名叫 compute_sum（求和），注释也写了"把两个数加起来"。
// 2. 看看运算符号：`+`、`-`、`*` 里哪个才是"加"。
// 3. 修改后运行 `bevylings test 0206`，两个测试都通过就过关了。
