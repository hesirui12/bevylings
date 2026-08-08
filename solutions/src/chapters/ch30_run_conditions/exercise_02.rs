//! # 练习 30.02 —— 条件组合：and / or / not
//!
//! 出处：https://bevy.org/examples-webgpu/ecs/run-conditions/
//!
//! ## 概念
//! 一个系统可以挂多个条件，也可以把条件**组合**起来：
//! - `.and_then(另一个条件)`：两个都为 `true` 才运行（类似 `&&`）。
//! - `.or_else(另一个条件)`：至少一个为 `true` 就运行（类似 `||`）。
//! - `not(条件)`：把条件取反（`true` 变 `false`）。
//!
//! 本练习模拟一个角色升级：需要"有体力"**并且**"还没满级"才能升级；
//! 没体力时每帧自动恢复一点。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 3002` 查看现象，改正后运行 `bevylings test 3002` 让测试通过。
//!
//! 小贴士：`and_then` 和 `or_else` 都会"短路"：前一个条件已经能决定结果时，
//! 就不再计算后一个条件了。

use bevy::prelude::*;

/// 体力值（消耗后靠恢复补回来）。
#[derive(Resource, Default)]
struct Energy(i32);

/// 等级。
#[derive(Resource, Default)]
struct Level(u32);

/// 记录系统干过的事，方便测试观察。
#[derive(Resource, Default)]
struct GameLog(Vec<&'static str>);

/// 条件：还有体力吗？
fn has_energy(energy: Res<Energy>) -> bool {
    energy.0 > 0
}

/// 条件：还没满级（等级 < 3）吗？
fn below_max(level: Res<Level>) -> bool {
    level.0 < 3
}

/// 升级：等级 +1，消耗 1 点体力。
fn level_up(mut level: ResMut<Level>, mut energy: ResMut<Energy>, mut log: ResMut<GameLog>) {
    level.0 += 1;
    energy.0 -= 1;
    log.0.push("level up");
}

/// 没体力时每帧恢复 1 点体力。
fn recover(mut energy: ResMut<Energy>) {
    energy.0 += 1;
}

/// 把系统接上组合条件。
fn wire_systems(app: &mut App) {
    // 注意顺序：recover 排在 level_up 后面（chain），
    // 否则"恢复体力"和"消耗体力"在同一帧互相打架。
    app.add_systems(
        Update,
        (
            level_up.run_if(has_energy.and_then(below_max)),
            recover.run_if(not(has_energy)),
        )
            .chain(),
    );
}

pub fn run() {
    let mut app = App::new();
    app.init_resource::<Energy>();
    app.init_resource::<Level>();
    app.init_resource::<GameLog>();
    wire_systems(&mut app);
    app.run();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_app(energy: i32, level: u32) -> App {
        let mut app = App::new();
        app.insert_resource(Energy(energy));
        app.insert_resource(Level(level));
        app.init_resource::<GameLog>();
        wire_systems(&mut app);
        app
    }

    #[test]
    fn requires_both_conditions() {
        let mut app = build_app(0, 1); // 没体力，但还没满级
        app.update();
        let world = app.world();
        assert_eq!(world.resource::<Level>().0, 1, "没体力不能升级");
        assert!(
            world.resource::<GameLog>().0.is_empty(),
            "升级日志应该是空的"
        );
    }

    #[test]
    fn both_conditions_true_level_up() {
        let mut app = build_app(5, 1);
        app.update();
        let world = app.world();
        assert_eq!(world.resource::<Level>().0, 2, "有体力且未满级，应该升级");
        assert_eq!(world.resource::<Energy>().0, 4, "升级消耗 1 点体力");
    }

    #[test]
    fn max_level_blocks_level_up() {
        let mut app = build_app(5, 3); // 满级
        app.update();
        assert_eq!(app.world().resource::<Level>().0, 3, "满级后不能再升级");
    }

    #[test]
    fn recovers_when_out_of_energy() {
        let mut app = build_app(0, 2);
        app.update();
        assert_eq!(app.world().resource::<Energy>().0, 1, "没体力时自动恢复 1 点");
    }
}

// 提示：
// 1. 先运行 `bevylings test 3002`，看"没体力却升级了"的测试怎么失败。
// 2. `and_then` = 而且（AND），`or_else` = 或者（OR），别用反。
// 3. `not(条件)` 表示"条件为假"时才运行，这里用来表达"没体力"。
