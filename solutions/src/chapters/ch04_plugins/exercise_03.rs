//! # 练习 04.03 —— 自定义插件组（PluginGroup）
//!
//! 出处：https://bevy.org/examples-webgpu/application/plugin_group/
//!
//! ## 概念
//! 插件组把多个插件打包在一起注册。实现 `PluginGroup` 时，
//! 在 `build(self)` 里用 `PluginGroupBuilder::start::<Self>()` 创建构建器。
//! 注意 `start` 有一个泛型参数，它必须是**当前插件组的类型**（`Self`），
//! 编译器靠它记录插件组的名字。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 0403` 观察现象，改正后运行 `bevylings test 0403` 让测试通过。
//!
//! 小贴士：泛型函数在不写泛型参数时，编译器有时无法自己推断，会要求"类型标注"。

use bevy::{app::{PluginGroupBuilder, ScheduleRunnerPlugin}, prelude::*};

/// 记录两个插件系统各自的运行次数。
#[derive(Resource, Default)]
struct Counts {
    hello: u32,
    world: u32,
}

/// 一组产生 "hello world" 行为的插件。
pub struct HelloWorldPlugins;

impl PluginGroup for HelloWorldPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(PrintHelloPlugin)
            .add(PrintWorldPlugin)
    }
}

struct PrintHelloPlugin;

impl Plugin for PrintHelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, print_hello_system);
    }
}

fn print_hello_system(mut counts: ResMut<Counts>) {
    counts.hello += 1;
    println!("hello");
}

struct PrintWorldPlugin;

impl Plugin for PrintWorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, print_world_system);
    }
}

fn print_world_system(mut counts: ResMut<Counts>) {
    counts.world += 1;
    println!("world");
}

pub fn run() {
    App::new()
        .add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_once()))
        .insert_resource(Counts::default())
        .add_plugins(HelloWorldPlugins)
        .run();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_app() -> App {
        let mut app = App::new();
        app.insert_resource(Counts::default());
        app.add_plugins(HelloWorldPlugins);
        app
    }

    #[test]
    fn hello_system_runs() {
        let mut app = build_app();
        app.update();
        let counts = app.world().resource::<Counts>();
        assert_eq!(counts.hello, 1, "hello 系统应该运行一次");
    }

    #[test]
    fn world_system_runs_every_frame() {
        let mut app = build_app();
        app.update();
        app.update();
        let counts = app.world().resource::<Counts>();
        assert_eq!(counts.world, 2, "world 系统应该每帧运行");
    }
}

// 提示：
// 1. 报错大意是 "type annotations needed"，也就是泛型参数没写。
// 2. `start` 的泛型参数应该填"当前正在实现 PluginGroup 的类型"。
// 3. 修改后运行 `bevylings test 0403`，两个测试都通过就过关了。
