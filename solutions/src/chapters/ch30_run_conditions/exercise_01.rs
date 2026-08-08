//! # 练习 30.01 —— run_if 基础：条件为真才运行
//!
//! 出处：https://bevy.org/examples-webgpu/ecs/run-conditions/
//!
//! ## 概念
//! 用 `.run_if(条件)` 给系统加"运行条件"：每帧先判断条件，
//! 条件返回 `true` 才执行这个系统，否则这一帧直接跳过它。
//! 条件就是一个普通函数，只不过它只能读取数据，而且**必须返回 `bool`**。
//!
//! 本练习里，计数器只在"前 3 帧"（帧数 < 3）才会加 1。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 3001` 查看现象，改正后运行 `bevylings test 3001` 让测试通过。
//!
//! 小贴士：运行条件必须返回 `bool`，返回别的类型编译都过不了。

use bevy::prelude::*;

/// 记录当前帧数（每帧加 1）。
#[derive(Resource, Default)]
struct FrameCounter(u32);

/// 判断"现在是早期帧"：帧数小于 3 才算。
fn is_early(count: u32) -> bool {
    count < 3
}

/// 把 `is_early` 包装成 Bevy 能用的运行条件（读资源、返回 bool）。
fn is_early_frame(counter: Res<FrameCounter>) -> bool {
    is_early(counter.0)
}

/// 每帧给帧数加 1，但只在条件成立时运行。
fn increment(mut counter: ResMut<FrameCounter>) {
    counter.0 += 1;
}

pub fn run() {
    App::new()
        .init_resource::<FrameCounter>()
        .add_systems(Update, increment.run_if(is_early_frame))
        .run();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn early_frames_only() {
        assert!(is_early(0), "第 0 帧算早期帧");
        assert!(is_early(2), "第 2 帧也算");
        assert!(!is_early(3), "第 3 帧开始不是早期帧");
        assert!(!is_early(10), "第 10 帧更不是");
    }

    #[test]
    fn increment_only_runs_while_condition_holds() {
        let mut app = App::new();
        app.init_resource::<FrameCounter>();
        app.add_systems(Update, increment.run_if(is_early_frame));
        for _ in 0..5 {
            app.update();
        }
        let counter = app.world().resource::<FrameCounter>();
        assert_eq!(counter.0, 3, "前 3 帧各加了一次，之后条件为假不再运行");
    }
}

// 提示：
// 1. 先运行 `bevylings test 3001`，看看编译报错在哪里。
// 2. `run_if` 要求条件返回 bool：`true` 运行、`false` 跳过。
// 3. "小于 3"要写成比较表达式 `count < 3`，它的结果才是 bool。
