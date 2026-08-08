//! # 练习 33.01 —— Parent / Children：生成层级并查询
//!
//! 出处：https://bevy.org/examples-webgpu/ecs/hierarchy/
//!
//! ## 概念
//! 实体可以组成**父子树**：父实体会带着一个 `Children` 组件，
//! 里面是"孩子的 Entity 列表"。生成层级有两种方式：
//! - 生成父实体时用 `children![...]` 直接把孩子带上；
//! - 先生成，再调用 `commands.entity(父).add_child(子)` 追加。
//!
//! 注意：`for child in children` 遍历时，每次拿到的 `child` 是 `&Entity`（引用），
//! 要解引用成 `Entity`（写 `*child`）才能传给查询去取数据。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 3301` 查看现象，改正后运行 `bevylings test 3301` 让测试通过。
//!
//! 小贴士：`Query<&Children, With<Tag>>` 表示"取所有带 Tag 并且有 Children 的实体"。

// I AM NOT DONE

use bevy::prelude::*;

/// 给实体贴个名字标签。
#[derive(Component)]
struct Tag(&'static str);

/// 统计孩子数量，方便测试观察。
#[derive(Resource, Default)]
struct ChildCount(usize);

fn setup(mut commands: Commands) {
    // 用 children! 一步生成"父 + 两个子"。
    let parent = commands
        .spawn((Tag("root"), children![(Tag("a"),), (Tag("b"),)]))
        .id();
    // 先生成一个孩子，再挂到父实体下面（add_child）。
    let extra = commands.spawn(Tag("extra")).id();
    commands.entity(parent).add_child(extra);
}

/// 数一数父实体一共有多少孩子。
fn count_children(
    parents: Query<&Children, With<Tag>>,
    tags: Query<&Tag>,
    mut report: ResMut<ChildCount>,
) {
    let mut total = 0;
    for children in &parents {
        for child in children {
            // BUG: child 是 &Entity（引用），而 tags.get 需要 Entity（值）。
            // 少写了一个 * 号，编译报"类型不匹配"。
            if tags.get(child).is_ok() {
                total += 1;
            }
        }
    }
    report.0 = total;
}

pub fn run() {
    App::new()
        .init_resource::<ChildCount>()
        .add_systems(Startup, setup)
        .add_systems(Update, count_children)
        .run();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_app() -> App {
        let mut app = App::new();
        app.init_resource::<ChildCount>();
        app.add_systems(Startup, setup);
        app.add_systems(Update, count_children);
        app
    }

    #[test]
    fn counts_all_children() {
        let mut app = build_app();
        app.update();
        let report = app.world().resource::<ChildCount>();
        assert_eq!(report.0, 3, "root 下面应该有 3 个孩子");
    }

    #[test]
    fn children_are_linked_to_parent() {
        let mut app = build_app();
        app.update();
        let mut q = app.world_mut().query::<&Children>();
        let children = q.single(app.world());
        let children = children.unwrap();
        assert_eq!(children.len(), 3, "只有一个父实体，它带着 3 个孩子");
    }
}

// 提示：
// 1. 先运行 `bevylings test 3301`，看编译报错里 child 的类型是什么。
// 2. 遍历 `Children` 拿到的是 `&Entity`，取数据前要写成 `*child`。
// 3. 修改后运行 `bevylings test 3301`，两个测试全绿就过关。
