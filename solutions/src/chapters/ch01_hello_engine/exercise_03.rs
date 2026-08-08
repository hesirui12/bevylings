//! # 练习 01.03 —— 多个 Update 系统与执行顺序（chain）
//!
//! 出处：https://bevy.org/learn/quick-start/getting-started/ecs/
//!
//! ## 概念
//! 一个 App 里可以注册很多个 Update 系统，Bevy 会尽量让它们**并行**运行，
//! 执行先后没有保证。但有些系统之间有依赖关系（比如"先计数、后记录"），
//! 这时可以用 `.chain()` 把几个系统串联起来，保证它们按书写顺序依次执行。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 0103` 观察现象，改正后运行 `bevylings test 0103` 让测试通过。
//!
//! 小贴士：`.chain()` 要求写在前面的系统先跑，写在后面的后跑。

use bevy::prelude::*;

/// 全局计数器：每帧加 1。
#[derive(Resource, Default)]
struct Counter(u32);

/// 记录每一帧"计数之后"的值，方便测试观察执行顺序。
#[derive(Resource, Default)]
struct History(Vec<u32>);

/// 把计数器加 1。
fn increment(mut counter: ResMut<Counter>) {
    counter.0 += 1;
}

/// 把当前计数值追加进历史记录。
fn record(mut history: ResMut<History>, counter: Res<Counter>) {
    history.0.push(counter.0);
}

/// 注册"计数 + 记录"两个系统，并用 chain() 保证顺序：先计数，后记录。
fn add_counter_systems(app: &mut App) {
    app.add_systems(Update, (increment, record).chain());
}

/// 每帧打印一行问候语（它和计数无关，可以和其他系统并行）。
fn hello_world() {
    println!("hello world!");
}

pub fn run() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .insert_resource(Counter::default())
        .insert_resource(History::default());
    add_counter_systems(&mut app);
    app.add_systems(Update, hello_world);
    app.run();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_app() -> App {
        let mut app = App::new();
        app.insert_resource(Counter::default());
        app.insert_resource(History::default());
        add_counter_systems(&mut app);
        app
    }

    #[test]
    fn record_sees_value_after_increment() {
        let mut app = build_app();
        app.update();
        app.update();
        app.update();
        let history = app.world().resource::<History>();
        assert_eq!(history.0, vec![1, 2, 3], "先计数后记录，历史应该是 1、2、3");
    }

    #[test]
    fn counter_reaches_expected_value() {
        let mut app = build_app();
        app.update();
        app.update();
        app.update();
        let counter = app.world().resource::<Counter>();
        assert_eq!(counter.0, 3, "跑了 3 帧，计数应该是 3");
    }
}

// 提示：
// 1. 运行测试，看看 History 里记录到的值是什么。
// 2. 如果记录到的是 0、1、2，说明 record 比 increment 先跑了。
//    chain() 里哪个系统写在前面，哪个就先跑。
// 3. 修改后运行 `bevylings test 0103`，两个测试都通过就过关了。
