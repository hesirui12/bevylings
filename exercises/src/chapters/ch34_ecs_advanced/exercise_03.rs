//! # 练习 34.03 —— 并行查询：par_iter_mut 同时迭代
//!
//! 出处：https://bevy.org/examples-webgpu/ecs/parallel-query/
//!
//! ## 概念
//! 普通查询用 `for ... in &query` 一个一个地处理实体；
//! 如果要处理的实体很多，可以用 **并行查询**：
//! `query.par_iter_mut().for_each(|(mut transform, velocity)| ...)`，
//! Bevy 会把实体分批，扔到多线程上同时处理，最后再汇总。
//!
//! 本练习给 4 个精灵设了速度，让它们每帧按速度移动。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 3403` 查看现象，改正后运行 `bevylings test 3403` 让测试通过。
//!
//! 小贴士：位置 = 原位置 + 速度，所以用的是 `+=`；用成 `-=` 就会反向移动。

// I AM NOT DONE

use bevy::prelude::*;

/// 速度（每帧移动多少）。
#[derive(Component, Deref)]
struct Velocity(Vec2);

/// 初始速度，启动时批量生成精灵用。
#[derive(Resource)]
struct StartVelocity(Vec2);

fn setup(mut commands: Commands, velocity: Res<StartVelocity>) {
    for _ in 0..4 {
        commands.spawn((
            Transform::from_xyz(0.0, 0.0, 0.0),
            Velocity(velocity.0),
        ));
    }
}

/// 并行地让每个精灵按自己的速度移动。
fn move_system(mut sprites: Query<(&mut Transform, &Velocity)>) {
    sprites
        .par_iter_mut()
        .for_each(|(mut transform, velocity)| {
            // BUG: 位置应该"加上"速度（+=），这里写成了减号（-=），
            // 所有精灵都会往反方向移动。
            transform.translation -= velocity.extend(0.0);
        });
}

pub fn run() {
    App::new()
        .insert_resource(StartVelocity(Vec2::new(1.0, 0.0)))
        .add_systems(Startup, setup)
        .add_systems(Update, move_system)
        .run();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_app(velocity: Vec2) -> App {
        let mut app = App::new();
        app.insert_resource(StartVelocity(velocity));
        app.add_systems(Startup, setup);
        app.add_systems(Update, move_system);
        app
    }

    #[test]
    fn moves_entities_along_x_by_velocity() {
        let mut app = build_app(Vec2::new(1.0, 0.0));
        app.update();
        let mut q = app.world_mut().query::<&Transform>();
        let xs: Vec<f32> = q.iter(app.world()).map(|t| t.translation.x).collect();
        assert_eq!(xs, vec![1.0, 1.0, 1.0, 1.0], "速度 (1,0) 跑一帧后 x 都是 1");
    }

    #[test]
    fn moves_entities_along_y_by_velocity() {
        let mut app = build_app(Vec2::new(0.0, 2.0));
        app.update();
        let mut q = app.world_mut().query::<&Transform>();
        let ys: Vec<f32> = q.iter(app.world()).map(|t| t.translation.y).collect();
        assert_eq!(ys, vec![2.0, 2.0, 2.0, 2.0], "速度 (0,2) 跑一帧后 y 都是 2");
    }
}

// 提示：
// 1. 先运行 `bevylings test 3403`，看第一个测试期望的位置是多少。
// 2. "移动"是位置加速度：`+=`。
// 3. 改完符号后两个测试都绿就过关。
