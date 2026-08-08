//! # 练习 03.03 —— Query：& 与 &mut 的冲突（先读后写）
//!
//! 出处：https://bevy.org/examples-webgpu/ecs/ecs_guide/
//!
//! ## 概念
//! 查询组件可以要只读引用 `Query<&T>`，也可以要可变引用 `Query<&mut T>`。
//! 遍历方式也有讲究：用 `&query` 遍历拿到的是**只读**数据（改不了），
//! 用 `&mut query` 遍历拿到的才是**可变**数据。
//! 如果既想读又想写同一个组件，正确做法是"先读完、再开始写"
//! —— 借用不能同时冲突。
//!
//! 本练习把每个实体的数值归一化：先算出总和，再把每个数除以总和。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 0303` 查看现象（目前编译会报错），
//! 改正后运行 `bevylings test 0303` 让测试通过。
//!
//! 小贴士：读总和那一步用 `query.iter()` 没问题；关键是写的那一步必须 `&mut query`。

// I AM NOT DONE

use bevy::prelude::*;

/// 一个数值。
#[derive(Component)]
struct Value(f32);

/// 把每个 Value 除以总和，让所有值的和为 1（归一化）。
fn normalize(mut query: Query<&mut Value>) {
    let sum: f32 = query.iter().map(|v| v.0).sum();
    for mut v in &query {
        // BUG: 这里用 `&query` 遍历，拿到的只是只读引用。
        // 编译器不允许给只读引用赋值，所以 `v.0 /= sum` 会报错。
        // 想修改组件数据，必须换成 `&mut query` 遍历。
        v.0 /= sum;
    }
}

pub fn run() {
    App::new()
        .add_systems(Startup, setup)
        .add_systems(Update, normalize)
        .run();
}

/// 生成两个数值不同的实体。
fn setup(mut commands: Commands) {
    commands.spawn(Value(1.0));
    commands.spawn(Value(3.0));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn values_sum_to_one_after_normalize() {
        let mut app = App::new();
        app.world_mut().spawn(Value(1.0));
        app.world_mut().spawn(Value(3.0));
        app.add_systems(Update, normalize);
        app.update();

        let mut q = app.world_mut().query::<&Value>();
        let mut vals: Vec<f32> = q.iter(app.world()).map(|v| v.0).collect();
        vals.sort_by(|a, b| a.partial_cmp(b).unwrap());
        assert!(
            (vals[0] - 0.25).abs() < 1e-5,
            "1.0 归一化后应该是 0.25，实际 {}",
            vals[0]
        );
        assert!(
            (vals[1] - 0.75).abs() < 1e-5,
            "3.0 归一化后应该是 0.75，实际 {}",
            vals[1]
        );
    }

    #[test]
    fn single_value_becomes_one() {
        let mut app = App::new();
        app.world_mut().spawn(Value(7.0));
        app.add_systems(Update, normalize);
        app.update();

        let mut q = app.world_mut().query::<&Value>();
        let v = q.single(app.world()).unwrap();
        assert!(
            (v.0 - 1.0).abs() < 1e-5,
            "只有一个值时它自己就是全部，归一化后应为 1.0，实际 {}",
            v.0
        );
    }
}

// 提示：
// 1. 先运行 `bevylings run 0303`，仔细读编译错误里"cannot assign ... behind a `&` reference"。
// 2. `&query` 与 `&mut query`：前者只能读，后者才能写。
// 3. 改成 `&mut query` 后运行 `bevylings test 0303`，测试全绿就过关了。
