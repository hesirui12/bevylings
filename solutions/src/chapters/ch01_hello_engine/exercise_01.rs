//! # 练习 01.01 —— 最小 App：什么都不做的程序
//!
//! 出处：https://bevy.org/learn/quick-start/getting-started/apps/
//!
//! ## 概念
//! 每一个 Bevy 程序都从一个 `App` 开始。`App::new()` 会创建一个空的 App，
//! 它就像一个空壳子，自己什么都不会做。
//! 要让 App 真正"跑起来"，需要调用 `.run()` —— 它会启动游戏循环。
//! （对于没有任何插件的空 App，循环只跑一帧就自然结束。）
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 0101` 观察现象，改正后运行 `bevylings test 0101` 让测试通过。
//!
//! 小贴士：`use bevy::prelude::*;` 会把最常用的 Bevy 类型一次性导入，
//! 后面的每个练习开头都会有这一行。

use bevy::prelude::*;

/// run() 是本练习的入口函数（由脚手架调用）。
pub fn run() {
    App::new().run();
}

/// 返回 App 启动时打印的欢迎语。
fn welcome() -> &'static str {
    "Hello, Bevy!"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn welcome_message_is_correct() {
        assert_eq!(welcome(), "Hello, Bevy!", "欢迎语应该和代码里写的一致");
    }

    #[test]
    fn empty_app_can_update_without_exiting() {
        let mut app = App::new();
        app.update();
        app.update();
        assert!(
            app.should_exit().is_none(),
            "只手动 update、不发送退出消息，App 不应该自行退出"
        );
    }
}

// 提示：
// 1. 先看 `App::new()` 后面跟的那个方法名，想一想它真的存在吗？
// 2. 编译器会告诉你"不存在这个方法"。启动游戏循环的方法名其实更短，
//    试试去掉方法名里多余的部分。
// 3. 修改后运行 `bevylings test 0101`，两个测试都通过就过关了。
