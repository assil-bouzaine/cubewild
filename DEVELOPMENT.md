# Survival improvement plan

Cubewild is a small Minecraft-style game. Prefer familiar Minecraft rules and controls,
scaled to this game's content. Keep gameplay responsive on the i3-7020U / Intel HD 620:
batch edits, bound simulation work, and keep generation, meshing and disk I/O off the
render thread. Preserve existing worlds and item metadata through every transition.

Baseline: `28e222f` (last night's work), 86 tests passed / 9 ignored.

- [x] 1. Shared, occluded block lighting for terrain, spawning and farming.
- [x] 2. Armour durability, XP repairs and basic enchantments.
- [x] 3. Hydrated farmland, light-dependent growth and two additional crops.
- [x] 4. Crafting grids, stack dragging, creative pick block and remappable controls.
- [x] 5. Fence gates, double chests, signs and matching stairs/slabs.
- [x] 6. Shields, attack cooldown feedback, charged bows and enemy attack cues.
- [x] 7. Extract gameplay responsibilities; version block storage and widen IDs.

For each step: exercise gameplay boundaries and persistence, run the relevant tests,
then run the complete release suite. At the end compare generation/meshing and real
frame-time benchmarks against the baseline, including repeated edits and autosaving.
Report measured limits honestly; compilation alone does not verify interactive play.

## Minecraft references

- [Controls](https://www.minecraft.net/en-us/article/minecraft-controls)
- [Farmland: water within four blocks, light and trampling](https://www.minecraft.net/en-us/article/block-week-farmland)
- [Shields: use to block attacks from the front](https://www.minecraft.net/en-us/article/taking-inventory--shield)
- [Overworld monsters require complete darkness](https://www.minecraft.net/en-us/article/new-world-generation-java-available-testing)
- [Fence gates and their recipe](https://www.minecraft.net/en-us/article/fence-gate)
- [Enchantments](https://learn.microsoft.com/en-us/minecraft/creator/documents/introtoenchantments)
- [Item list](https://minecraft.wiki/w/Item): every item, linking to its recipe and behaviour
  (block pages too, e.g. [Chest](https://minecraft.wiki/w/Chest), [Bed](https://minecraft.wiki/w/Bed))
- [Item and block IDs](https://minecraft-ids.grahamedgecombe.com/): a compact checklist of all items
- [Block list](https://minecraftsearch.com/blocks): every block by category (crafting,
  redstone, functional, decorative, nature), each with hardness, blast resistance, tool,
  burn time, stack size and recipes

## Blocks from Minecraft's block list

Every block's Minecraft properties live in `src/blockinfo.rs`; mining time, drops,
fuel, explosions and tooltips read them. New blocks are rows of `src/catalogue.rs`
(world id `448 + row`, item id `1024 + row`, appended only), each with its name,
procedural textures, properties, shape and light. Adding a block is adding a row,
its recipes in `items::RECIPES` and, for a shaped recipe, its pattern in `crafting.rs`.

Done (phase 1): Minecraft properties and mining rules for every existing block; the
crafting category's stations (crafting table, furnace, smoker, blast furnace, smithing
table, stonecutter, grindstone, anvil and its chipped/damaged stages, enchanting table
with Minecraft's shape and recipe, fletching/cartography tables and loom as placeable
blocks); 52 new blocks; recipe fixes (metal blocks, stone bricks, sandstone, clay and
bricks, mossy cobblestone).

Done (phase 2): campfire and soul campfire (four cooking places kept in the furnace
map as a cooker kind, saved with per-place timers), iron bars, sixteen stained glass and
stained glass panes (panes and bars share the glass pane's flat quads; stained glass is
drawn in the translucent pass), sixteen carpets, eleven walls, the hanging lantern (a
state row that drops the lantern). Shaped blocks may now have up to eight boxes.

Next phases, by how well they fit:
1. Brewing stand (needs potions); stairs and slabs for the new stone families (needs
   new stair and slab families).
2. Concrete and concrete powder (16 each; powder falls and sets in water), glazed
   terracotta.
3. New wood types (spruce, jungle, acacia, dark oak) for the trees already generated:
   logs, planks, stairs, slabs, fences, doors, signs.
4. Nature blocks: pumpkins, melons, flowers, vines, lily pads, mushroom blocks, coral.
5. Redstone (54 blocks) once there is a power system.
Stack size stays 99 for now: Minecraft's 64 would cut saved stacks above 64 on load
(`loot::parse_stack` clamps), so it needs a split-on-load migration first.

## Items from Minecraft's item list

Added: bowl, apple, golden apple, iron and gold nuggets, golden carrot, bone meal,
rotten flesh and snowball (items `BOWL`..`SNOWBALL`, icons in `src/icons.rs`); chests
with facing and joined large chests; two-block, 9/16-high beds with the wool recipe.

Equivalents already present under other names: crystal (diamond), gem (emerald),
striker (flint and steel), hide (leather), fibre (string), coal from logs (charcoal),
forest stew (mushroom stew), ember gate (nether portal).

Still missing, roughly in order of how well they fit the existing systems: beetroot
(crop, seeds, soup), sugar cane with sugar/paper/book/bookshelf, melon and pumpkin (with
pie), cookie and cake, feather/egg/chicken, glass bottle, brick and clay ball as items,
lead and name tag, flower pot, painting and item frame, armour stand, map, crossbow,
minecart and rails, redstone, lapis, potions and brewing, ender pearls, elytra, trident.

## Implementation

All seven requested systems are implemented. Lighting, farming, crafting, equipment,
controls, building interactions, combat, persistence and inventory UI have dedicated
modules; `main.rs` is approximately 3,800 lines rather than 5,580.

New chunks use a checksummed `CWB2` header, format and registry versions, a cell count,
and little-endian 16-bit IDs. Existing raw byte chunks and half-height worlds remain
readable. Full-height byte chunks upgrade when edited; untouched terrain is not rewritten.
Height migrations and edited chunks commit through coordinated checkpoints. Permanent ID families live
in `src/registry.rs`; old ID meanings are never reassigned. Terrain materials use a
separate 16-bit texture index, removing the old seven-bit limit.

Static light is built on workers, cropped vertically to the region sources can reach.
Mesh jobs reuse current light where transmission and emission did not change. Changing
walls or emitters invalidates neighbouring light and meshes; stale jobs cannot publish
results. Workers bake brightness into a vertex byte; the GPU only normalizes it,
with no light-table lookup or per-pixel source loop.

The XP systems use existing Cubewild materials and three selectable enchantment tiers.
Signs have four 15-character lines and display their text when targeted. Double chests
retain two independent 27-slot stores so splitting them preserves both halves.

## Validation

`cargo test --offline --release`: **108 passed, 9 ignored**, with no compiler warnings.
The new coverage exercises wall and border lighting, stale/cached light, crop hydration
and darkness, equipment costs/metadata/upgrades, shaped crafting, full-inventory grid
checkpoints, stack distribution, binding persistence, container pairing, sign persistence,
gate orientation, combat warnings, directional defence and versioned chunk corruption.

Automated desktop flows passed in isolated temporary worlds, with assertions against
their saved state: 2×2 and 3×3 crafting, even stack dragging, XP repair/enchanting,
large-chest shift transfer, sign editing, charged-bow release and cancellation, shield
raising, and remapping/reloading controls. Screenshots were inspected and revealed a
texture-layer routing bug and small-window layout issues, both fixed. Performance runs
are separate from these functional tests and from compilation.

Final matched performance results are recorded in `PERFORMANCE.md`.
