//! # 练习 04.04 —— 插件配置字段与 DefaultPlugins.set
//!
//! 出处：https://bevy.org/examples-webgpu/application/settings/
//!
//! ## 概念
//! 插件最常见的用法之一就是"可配置"：把可变的部分（标题、颜色、开关……）
//! 做成结构体字段，创建插件时传入，插件在 `build` 里把它们变成资源或行为。
//! 同时，`DefaultPlugins.set(...)` 可以整体替换默认插件组里的某个插件，
//! 比如换掉默认的 `WindowPlugin` 来修改窗口标题。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 0404` 观察现象，改正后运行 `bevylings test 0404` 让测试通过。
//!
//! 小贴士：配置字段是给别人用的"API"，在代码里写死常量就失去可配置的意义了。

use bevy::prelude::*;

/// 窗口标题资源，供系统读取。
#[derive(Resource)]
struct WindowTitle(String);

/// 一个"标题可配置"的插件。
struct WindowTitlePlugin {
    title: String,
}

impl Plugin for WindowTitlePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WindowTitle(self.title.clone()));
        app.add_systems(Update, print_title);
    }
}

/// 每帧打印当前标题。
fn print_title(title: Res<WindowTitle>) {
    println!("window title: {}", title.0);
}

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Settings Demo".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(WindowTitlePlugin {
            title: "My First Game".to_string(),
        })
        .run();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn title_comes_from_plugin_config() {
        let mut app = App::new();
        app.add_plugins(WindowTitlePlugin {
            title: "Space Invaders".to_string(),
        });
        app.update();
        let title = app.world().resource::<WindowTitle>();
        assert_eq!(title.0, "Space Invaders", "标题应该来自插件的配置字段");
    }

    #[test]
    fn different_config_gives_different_title() {
        let mut app = App::new();
        app.add_plugins(WindowTitlePlugin {
            title: "Bevy 0.19".to_string(),
        });
        app.update();
        assert_eq!(
            app.world().resource::<WindowTitle>().0,
            "Bevy 0.19",
            "改配置后标题应该跟着变"
        );
    }
}

// 提示：
// 1. 插件创建时的字段 `title`（比如 "Space Invaders"）应该去哪儿？
// 2. 对比一下 `build` 里 insert_resource 的参数：它用的是 `self.title`
//    还是写死的字符串？
// 3. 修改后运行 `bevylings test 0404`，两个测试都通过就过关了。
