//! # 练习 13.03 —— 加载屏：进度与状态切换
//!
//! 出处：https://bevy.org/examples/games/loading-screen/
//!
//! ## 概念
//! 大场景要加载一堆资源（模型、贴图），需要先显示"加载中"。
//! 官方示例用 `LoadingState` 表示当前状态（`LevelLoading` / `LevelReady`），
//! 再用 `LoadingData` 记录还剩多少资源没加载完。
//! 简化版里我们用 `remaining` 这个数字代替真实的资源句柄：
//! 每帧少一个，减到 0 就认为全部加载完成、可以进游戏了。
//!
//! ## 任务
//! 运行 `bevylings test 1303` 让测试通过。
//!
//! 小贴士：`ResMut<T>` 是资源的可变句柄，改内部值要写成 `*资源 = 新值`。

use bevy::prelude::*;

/// 加载状态：加载中 / 已就绪。
#[derive(Resource, Default, PartialEq, Eq, Debug)]
enum LoadingState {
    #[default]
    LevelReady,
    LevelLoading,
}

/// 还剩多少"资源"没加载完（简化：用数字代替真实资源）。
#[derive(Resource, Debug)]
struct LoadingData {
    remaining: usize,
}

impl LoadingData {
    fn new(remaining: usize) -> Self {
        Self { remaining }
    }
}

/// 每帧更新加载进度：还有剩余就继续加载，全加载完就就绪。
fn update_loading(
    mut loading_state: ResMut<LoadingState>,
    mut loading_data: ResMut<LoadingData>,
) {
    if loading_data.remaining > 0 {
        loading_data.remaining -= 1;
        *loading_state = LoadingState::LevelLoading;
    } else {
        *loading_state = LoadingState::LevelReady;
    }
}

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(LoadingState::default())
        .insert_resource(LoadingData::new(3))
        .add_systems(Startup, setup)
        .add_systems(Update, update_loading)
        .run();
}

/// 生成相机。
fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finishes_loading_when_nothing_remains() {
        let mut app = App::new();
        app.insert_resource(LoadingState::default());
        app.insert_resource(LoadingData::new(2));
        app.add_systems(Update, update_loading);
        app.update(); // 剩 1
        app.update(); // 剩 0
        app.update(); // 全部完成
        let state = app.world().resource::<LoadingState>();
        assert_eq!(*state, LoadingState::LevelReady, "加载完应进入就绪状态");
    }

    #[test]
    fn still_loading_while_assets_remain() {
        let mut app = App::new();
        app.insert_resource(LoadingState::default());
        app.insert_resource(LoadingData::new(3));
        app.add_systems(Update, update_loading);
        app.update();
        let state = app.world().resource::<LoadingState>();
        assert_eq!(*state, LoadingState::LevelLoading, "还有资源时应显示加载中");
        let data = app.world().resource::<LoadingData>();
        assert_eq!(data.remaining, 2, "每帧应少一个待加载项");
    }
}

// 提示：
// 1. `ResMut<T>` 只是句柄，想改它包着的值必须先 `*` 解引用。
// 2. 修改后运行 `bevylings test 1303`，两个测试全绿就过关。
