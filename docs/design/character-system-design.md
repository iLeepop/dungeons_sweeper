# 角色系统设计

与 [save-system-design.md](./save-system-design.md)、[5-16-design.md](./5-16-design.md) 衔接。角色在进入关卡前挂载可序列化效果；主菜单选择/解锁；Game Over 发放全局 Gem。

---

## 1. 目标

| 能力 | 说明 |
|------|------|
| 主菜单选角 | 滚轮切换、点击选中或 Gem 解锁；开发期纯色 Sprite 肖像 |
| 角色效果 | 复用 `effects/` 四类加载器；PreGame 建玩家/棋盘时挂载 |
| 全局状态 | 已解锁角色列表、上次选中角色、宝石（`GlobalSave` v2） |
| 局内存档 | `character_id` + `effect_specs`（`RunSave` v2） |
| Gem 来源 | Game Over 时 `gems_earned = stage` 写入全局 |

---

## 2. 角色定义

```rust
pub enum CharacterId {
    Herbalist = 0,  // 开局可用
    Slayer = 1,     // 10 Gem 解锁
}
```

| 角色 | 解锁 | 效果 |
|------|------|------|
| **Herbalist** | 默认可用 | 草地格 `TileEffectLoader` + `GrassHealPlayer(1)`，`AfterPlayerTileTrigger` |
| **Slayer** | 10 Gem | `PlayerEffectLoader` + `KillBonusDamage(1)`，`AfterEnemyKill` |

静态表：`character/defs.rs` 中 `ALL_CHARACTERS`。

---

## 3. 可序列化效果

`effects/spec.rs`：

```ron
GrassHealOnTile: ( amount: 1 ),
KillBonusDamage: ( amount: 1 ),
```

- `build_player_loader(&[SerializableEffect])` → `PlayerEffectLoader`
- `grass_heal_amount_from_specs` → 供 `GrassTile::grass_bundle` 与棋盘生成
- 写档：`capture_effect_specs_from_loader`

---

## 4. 效果阶段扩展

新增 `EffectPhase::AfterEnemyKill`：在 `taggle_consumer` 击杀敌方后 `write(EffectPhaseMessage { phase: AfterEnemyKill, ... })`。

`KillBonusDamage` 在 `apply_on_player` 中对 `Damage` 做 `saturating_add`。

---

## 5. 存档 Schema

### 5.1 `GlobalSave` v2

```ron
(
    version: 2,
    gems: 42,
    unlocked_characters: [0],
    last_selected_character: 0,
)
```

v1 迁移：`unlocked = [Herbalist]`，`last_selected = Herbalist`。

### 5.2 `PlayerSnapshot` / `RunSave` v2

```ron
player: (
    health: 100,
    damage: 1,
    defense: 0,
    gold: 0,
    gems: 0,
    character_id: 0,
    effect_specs: [GrassHealOnTile(amount: 1)],
),
```

旧局档（version 1）拒绝加载。

---

## 6. 主菜单 UI

- 标题与宝石行之间：角色轮播（中央大卡片）
- **滚轮**：环状切换 `SelectedCharacter`
- **点击已解锁**：设为选中并写 `last_selected_character`
- **点击未解锁**：若 `global_gems >= cost` 则扣费解锁并写盘
- **亮/暗**：已解锁亮色，未解锁暗色（×0.35）
- **Start / Continue**：当前浏览角色未解锁时置灰且不可点

Continue 仍从 `RunSave` 恢复局内角色与效果；仅当轮播指向已解锁角色时可点（防误触）。

---

## 7. Game Over Gem

进入 `GameOver`：

```text
gems_earned = stage.stage  // 最小为 1
global_gem += gems_earned
persist_global_save(...)
```

Game Over UI 显示 `获得宝石: +N`。与 Next Level 合并局内 player gems 独立。

---

## 8. PreGame 流程

| 路径 | 角色与效果来源 |
|------|----------------|
| Start Game | `SelectedCharacter`（须已解锁）→ `effects_from_character` |
| Continue | `RunSave.player.character_id` + `effect_specs` |

玩家实体：`RunCharacter(id)` + `PlayerEffectLoader`；棋盘草地 heal 由 `effect_specs` 推导。

---

## 9. 代码索引

| 模块 | 路径 |
|------|------|
| 角色 | `dungeons_plugin/src/character/` |
| 效果规格 | `dungeons_plugin/src/effects/spec.rs` |
| 全局档 | `dungeons_plugin/src/save/global.rs` |
| 主菜单 | `dungeons_plugin/src/ui/plugins/main_menu/` |
| Game Over 奖励 | `dungeons_plugin/src/save/mod.rs` |
