//! # 练习 28.04 —— 生成资产：add 与 reserve + insert
//!
//! 出处：https://bevy.org/examples-webgpu/asset/generated-assets/
//!
//! ## 概念
//! 资产不一定要从文件加载，也可以在运行时"生成"：
//! - `Assets::add(asset)`：把资产放进集合，立刻返回一个可用 handle；
//! - `reserve_handle() + insert(&handle, asset)`：先占一个 handle，
//!   之后随时用 `Assets::insert` 把资产填进去（适合由系统异步生成）。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 2804` 查看现象，改正后运行 `bevylings test 2804` 让测试通过。
//!
//! 小贴士：`reserve_handle` 拿到的 handle 是"空的"，必须用同一个 handle 调用 `insert` 才有数据。

// I AM NOT DONE

use bevy::prelude::*;

/// 记录"稍后要填充"的网格 handle。
#[derive(Resource)]
struct HandleToGenerate(Handle<Mesh>);

/// 运行时生成一个环面网格，塞进预先保留的 handle。
fn generate_mesh_system(
    handle_to_generate: Res<HandleToGenerate>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let mesh = Mesh::from(Torus::new(0.8, 1.2));
    // BUG: 这里用 add() 另起了一个新 handle，实体上持有的还是
    // 之前 reserve 的那个 handle，两者对不上，网格永远取不到。
    let _new_handle = meshes.add(mesh);
}

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, generate_mesh_system.run_if(run_once))
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    meshes: Res<Assets<Mesh>>,
) {
    commands.spawn((Camera3d::default(), Transform::from_xyz(0.0, 0.0, 5.0)));
    commands.spawn((DirectionalLight::default(), Transform::default()));

    let material_handle = materials.add(StandardMaterial::default());

    // 方式一：add —— 由异步任务生成一个圆锥
    commands.spawn((
        Transform::from_xyz(-2.0, 0.0, 0.0),
        MeshMaterial3d(material_handle.clone()),
        Mesh3d(asset_server.add_async(generate_mesh_async())),
    ));

    // 方式二：reserve + insert —— 先占位，稍后由系统填充
    let mesh_handle = meshes.reserve_handle();
    commands.insert_resource(HandleToGenerate(mesh_handle.clone()));
    commands.spawn((
        Transform::from_xyz(2.0, 0.0, 0.0),
        Mesh3d(mesh_handle),
        MeshMaterial3d(material_handle),
    ));
}

async fn generate_mesh_async() -> Result<Mesh, std::io::Error> {
    Ok(Mesh::from(Cone::new(1.0, 2.0)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reserved_handle_gets_populated() {
        let mut app = App::new();
        let meshes = Assets::<Mesh>::default();
        let handle = meshes.reserve_handle();
        app.insert_resource(meshes);
        app.insert_resource(HandleToGenerate(handle.clone()));
        app.add_systems(Update, generate_mesh_system);
        app.update();

        let meshes = app.world().resource::<Assets<Mesh>>();
        assert!(
            meshes.get(&handle).is_some(),
            "系统应该把网格塞进 reserve 出来的 handle"
        );
    }

    #[test]
    fn add_returns_working_handle() {
        let mut meshes = Assets::<Mesh>::default();
        let handle = meshes.add(Mesh::from(Cone::new(1.0, 2.0)));
        assert!(meshes.get(&handle).is_some(), "add 返回的 handle 立刻可用");
    }
}

// 提示：
// 1. `reserve_handle` 只占号、不放数据；`add` 才是"放数据并给新号"。
// 2. 想往"已有的 handle"里放数据，要用 `Assets::insert(&handle, asset)`。
// 3. 修改后运行 `bevylings test 2804`，两个测试全绿就过关了。
