//! # 练习 03.04 —— Query::get 与 Query::single（按实体精确取值）
//!
//! 出处：https://bevy.org/examples-webgpu/ecs/ecs_guide/
//!
//! ## 概念
//! 除了遍历所有实体，Query 还能"精确取一个"：
//! - `query.get(entity)`：按实体编号取它的组件，实体不存在时返回 `Err`；
//! - `query.single()`：要求"恰好只有一个"匹配实体，多一个少一个都返回 `Err`。
//!
//! 注意：`Entity` 是一个带类型的安全编号，不是普通整数！
//! 不能把它拆成数字当数组下标用。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 0304` 查看现象（目前编译会报错），
//! 改正后运行 `bevylings test 0304` 让测试通过。
//!
//! 小贴士：`get` 的第一个参数必须是完整的 `Entity`，不是 `u32` / `usize`。

use bevy::prelude::*;

/// 分数。
#[derive(Component)]
struct Score(usize);

/// 主角：启动时保存主角实体的编号。
#[derive(Resource)]
struct MainPlayer(Entity);

/// 记录主角分数，方便测试观察。
#[derive(Resource, Default)]
struct ScoreLog(Vec<usize>);

/// 按实体编号读取分数；实体不存在时返回 None。
fn read_score(query: &Query<&Score>, target: Entity) -> Option<usize> {
    query.get(target).ok().map(|s| s.0)
}

/// 每帧读取主角分数并记录，方便观察。
fn report_main_score(mut log: ResMut<ScoreLog>, query: Query<&Score>, main: Res<MainPlayer>) {
    if let Some(score) = read_score(&query, main.0) {
        log.0.push(score);
    }
}

pub fn run() {
    App::new()
        .insert_resource(ScoreLog::default())
        .add_systems(Startup, setup)
        .add_systems(Update, report_main_score)
        .run();
}

/// 生成两个玩家，主角是 Alice（30 分）。
fn setup(mut commands: Commands) {
    let alice = commands.spawn(Score(30)).id();
    commands.spawn(Score(50));
    commands.insert_resource(MainPlayer(alice));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_app() -> App {
        let mut app = App::new();
        app.insert_resource(ScoreLog::default());
        app.add_systems(Startup, setup);
        app.add_systems(Update, report_main_score);
        app
    }

    #[test]
    fn reads_main_player_score() {
        let mut app = build_app();
        app.update();
        let log = app.world().resource::<ScoreLog>();
        assert_eq!(log.0, vec![30], "主角 Alice 的分数应该是 30，实际 {:?}", log.0);
    }

    #[test]
    fn single_errors_when_not_exactly_one() {
        let mut world = World::new();
        world.spawn(Score(1));
        world.spawn(Score(2));
        let mut state = world.query::<&Score>();
        assert!(state.single(&world).is_err(), "有两个实体时 single() 应该返回 Err");

        let mut empty = World::new();
        let mut state = empty.query::<&Score>();
        assert!(state.single(&empty).is_err(), "一个都没有时 single() 也应该返回 Err");
    }
}

// 提示：
// 1. 先运行 `bevylings run 0304`，看编译错误里"expected `Entity`, found `usize`"。
// 2. `Entity` 是安全编号类型，`query.get()` 直接收 `Entity` 本身，不用拆开。
// 3. 删掉 `.index() as usize` 把目标传进去，再运行 `bevylings test 0304`。
