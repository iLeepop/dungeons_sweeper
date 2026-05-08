//! HUD 与玩家实体状态的绑定：从组件读取并刷新 UI，避免 Observer 与 PostUpdate 效果顺序导致的显示滞后。

use bevy::prelude::*;

use crate::components::{Damage, Defense, Gem, GoldCoin, Health, Player};
use crate::effects::PLAYER_HP_MAX;
use crate::ui::plugins::hud::{HPBar, HPProgressBar};

// ---------------------------------------------------------------------------
// 每帧 late PostUpdate：在效果调度之后执行（由 DungeonsPlugin 中 `.chain()` 保证顺序）
// ---------------------------------------------------------------------------

/// 从玩家实体上的 [`Health`] / [`Defense`] / [`Damage`] / [`GoldCoin`] / [`Gem`] 同步 HUD 文本与血条宽度。
///
/// 不依赖事件或 Message，因此草地回血（效果系统）与受伤（`PlayerHurt`）以任意顺序改组件后，
/// 同一帧末尾仍会显示一致数值。
pub fn sync_player_hud_from_components(
    player: Single<Entity, With<Player>>,
    status: Query<
        (
            Option<&Health>,
            Option<&Defense>,
            Option<&Damage>,
            Option<&GoldCoin>,
            Option<&Gem>,
        ),
        With<Player>,
    >,
    mut hp_line: Single<&mut Text, With<HPBar>>,
    mut prog_transform: Query<&mut Transform, With<HPProgressBar>>,
) {
    let Ok((health, defense, damage, gold, gem)) = status.get(*player) else {
        return;
    };

    let hp = health.map(|h| h.0).unwrap_or(0);
    let def = defense.map(|d| d.0).unwrap_or(0);
    let atk = damage.map(|d| d.0).unwrap_or(0);
    let coins = gold.map(|g| g.0).unwrap_or(0);
    let gems = gem.map(|g| g.0).unwrap_or(0);

    // --- 多行文本：血量 / 护盾 / 攻击力 / 金币 / 宝石 ---
    hp_line.0 = format!(
        "血量: {}/{}\n护盾: {}\n攻击力: {}\n金币: {}\n宝石: {}",
        hp, PLAYER_HP_MAX, def, atk, coins, gems
    );

    // --- 血条：按当前血量比例缩放 X（与 layout 中精灵 `custom_size.x` 一致） ---
    let ratio = (hp as f32 / PLAYER_HP_MAX as f32).clamp(0.0, 1.0);
    for mut xf in prog_transform.iter_mut() {
        xf.scale.x = ratio.max(0.001);
        xf.scale.y = 1.0;
    }
}