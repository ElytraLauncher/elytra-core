# 第三方声明 / Third-Party Notices

本仓库的 [GPL-3.0-only 许可证](LICENSE) 覆盖 Elytra 自身代码；下列第三方内容与数据随本仓库分发，权利归各自所有者。

The project's [GPL-3.0-only license](LICENSE) covers Elytra's own code; the third-party content and data below are distributed with this repository and remain the property of their respective owners.

---

## PCL WikiEntries 快照 / PCL WikiEntries snapshot

中文内容搜索内嵌一份来自 Plain Craft Launcher 的 `WikiEntries.txt` **未修改**快照（`crates/domain/src/data/wiki_entries.txt`），字节级等同 PCL 仓库 commit [`ade6e04c`](https://github.com/Meloong-Git/PCL/blob/ade6e04c6eab65e945b0fc51faae1ba8e714245c/PCLCS/Resource/WikiEntries.txt)（版本 2.14.0.0）的 `PCLCS/Resource/WikiEntries.txt`。

Plain Craft Launcher 版权归成都瓜皮龙科技有限公司所有，按 [PCL 分发有限许可](https://github.com/Meloong-Git/PCL/blob/ade6e04c6eab65e945b0fc51faae1ba8e714245c/LICENCE) 分发；该快照包含由 [MC 百科](https://www.mcmod.cn/) 贡献的中文译名信息。

该快照的使用与本项目整体无关，不代表 PCL 或成都瓜皮龙科技有限公司，亦不得将本项目与 PCL 相混淆。

The Chinese content search embeds an **unmodified** `WikiEntries.txt` snapshot from Plain Craft Launcher (`crates/domain/src/data/wiki_entries.txt`), byte-identical to `PCLCS/Resource/WikiEntries.txt` at PCL commit [`ade6e04c`](https://github.com/Meloong-Git/PCL/blob/ade6e04c6eab65e945b0fc51faae1ba8e714245c/PCLCS/Resource/WikiEntries.txt) (version 2.14.0.0).

Plain Craft Launcher is copyright Chengdu Gua Pi Long Technology Co., Ltd. and distributed under the [PCL Limited Distribution License](https://github.com/Meloong-Git/PCL/blob/ade6e04c6eab65e945b0fc51faae1ba8e714245c/LICENCE). The snapshot contains Chinese project-name information contributed by [MC Encyclopedia](https://www.mcmod.cn/).

This use is independent of the project as a whole, does not represent PCL or Chengdu Gua Pi Long Technology Co., Ltd., and must not be confused with PCL.

---

## 世界预览调色数据 / World preview palette data

`crates/map/assets/natural_block_colors.json`、`grass-colourmap.png` 与 `foliage-colourmap.png` 由 Minecraft 客户端素材派生，仅用于存档世界预览的地图着色；版权归 Mojang Studios 所有，不代表官方背书。

The world-preview palette files under `crates/map/assets/` are derived from Minecraft client assets and are used only for coloring the save-map preview. Copyright belongs to Mojang Studios; no endorsement is implied.

## cubiomes（种子预览）/ cubiomes (seed preview)

`crates/seedmap/vendor/cubiomes/` 是 [xpple/cubiomes](https://github.com/xpple/cubiomes) 在 commit [`d934de1d4e5e71db0b789296241f929a50306504`](https://github.com/xpple/cubiomes/commit/d934de1d4e5e71db0b789296241f929a50306504) 的裁剪快照（仅核心生成器、feature helper 与参数表，不含 loot），按 MIT License 分发，版权归 Cubitect 与 xpple 贡献者所有（见 `crates/seedmap/vendor/cubiomes/LICENSE`）。其中的群系/结构参数表由 Minecraft Java Edition 数据逆向得到；Minecraft 数据版权归 Mojang Studios 所有，本项目仅用于本地种子预览，与 Mojang/Microsoft 无关联。

`crates/seedmap/vendor/cubiomes/` is a trimmed snapshot of [xpple/cubiomes](https://github.com/xpple/cubiomes) at commit [`d934de1d4e5e71db0b789296241f929a50306504`](https://github.com/xpple/cubiomes/commit/d934de1d4e5e71db0b789296241f929a50306504) (core generator, feature helpers and parameter tables only; no loot), distributed under the MIT License, copyright Cubitect and xpple contributors (see `crates/seedmap/vendor/cubiomes/LICENSE`). Its biome/structure tables are reverse-engineered from Minecraft: Java Edition data; Minecraft data is copyright Mojang Studios and is used here only for local seed preview. This project is not affiliated with Mojang Studios or Microsoft.

## Minecraft 素材 / Minecraft assets

`crates/map/assets/` 中的调色素材由 Minecraft 客户端素材派生，版权归 Mojang Studios 所有；本项目与 Mojang Studios / Microsoft 无关联。
