//! # 练习 17.03 —— 计算状态：ComputedStates
//!
//! 出处：https://bevy.org/examples-webgpu/state/computed_states/
//!
//! ## 概念
//! 有时候"当前在哪"可以从其他状态**算出来**。比如 AppState 里存着
//! 一堆细分状态，而"是否在游戏中"由它们共同决定。
//! `ComputedStates` 就是这样一种自动推导的状态：
//! - `type SourceStates = AppState;` 声明它依赖哪个状态；
//! - `fn compute(sources: AppState) -> Option<Self>`：返回 `Some` 时
//!   该状态存在，返回 `None` 时不存在；
//! - `app.add_computed_state::<InGame>()` 注册。
//!
//! 本练习：AppState 处于 `InGame`（无论是否暂停）时，
//! 计算状态 `InGame` 就存在，并触发 `OnEnter(InGame)`。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 1703` 查看现象，改正后运行 `bevylings test 1703` 让测试通过。
//!
//! 小贴士：`compute` 返回 `None` 表示"这个计算状态当前不成立"。

// I AM NOT DONE

use bevy::prelude::*;

/// 根状态：菜单 / 游戏中（可暂停）。
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
enum AppState {
    #[default]
    Menu,
    InGame { paused: bool },
}

/// 计算状态：只要在游戏里（不管是否暂停），它就存在。
// BUG: InGame 只 derive 了 Debug 和 Clone，缺少 PartialEq、Eq、Hash，
// 不满足 ComputedStates 的要求，add_computed_state 编译不过。
#[derive(Debug, Clone, Copy)]
struct InGame;

impl ComputedStates for InGame {
    type SourceStates = AppState;

    fn compute(sources: AppState) -> Option<Self> {
        match sources {
            AppState::InGame { .. } => Some(InGame),
            _ => None,
        }
    }
}

/// 进入游戏（计算状态成立）时调用。
fn enter_game(mut commands: Commands) {
    commands.insert_resource(InGameSetup(true));
}

/// 记录"进入游戏"是否发生过，方便测试观察。
#[derive(Resource, Default)]
struct InGameSetup(bool);

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<AppState>()
        .add_computed_state::<InGame>()
        .add_systems(OnEnter(InGame), enter_game)
        .run();
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::state::app::StatesPlugin;

    #[test]
    fn compute_detects_in_game() {
        assert_eq!(InGame::compute(AppState::Menu), None);
        assert_eq!(
            InGame::compute(AppState::InGame { paused: false }),
            Some(InGame)
        );
        assert_eq!(
            InGame::compute(AppState::InGame { paused: true }),
            Some(InGame)
        );
    }

    #[test]
    fn on_enter_runs_when_computed_state_appears() {
        let mut app = App::new();
        app.add_plugins(StatesPlugin);
        app.init_state::<AppState>();
        app.add_computed_state::<InGame>();
        app.add_systems(OnEnter(InGame), enter_game);
        app.update(); // 菜单：计算状态不存在
        assert!(!app.world().contains_resource::<InGameSetup>());
        app.world_mut()
            .resource_mut::<NextState<AppState>>()
            .set(AppState::InGame { paused: false });
        app.update(); // 进入游戏：OnEnter(InGame) 触发
        assert!(app.world().contains_resource::<InGameSetup>());
    }
}

// 提示：
// 1. 先运行 `bevylings run 1703`，编译器会提示 InGame 不满足 ComputedStates。
// 2. ComputedStates 要求类型能比较相等、能取哈希、能克隆、能调试打印。
// 3. 补上对应的 derive 后运行 `bevylings test 1703`，两个测试全绿就过关。
