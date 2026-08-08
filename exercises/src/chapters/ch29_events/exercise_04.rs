//! # 练习 29.04 —— 组件生命周期：On<Add<T>> 与 On<Remove<T>>
//!
//! 出处：https://bevy.org/examples-webgpu/ecs/observers/
//!
//! ## 概念
//! 除了自定义事件，观察者还能监听**组件生命周期**：
//! 组件被加到实体上时触发 `On<Add, 组件>`，被移除（包括实体被销毁）时
//! 触发 `On<Remove, 组件>`。这很适合维护"索引"之类的辅助数据——
//! 不用每帧扫描全世界的实体，加一个、记一个，删一个、减一个。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 2904` 观察计数，改正后运行
//! `bevylings test 2904` 让测试通过。
//!
//! 小贴士：生命周期事件的参数写法是 `On<Add, Mine>`（第一个是事件，
//! 第二个是组件），通过 `event.entity` 能拿到触发事件的实体。

// I AM NOT DONE

use bevy::prelude::*;

/// 一颗地雷
#[derive(Component)]
struct Mine;

/// 当前地雷数量
#[derive(Resource, Default)]
struct MineCount(i32);

/// 地雷被添加时：计数 +1
fn on_add_mine(_add: On<Add, Mine>, mut count: ResMut<MineCount>) {
    count.0 += 1;
    info!("mine count: {}", count.0);
}

/// 地雷被移除时：计数 -1
fn on_remove_mine(_remove: On<Add, Mine>, mut count: ResMut<MineCount>) {
    // BUG: 监听的事件写错了：本意是监听"移除"（Remove），却写成了
    // "添加"（Add），导致移除地雷时计数不会减少，反而在添加时多减一次。
    count.0 -= 1;
    info!("mine count: {}", count.0);
}

pub fn run() {
    App::new()
        .insert_resource(MineCount::default())
        .add_observer(on_add_mine)
        .add_observer(on_remove_mine)
        .add_systems(Startup, |mut commands: Commands| {
            commands.spawn((Mine,));
        })
        .run();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spawning_mines_increases_count() {
        let mut app = App::new();
        app.insert_resource(MineCount::default());
        app.add_observer(on_add_mine);
        app.add_observer(on_remove_mine);
        app.world_mut().spawn((Mine,));
        app.world_mut().spawn((Mine,));
        let count = app.world().resource::<MineCount>();
        assert_eq!(count.0, 2, "生成 2 颗地雷，数量应为 2");
    }

    #[test]
    fn despawning_mines_decreases_count() {
        let mut app = App::new();
        app.insert_resource(MineCount::default());
        app.add_observer(on_add_mine);
        app.add_observer(on_remove_mine);
        let mine = app.world_mut().spawn((Mine,)).id();
        app.world_mut().spawn((Mine,));
        assert_eq!(app.world().resource::<MineCount>().0, 2);
        app.world_mut().despawn(mine);
        let count = app.world().resource::<MineCount>();
        assert_eq!(count.0, 1, "移除 1 颗后，数量应为 1");
    }
}

// 提示：
// 1. 注意 `on_remove_mine` 的第一个参数：它监听的是"添加"还是"移除"？
// 2. 生命周期事件用 `On<Add, 组件>` / `On<Remove, 组件>` 两种写法。
// 3. 改好后运行 `bevylings test 2904`，两个测试都通过就过关了。
