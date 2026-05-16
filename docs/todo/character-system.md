# 角色系统实现清单

设计详见 [character-system-design.md](../design/character-system-design.md)。

## 阶段 1：文档

- [x] `docs/design/character-system-design.md`
- [x] `docs/todo/character-system.md`
- [x] `save-system-design.md` 交叉引用

## 阶段 2：效果规格与击杀阶段

- [x] `effects/spec.rs`：`SerializableEffect`、`build_player_loader`、`grass_heal_amount_from_specs`
- [x] `EffectPhase::AfterEnemyKill`、`KillBonusDamage`
- [x] `PlayerEffectContext` 增加 `player_damage`
- [x] `taggle_consumer` 击杀后广播 `AfterEnemyKill`

## 阶段 3：角色模块与全局档 v2

- [x] `character/`：`CharacterId`、`CharacterDef`、Herbalist / Slayer
- [x] `GlobalSave` v2：解锁列表、`last_selected`
- [x] `SelectedCharacter`、`UnlockedCharacters` 资源/组件

## 阶段 4：PreGame 挂载

- [x] `RunCharacter` 组件
- [x] `player_bundle` / `player_bundle_from_snapshot` 带效果
- [x] `GrassTile` 按角色启用 `TileEffectLoader`
- [x] `lib.rs` PreGame 链传入 `effect_specs`

## 阶段 5：局存档 v2

- [x] `PlayerSnapshot`：`character_id`、`effect_specs`
- [x] `RUN_SAVE_VERSION = 2`
- [x] `capture_player_snapshot` / 读档恢复

## 阶段 6：主菜单 UI

- [x] 角色轮播布局（纯色块）
- [x] 滚轮切换、点击选中/解锁
- [x] Start / Continue 门控

## 阶段 7：Game Over Gem

- [x] `on_enter_game_over` 发放全局 Gem
- [x] Game Over UI 显示奖励

## 手动验收

- [ ] 主菜单滚轮：Herbalist 亮、Slayer 暗
- [ ] Game Over 后 Gem 增加；10 Gem 解锁 Slayer
- [ ] Herbalist 踩草 +1 HP
- [ ] Slayer 击杀 +1 Damage
- [ ] 暂停 Continue 效果仍在
- [ ] 浏览未解锁角色时 Start/Continue 不可点
