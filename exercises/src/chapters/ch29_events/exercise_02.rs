//! # 练习 29.02 —— 读取消息：MessageReader 与系统顺序
//!
//! 出处：https://bevy.org/examples-webgpu/ecs/message/
//!
//! ## 概念
//! 消息的读写要讲究顺序：`MessageWriter` 先把消息写进"邮箱"，
//! `MessageReader` 才能读到。同一个帧里，如果读者先于写者运行，
//! 它就要等到下一帧才看得到。用 `.chain()` 把系统串起来，
//! 就能保证"先写后读"。注意 `MessageReader` 要声明成 `mut`，
//! 因为它要记录哪些消息已经被读过。
//!
//! ## 任务
//! 下面的代码有一处故意改错的地方（`// BUG:` 注释标出）。
//! 运行 `bevylings run 2902` 观察血量变化，改正后运行
//! `bevylings test 2902` 让测试通过。
//!
//! 小贴士：reader 拿到的是 `&DealDamage`，直接用 `message.amount`
//! 就能访问字段，不需要解引用。

// I AM NOT DONE

use bevy::prelude::*;

/// 一条"造成伤害"的消息
#[derive(Message)]
struct DealDamage {
    amount: i32,
}

/// 玩家的血量
#[derive(Resource)]
struct Health(i32);

/// 每帧发一条 10 点伤害的消息
fn deal_damage(mut writer: MessageWriter<DealDamage>) {
    writer.write(DealDamage { amount: 10 });
}

/// 收到伤害后扣血（只处理正数伤害）
fn apply_damage(
    mut reader: MessageReader<DealDamage>,
    mut health: ResMut<Health>,
) {
    for damage in reader.read() {
        info!("applying {} damage", damage.amount);
        // BUG: 比较符号写反了：正数伤害被无视，负数反而会"扣血"。
        if damage.amount < 0 {
            health.0 -= damage.amount;
        }
    }
}

pub fn run() {
    App::new()
        .insert_resource(Health(100))
        .add_message::<DealDamage>()
        .add_systems(Update, (deal_damage, apply_damage).chain())
        .run();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_app() -> App {
        let mut app = App::new();
        app.insert_resource(Health(100));
        app.add_message::<DealDamage>();
        app.add_systems(Update, (send_damages, apply_damage).chain());
        app
    }

    /// 测试用：发一条 10 点伤害
    fn send_damages(mut writer: MessageWriter<DealDamage>) {
        writer.write(DealDamage { amount: 10 });
    }

    #[test]
    fn positive_damage_reduces_health() {
        let mut app = build_app();
        app.update();
        let health = app.world().resource::<Health>();
        assert_eq!(health.0, 90, "100 - 10 = 90");
    }

    #[test]
    fn negative_amount_is_ignored() {
        let mut app = App::new();
        app.insert_resource(Health(100));
        app.add_message::<DealDamage>();
        app.add_systems(Update, (send_negative, apply_damage).chain());
        app.update();
        let health = app.world().resource::<Health>();
        assert_eq!(health.0, 100, "负数伤害应被忽略");
    }

    /// 测试用：发一条负数伤害
    fn send_negative(mut writer: MessageWriter<DealDamage>) {
        writer.write(DealDamage { amount: -5 });
    }
}

// 提示：
// 1. 先想：`amount` 是正数时，应不应该扣血？
// 2. 问题只有一个符号：比较的方向反了（`<` 应该改成 `>`）。
// 3. 改好后运行 `bevylings test 2902`，两个测试都通过就过关了。
