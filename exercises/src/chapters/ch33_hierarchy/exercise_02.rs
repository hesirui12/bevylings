//! # 练习 33.02 —— 父子变换：子随父动
//!
//! 出处：https://bevy.org/examples-webgpu/3d/parenting/
//!
//! ## 概念
//! 父实体一旦移动、旋转，所有孩子都会跟着动——这就是层级变换传播。
//! 孩子的**世界位置** = 父的位置 + 孩子相对父的偏移。
//! 官方示例用 `children![...]` 在生成父实体时顺带生成孩子；
//! 本练习用 `add_child` 演示"先有实体、后建立关系"。
//!
//! 注意：Bevy 会自己算全局变换，这里我们手动写一个纯函数
//! `child_global_position` 来模拟这个加法，方便单元测试。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 3302` 查看现象，改正后运行 `bevylings test 3302` 让测试通过。
//!
//! 小贴士：父实体在 (1, 2, 0)，孩子的相对偏移是 (0, 0, 3)，
//! 那么孩子在世界空间应该在 (1, 2, 3)。

// I AM NOT DONE

use bevy::prelude::*;

/// 记录算出来的孩子世界位置，方便测试观察。
#[derive(Resource, Default)]
struct Positions(Vec<Vec3>);

/// 计算孩子在世界空间中的位置：父位置 + 相对偏移。
fn child_global_position(parent: Vec3, offset: Vec3) -> Vec3 {
    // BUG: 偏移方向算反了 —— 孩子应该在父位置的基础上"加上"偏移，
    // 这里却用了减号，孩子会跑到反方向去。
    parent - offset
}

fn setup(mut commands: Commands) {
    // 父实体。
    let parent = commands.spawn(Transform::from_xyz(1.0, 2.0, 0.0)).id();
    // 孩子的偏移：相对父实体往 z 方向 3 格。
    let child = commands.spawn(Transform::from_xyz(0.0, 0.0, 3.0)).id();
    // 建立父子关系。
    commands.entity(parent).add_child(child);
}

/// 用局部变换手工算出每个孩子的世界位置（效果等同 Bevy 的变换传播）。
fn record_positions(
    parents: Query<(&Transform, &Children)>,
    transforms: Query<&Transform>,
    mut out: ResMut<Positions>,
) {
    for (parent_transform, children) in &parents {
        for child in children {
            if let Ok(child_transform) = transforms.get(*child) {
                out.0.push(child_global_position(
                    parent_transform.translation,
                    child_transform.translation,
                ));
            }
        }
    }
}

pub fn run() {
    App::new()
        .init_resource::<Positions>()
        .add_systems(Startup, setup)
        .add_systems(Update, record_positions)
        .run();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_app() -> App {
        let mut app = App::new();
        app.init_resource::<Positions>();
        app.add_systems(Startup, setup);
        app.add_systems(Update, record_positions);
        app
    }

    #[test]
    fn child_position_is_parent_plus_offset() {
        assert_eq!(
            child_global_position(Vec3::new(1.0, 2.0, 0.0), Vec3::new(0.0, 0.0, 3.0)),
            Vec3::new(1.0, 2.0, 3.0),
            "世界位置 = 父位置 + 相对偏移"
        );
    }

    #[test]
    fn recorded_child_position_matches() {
        let mut app = build_app();
        app.update();
        let positions = &app.world().resource::<Positions>().0;
        assert_eq!(
            positions,
            &vec![Vec3::new(1.0, 2.0, 3.0)],
            "父 (1,2,0) + 偏移 (0,0,3) 应该得到 (1,2,3)"
        );
    }
}

// 提示：
// 1. 先运行 `bevylings test 3302`，看第二个测试期望的坐标是什么。
// 2. 偏移是"相对父的位置"，世界位置要把父位置**加**上去。
// 3. 改完 `child_global_position` 后两个测试就都绿了。
