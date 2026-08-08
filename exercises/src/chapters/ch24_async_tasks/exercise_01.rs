//! # 练习 24.01 —— 异步计算：任务池与任务轮询
//!
//! 出处：https://bevy.org/examples-webgpu/async_tasks/async_compute/
//!
//! ## 概念
//! 有些计算很耗时（寻路、物理模拟等），如果都堆在主线程里做，
//! 游戏就会一卡一卡。Bevy 的 `AsyncComputeTaskPool`（异步计算任务池）
//! 提供了一批后台线程：用 `spawn` 把一个"未来（future）"丢进去，
//! 它就在后台线程上执行，主线程完全不用等它。
//!
//! `spawn` 会返回一个 `Task<T>`——它像一张"取货单"。每帧我们都可以
//! 用 `check_ready` 问一次"货到了没有"，到了就把结果取走。
//! 我们把 `Task` 当成一个组件挂在实体上，方便逐帧跟踪。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 2401` 观察现象，改正后运行 `bevylings test 2401` 让测试通过。
//!
//! 小贴士：`check_ready` 要求传入**可变引用**，因为它要"催一催"
//! 任务内部的状态（轮询本质上会修改任务）。

// I AM NOT DONE

use bevy::{
    prelude::*,
    tasks::{futures::check_ready, AsyncComputeTaskPool, Task},
};

/// 记录后台任务算完的结果（这里假设任务算出的一个新位置）。
#[derive(Resource, Default)]
struct ComputedTransforms(Vec<Transform>);

/// 挂在实体上、尚未完成的异步任务。
#[derive(Component)]
struct ComputeTransform(Task<Transform>);

/// 启动时给 3 个实体各派一个后台任务：计算一个位置。
fn spawn_tasks(mut commands: Commands) {
    let pool = AsyncComputeTaskPool::get();
    for x in 0..3 {
        let entity = commands.spawn_empty().id();
        let task = pool.spawn(async move {
            // 这里假装做了很耗时的计算，最后返回一个位置。
            Transform::from_xyz(x as f32, 0.0, 0.0)
        });
        commands.entity(entity).insert(ComputeTransform(task));
    }
}

/// 每帧检查任务完成没有：完成了就把结果收进资源，并摘掉任务组件。
fn handle_tasks(
    mut commands: Commands,
    mut computed: ResMut<ComputedTransforms>,
    mut tasks: Query<(Entity, &mut ComputeTransform)>,
) {
    for (entity, mut task) in &mut tasks {
        // BUG: check_ready 需要的是可变引用（它要轮询任务内部状态），
        // 这里却传了不可变引用，编译会报错。
        if let Some(transform) = check_ready(&task.0) {
            computed.0.push(transform);
            commands.entity(entity).remove::<ComputeTransform>();
        }
    }
}

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ComputedTransforms::default())
        .add_systems(Startup, spawn_tasks)
        .add_systems(Update, handle_tasks)
        .run();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spawn_creates_three_task_entities() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_systems(Startup, spawn_tasks);
        app.update(); // 第一次 update 会执行 Startup
        let mut query = app.world_mut().query::<&ComputeTransform>();
        assert_eq!(
            query.iter(app.world()).count(),
            3,
            "应该创建 3 个挂着任务组件的实体"
        );
    }

    #[test]
    fn finished_tasks_are_collected() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.insert_resource(ComputedTransforms::default());
        app.add_systems(Startup, spawn_tasks);
        app.add_systems(Update, handle_tasks);
        // 后台任务很快完成；多跑几帧直到 3 个结果全部收齐（上限防死循环）。
        for _ in 0..200 {
            app.update();
            if app.world().resource::<ComputedTransforms>().0.len() >= 3 {
                break;
            }
        }
        let computed = app.world().resource::<ComputedTransforms>();
        assert_eq!(computed.0.len(), 3, "3 个任务的结果都应被收集");
        assert!(
            computed.0.iter().any(|t| t.translation.x == 1.0),
            "x = 1 的那个任务结果应该在"
        );
    }
}

// 提示：
// 1. 先运行 `bevylings run 2401`，看编译错误指向哪一行。
// 2. `check_ready(future: &mut F)` 的参数是 `&mut`——轮询会改变任务的状态。
// 3. 改好后运行 `bevylings test 2401`，两个测试全绿就过关了。
