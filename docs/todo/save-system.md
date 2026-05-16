# 存档系统实现清单

设计详见 [save-system-design.md](../design/save-system-design.md)。

## 阶段 1：基础设施

- [x] `serde` / `ron` / `dirs` 依赖
- [x] `dungeons_plugin/src/save/`：`io`、`global`、`run`、`snapshot`、`SavePlugin`
- [x] `SavePaths`、`GLOBAL_SAVE_VERSION`、`RUN_SAVE_VERSION`

## 阶段 2：全局宝石

- [x] `GlobalSave` 读写 `global_save.ron`
- [x] 启动 spawn `GlobalProfile` + `Gem`
- [x] Next Level Continue：合并玩家宝石 → 全局 → 写盘
- [x] 主菜单显示全局宝石

## 阶段 3：局内快照

- [x] `SerializableTile` ↔ `Tile` / `EnemyType`
- [x] `capture_run_snapshot` / `write_run_save` / `load_run_save` / `delete_run_save`
- [x] `TileMap::from_saved_grid`
- [x] `rebuild_board_from_snapshot` + `apply_board_restoration`

## 阶段 4：流程挂钩

- [x] `PendingRunRestore`、`RunSaveAvailable`
- [x] PreGame：`prepare_pregame` 分支 procedural / restore
- [x] `setup_player` 读档更新组件
- [x] 回主菜单写档；Game Over 删档；新游戏/重启删档

## 阶段 5：主菜单 UI

- [x] `ContinueRunButton` + interaction
- [x] Start Game 删除局存档

## 手动验收

- [ ] 新游戏 → 玩几步 → 暂停回主菜单 → Continue 恢复棋盘与 HP/金币
- [ ] 踩出口 → Next Level 菜单回主菜单 → Continue 仍为出口前局面
- [ ] Next Level Continue 升关：玩家宝石并入主菜单显示，局内 Gem 归零
- [ ] 死亡 Game Over：Continue 按钮不可用
- [ ] Start Game 覆盖旧局存档
