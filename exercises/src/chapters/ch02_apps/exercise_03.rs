//! # 练习 02.03 —— 自定义插件组（PluginGroup）
//!
//! 出处：https://bevy.org/examples-webgpu/application/plugin_group/
//!
//! ## 概念
//! 多个插件可以打包成一个**插件组**（`PluginGroup`），一次性注册进 App，
//! 就像 `DefaultPlugins`、`MinimalPlugins` 那样。
//! 实现 `PluginGroup` 时，`build(self)` 返回一个 `PluginGroupBuilder`，
//! 用 `.add(...)` 把插件一个个加进去。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 0203` 观察现象，改正后运行 `bevylings test 0203` 让测试通过。
//!
//! 小贴士：`.add()` 期望的是"插件"（实现了 Plugin 的类型），而不是系统函数。

// I AM NOT DONE

use bevy::{app::{PluginGroupBuilder, ScheduleRunnerPlugin}, prelude::*};

/// 记录两个插件的系统各自的运行情况。
#[derive(Resource, Default)]
struct Log(Vec<String>);

/// 一组插件：让 App 打印 "hello" 和 "world"。
pub struct HelloWorldPlugins;

impl PluginGroup for HelloWorldPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(PrintHelloPlugin)
            // BUG: 这一行传给 .add() 的是一个"系统函数"而不是"插件"，
            // 编译器会报 trait 不满足的错误。
            .add(print_world_system)
    }
}

struct PrintHelloPlugin;

impl Plugin for PrintHelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, print_hello_system);
    }
}

fn print_hello_system(mut log: ResMut<Log>) {
    log.0.push("hello".to_string());
    println!("hello");
}

struct PrintWorldPlugin;

impl Plugin for PrintWorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, print_world_system);
    }
}

fn print_world_system(mut log: ResMut<Log>) {
    log.0.push("world".to_string());
    println!("world");
}

pub fn run() {
    App::new()
        .add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_once()))
        .insert_resource(Log::default())
        .add_plugins(HelloWorldPlugins)
        .run();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn group_registers_hello_plugin() {
        let mut app = App::new();
        app.insert_resource(Log::default());
        app.add_plugins(HelloWorldPlugins);
        app.update();
        let log = app.world().resource::<Log>();
        assert!(
            log.0.contains(&"hello".to_string()),
            "插件组里应该有打印 hello 的插件，实际: {:?}",
            log.0
        );
    }

    #[test]
    fn group_registers_world_plugin() {
        let mut app = App::new();
        app.insert_resource(Log::default());
        app.add_plugins(HelloWorldPlugins);
        app.update();
        let log = app.world().resource::<Log>();
        assert!(
            log.0.contains(&"world".to_string()),
            "插件组里应该有打印 world 的插件，实际: {:?}",
            log.0
        );
    }
}

// 提示：
// 1. 报错大意是：the trait bound ...: Plugin is not satisfied。
//    也就是说 .add() 收到的东西不是"插件"。
// 2. 看看本文件里定义了哪些"插件"（实现了 Plugin 的 struct）。
// 3. 修改后运行 `bevylings test 0203`，两个测试都通过就过关了。
