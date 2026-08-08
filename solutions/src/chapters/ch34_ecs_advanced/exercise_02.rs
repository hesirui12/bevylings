//! # 练习 34.02 —— 泛型系统：一套逻辑服务多个组件
//!
//! 出处：https://bevy.org/examples-webgpu/ecs/generic-system/
//!
//! ## 概念
//! 有时好几类实体"要做的事一模一样"（比如菜单要清理、关卡要清理，
//! 都是"把带某标签的实体删掉"）。与其复制粘贴系统，
//! 不如写一个**泛型系统**：`fn cleanup<T>(...)`，然后用 `::<T>`（turbofish）
//! 指定具体类型：`cleanup::<MenuClose>` 只清理带 `MenuClose` 的实体。
//!
//! 关键：泛型参数 `T` 必须加上 `T: Component` 约束，
//! 告诉编译器"T 是可以当组件用的类型"，否则 `With<T>` 编译不过。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 3402` 查看现象，改正后运行 `bevylings test 3402` 让测试通过。
//!
//! 小贴士：turbofish 语法是 `cleanup::<MenuClose>`，写在函数名后面。

use bevy::prelude::*;

/// 标记：属于"菜单"的实体。
#[derive(Component)]
struct MenuClose;

/// 标记：属于"关卡"的实体。
#[derive(Component)]
struct LevelUnload;

/// 记录还剩多少实体，方便测试观察。
#[derive(Resource, Default)]
struct AliveCount(usize);

fn setup(mut commands: Commands) {
    commands.spawn(MenuClose);
    commands.spawn(LevelUnload);
}

/// 泛型清理系统：删掉所有带 T 组件的实体。
fn cleanup<T: Component>(mut commands: Commands, query: Query<Entity, With<T>>) {
    for e in &query {
        commands.entity(e).despawn();
    }
}

/// 数一数世界上还剩多少实体。
fn count_alive(
    mut report: ResMut<AliveCount>,
    query: Query<Entity, Or<(With<MenuClose>, With<LevelUnload>)>>,
) {
    report.0 = query.iter().count();
}

pub fn run() {
    App::new()
        .init_resource::<AliveCount>()
        .add_systems(Startup, setup)
        .add_systems(Update, (cleanup::<MenuClose>, count_alive).chain())
        .run();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_app() -> App {
        let mut app = App::new();
        app.init_resource::<AliveCount>();
        app.add_systems(Startup, setup);
        app
    }

    #[test]
    fn cleanup_menu_keeps_level_entities() {
        let mut app = build_app();
        app.add_systems(Update, (cleanup::<MenuClose>, count_alive).chain());
        app.update();
        let alive = app.world().resource::<AliveCount>();
        assert_eq!(alive.0, 1, "只清菜单，关卡实体应该还留着");
    }

    #[test]
    fn cleanup_both_empties_the_world() {
        let mut app = build_app();
        app.add_systems(
            Update,
            (cleanup::<MenuClose>, cleanup::<LevelUnload>, count_alive).chain(),
        );
        app.update();
        let alive = app.world().resource::<AliveCount>();
        assert_eq!(alive.0, 0, "两类实体都被清理后，世界上啥也不剩");
    }
}

// 提示：
// 1. 先运行 `bevylings test 3402`，看编译报错里缺了什么约束。
// 2. `With<T>` 要求 `T: Component`，泛型参数要带上这个约束。
// 3. 改完后再运行一次测试，两个测试都绿就过关。
