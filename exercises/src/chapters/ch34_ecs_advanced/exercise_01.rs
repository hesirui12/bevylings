//! # 练习 34.01 —— 一次性系统：OneShotSystem
//!
//! 出处：https://bevy.org/examples-webgpu/ecs/one-shot-systems/
//!
//! ## 概念
//! 普通系统每帧都被调度器检查，但如果某个逻辑**很少触发**（比如按下某键才执行），
//! 可以先把它"注册"成**一次性系统**：注册后拿到一个 `SystemId`，
//! 想让它跑的时候用 `commands.run_system(id)` 手动触发，平时完全不占调度。
//!
//! 流程三步走：
//! 1. `commands.register_system(函数)` 注册，得到 `SystemId`；
//! 2. 把 `SystemId` 存进资源或组件里；
//! 3. 需要时 `commands.run_system(那个 id)` 触发它。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 3401` 查看现象，改正后运行 `bevylings test 3401` 让测试通过。
//!
//! 小贴士：`run_system` 要的是**注册后拿到的 id**，不是函数本身。

// I AM NOT DONE

use bevy::{ecs::system::SystemId, prelude::*};

/// 分数。
#[derive(Resource, Default)]
struct Score(u32);

/// 是否触发一次性系统。
#[derive(Resource, Default)]
struct TriggerFlag(bool);

/// 保存注册好的一次性系统的 id。
#[derive(Resource)]
struct CallbackId(SystemId);

/// 启动时注册一次性系统，把 id 存起来。
fn setup(mut commands: Commands) {
    let id = commands.register_system(grant_bonus);
    commands.insert_resource(CallbackId(id));
}

/// 一次性系统：给分数加 100（平时不被调度，只在被触发时跑）。
fn grant_bonus(mut score: ResMut<Score>) {
    score.0 += 100;
}

/// 标志为真时，触发注册好的一次性系统。
fn trigger(mut commands: Commands, flag: Res<TriggerFlag>, id: Res<CallbackId>) {
    if flag.0 {
        // BUG: run_system 需要的是注册时拿到的 SystemId（存在 id.0 里），
        // 这里却把函数本身传了进去，类型对不上，编译失败。
        commands.run_system(grant_bonus);
    }
}

pub fn run() {
    App::new()
        .init_resource::<Score>()
        .insert_resource(TriggerFlag(true))
        .add_systems(Startup, setup)
        .add_systems(Update, trigger)
        .run();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_app(flag: bool) -> App {
        let mut app = App::new();
        app.init_resource::<Score>();
        app.insert_resource(TriggerFlag(flag));
        app.add_systems(Startup, setup);
        app.add_systems(Update, trigger);
        app
    }

    #[test]
    fn one_shot_system_grants_bonus_when_triggered() {
        let mut app = build_app(true);
        app.update();
        let score = app.world().resource::<Score>();
        assert_eq!(score.0, 100, "触发了一次，应该加上 100 分");
    }

    #[test]
    fn no_bonus_without_trigger() {
        let mut app = build_app(false);
        app.update();
        let score = app.world().resource::<Score>();
        assert_eq!(score.0, 0, "没触发就不该加分");
    }
}

// 提示：
// 1. 先运行 `bevylings test 3401`，看编译报错提示的类型是什么。
// 2. `setup` 里已经用 `commands.register_system(grant_bonus)` 拿到了 id。
// 3. 触发时应该写 `commands.run_system(id.0)`。
