//! # 练习 02.05 —— 线程池资源（TaskPool）
//!
//! 出处：https://bevy.org/examples-webgpu/application/thread_pool_resources/
//!
//! ## 概念
//! Bevy 用**线程池**（`TaskPool`）来并行执行任务，线程数越多并行能力越强。
//! `TaskPoolPlugin` 里的 `TaskPoolOptions` 控制线程数：
//! `with_num_threads(n)` 把总线程数固定在 n 个
//! （对应 `min_total_threads` 和 `max_total_threads` 两个字段）。
//! 运行期间可以用 `ComputeTaskPool::get()` 查到实际线程数。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 0205` 观察现象，改正后运行 `bevylings test 0205` 让测试通过。
//!
//! 小贴士：`TaskPoolOptions` 是"配置"，构造它不会真的开线程，测试起来很安全。

// I AM NOT DONE

use bevy::prelude::*;
use bevy::tasks::{ComputeTaskPool, TaskPool};

/// 创建一个 4 线程的任务池配置。
fn four_thread_pool() -> TaskPoolOptions {
    // BUG: 这里把线程数写成了别的值，导致配置出来的线程数不符合预期。
    TaskPoolOptions::with_num_threads(2)
}

/// 每帧打印当前的计算线程数。
fn print_threads() {
    println!("compute threads: {}", ComputeTaskPool::get().thread_num());
}

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins.set(TaskPoolPlugin {
            task_pool_options: four_thread_pool(),
        }))
        .add_systems(Update, print_threads)
        .run();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn four_thread_pool_uses_four_threads() {
        let options = four_thread_pool();
        assert_eq!(options.max_total_threads, 4, "最多 4 个线程");
        assert_eq!(options.min_total_threads, 4, "至少 4 个线程");
    }

    #[test]
    fn a_task_pool_always_has_threads() {
        let pool = TaskPool::new();
        assert!(pool.thread_num() >= 1, "任务池至少要有 1 个线程");
    }
}

// 提示：
// 1. 函数的意图都写在注释里了："4 线程的任务池配置"。
// 2. `with_num_threads(n)` 的参数 n 会把 min 和 max 都设为 n。
// 3. 修改后运行 `bevylings test 0205`，两个测试都通过就过关了。
