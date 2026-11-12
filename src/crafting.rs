//! Shaped crafting and lossless cursor distribution. Recipes can be offset and
//! mirrored, as in Minecraft. One ingredient is consumed from each occupied cell.
use crate::items::{self::*, Kind};
use crate::world::*;
use crate::catalogue as c;

/// Recipes that, as in Minecraft, take any wood's planks (where they name oak planks).
fn any_planks(out: Item) -> bool {
    matches!(out, STICK | WORKBENCH | CHEST | BOWL | BOAT | SHIELD | BED | WOOD_PICK | WOOD_AXE | WOOD_SHOVEL | WOOD_HOE | WOOD_SWORD)
        || [c::BOOKSHELF, c::SMITHING_TABLE, c::FLETCHING_TABLE, c::CARTOGRAPHY_TABLE, c::LOOM, c::GRINDSTONE].iter().any(|&row| out == cat(row))
}
/// Recipes that take any log (where they name oak logs).
fn any_log(out: Item) -> bool {
    out == cat(c::SMOKER) || out == cat(c::CAMPFIRE) || out == cat(c::SOUL_CAMPFIRE)
}
pub fn is_planks_item(i: Item) -> bool {
    matches!(i, PLANKS | BIRCH_PLANKS) || i == DEEP_ITEM + 8 || (0..3).any(|w| i == cat(c::planks_row(w)))
}
pub fn is_log_item(i: Item) -> bool {
    matches!(i, LOG | BIRCH_LOG) || i == DEEP_ITEM + 6 || (0..3).any(|w| i == cat(c::log_row(w)))
}
/// Whether the recipe for `out` takes `have` where it lists `want`.
pub fn takes(out: Item, want: Item, have: Item) -> bool {
    have == want || (want == PLANKS && any_planks(out) && is_planks_item(have)) || (want == LOG && any_log(out) && is_log_item(have))
}

fn pattern(r: &Recipe) -> Option<(usize, Vec<Item>)> {
    let p = r.ins.first()?.0;
    let s = STICK;
    let shaped = if let Some((kind, _)) = gear(r.out.0).filter(|_| r.ins.first().is_some_and(|&(i, _)| gear(i).is_none())) {
        match kind {
            Kind::Pick => (3, vec![p, p, p, 0, s, 0, 0, s, 0]),
            Kind::Axe => (2, vec![p, p, p, s, 0, s]),
            Kind::Shovel => (1, vec![p, s, s]),
            Kind::Hoe => (2, vec![p, p, 0, s, 0, s]),
            Kind::Sword => (1, vec![p, p, s]),
            Kind::Helmet => (3, vec![p, p, p, p, 0, p]),
            Kind::Chest => (3, vec![p, 0, p, p, p, p, p, p, p]),
            Kind::Legs => (3, vec![p, p, p, p, 0, p, p, 0, p]),
            Kind::Boots => (3, vec![p, 0, p, p, 0, p]),
        }
    } else {
        match r.out.0 {
            PLANKS | BIRCH_PLANKS => (1, vec![p]),
            STICK => (1, vec![p, p]),
            TORCH => (1, vec![COAL, STICK]),
            WORKBENCH => (2, vec![p, p, p, p]),
            FURNACE | CHEST => (3, vec![p, p, p, p, 0, p, p, p, p]),
            LADDER => (3, vec![s, 0, s, s, s, s, s, 0, s]),
            i if matches!(wood_piece(i), Some((_, Piece::Trapdoor))) => (3, vec![p, p, p, p, p, p]),
            // a door: two wide, three high
            i if matches!(wood_piece(i), Some((_, Piece::Door))) => (2, vec![p, p, p, p, p, p]),
            BUCKET => (3, vec![p, 0, p, 0, p, 0]),
            BOAT => (3, vec![p, 0, p, p, p, p]),
            BREAD => (3, vec![p, p, p]),
            ANVIL_ITEM => (3, vec![IRON_BLOCK, IRON_BLOCK, IRON_BLOCK, 0, IRON_INGOT, 0, IRON_INGOT, IRON_INGOT, IRON_INGOT]),
            i if matches!(wood_piece(i), Some((_, Piece::Gate))) => {
                let wood = r.ins[1].0;
                (3, vec![s, wood, s, s, wood, s])
            }
            i if matches!(wood_piece(i), Some((_, Piece::Sign))) => (3, vec![p, p, p, p, p, p, 0, s, 0]),
            i if i == FENCE || i == BIRCH_FENCE || (0..3).any(|w| i == cat(c::fence_row(w))) => (3, vec![p, s, p, p, s, p]),
            SHIELD => (3, vec![p, IRON_INGOT, p, p, p, p, 0, p, 0]),
            // wool over planks (the old hide recipe stays shapeless)
            BED if p != PLANKS => {
                let wood = r.ins[1].0;
                (3, vec![p, p, p, wood, wood, wood])
            }
            BOWL => (3, vec![p, 0, p, 0, p, 0]),
            // gold all round the apple or carrot
            GOLDEN_APPLE | GOLDEN_CARROT => {
                let middle = r.ins[1].0;
                (3, vec![p, p, p, p, middle, p, p, p, p])
            }
            SNOW => (2, vec![p, p, p, p]),
            PAPER => (3, vec![p, p, p]),
            CLAY | BRICK => (2, vec![p, p, p, p]),
            i if i == cat(c::BOOKSHELF) => {
                let b = r.ins[1].0;
                (3, vec![p, p, p, b, b, b, p, p, p])
            }
            i if i == cat(c::CHISELED_STONE_BRICKS) || i == cat(c::CHISELED_SANDSTONE) || i == cat(c::CHISELED_SLATE) => (1, vec![p, p]),
            i if [c::CUT_SANDSTONE, c::RED_SANDSTONE, c::CUT_RED_SANDSTONE, c::POLISHED_SLATE, c::SLATE_BRICKS, c::SLATE_TILES]
                .iter()
                .any(|&row| i == cat(row)) =>
            {
                (2, vec![p, p, p, p])
            }
            // dye in the middle of eight terracotta
            i if (cat(c::TERRACOTTA)..cat(c::TERRACOTTA + 16)).contains(&i) => {
                let dye = r.ins[1].0;
                (3, vec![p, p, p, p, dye, p, p, p, p])
            }
            i if i == cat(c::SMOKER) => {
                let log = r.ins[1].0;
                (3, vec![0, log, 0, log, p, log, 0, log, 0])
            }
            i if i == cat(c::BLAST_FURNACE) => {
                let smooth = r.ins[2].0;
                (3, vec![p, p, p, p, FURNACE, p, smooth, smooth, smooth])
            }
            i if i == cat(c::SMITHING_TABLE) || i == cat(c::FLETCHING_TABLE) || i == cat(c::CARTOGRAPHY_TABLE) => {
                let wood = r.ins[1].0;
                (2, vec![p, p, wood, wood, wood, wood])
            }
            i if i == cat(c::STONECUTTER) => (3, vec![0, p, 0, STONE, STONE, STONE]),
            i if i == cat(c::GRINDSTONE) => (3, vec![p, SLAB + 2, p, PLANKS, 0, PLANKS]),
            i if i == cat(c::LOOM) => (2, vec![p, p, PLANKS, PLANKS]),
            i if i == cat(c::LANTERN) => (3, vec![p, p, p, p, TORCH, p, p, p, p]),
            i if i == cat(c::CAMPFIRE) || i == cat(c::SOUL_CAMPFIRE) => {
                let (fuel, log) = (r.ins[1].0, r.ins[2].0);
                (3, vec![0, p, 0, p, fuel, p, log, log, log])
            }
            // two rows of three: bars, panes and walls
            i if i == cat(c::IRON_BARS) || (cat(c::WALL_ROW)..cat(c::WALL_ROW + 11)).contains(&i) => (3, vec![p; 6]),
            i if (cat(c::STAINED_PANE_ROW)..cat(c::STAINED_PANE_ROW + 16)).contains(&i) && p != PANE => (3, vec![p; 6]),
            // eight glass (or panes) round a dye
            i if (cat(c::STAINED_GLASS_ROW)..cat(c::STAINED_PANE_ROW + 16)).contains(&i) => {
                let dye = r.ins[1].0;
                (3, vec![p, p, p, p, dye, p, p, p, p])
            }
            i if (cat(c::CARPET_ROW)..cat(c::CARPET_ROW + 16)).contains(&i) => (2, vec![p, p]),
            i if stair_material(i).is_some() => (3, vec![p, 0, 0, p, p, 0, p, p, p]),
            i if slab_material(i).is_some() => (3, vec![p, p, p]),
            _ => return None,
        }
    };
    Some(shaped)
}

pub fn recipe(grid: &[Stack; 9], size: usize, workbench: bool) -> Option<usize> {
    if !(2..=3).contains(&size) {
        return None;
    }
    if grid.iter().enumerate().any(|(i, s)| !s.is_empty() && (i % 3 >= size || i / 3 >= size)) {
        return None;
    }
    RECIPES.iter().enumerate().find_map(|(ri, r)| {
        if r.station != AIR && (r.station != WORKBENCH || !workbench) {
            return None;
        }
        // Tools and machinery must use the documented shape; dyes and simple
        // material combinations retain their shapeless recipes.
        let matches = if let Some((pw, cells)) = pattern(r) {
            let ph = cells.len() / pw;
            if pw > size || ph > size {
                false
            } else {
                (0..=size - ph).any(|oy| {
                    (0..=size - pw).any(|ox| {
                        [false, true].into_iter().any(|mirror| {
                            (0..size).all(|y| {
                                (0..size).all(|x| {
                                    let expected = if x >= ox && x < ox + pw && y >= oy && y < oy + ph {
                                        cells[(y - oy) * pw + if mirror { pw - 1 - (x - ox) } else { x - ox }]
                                    } else {
                                        AIR
                                    };
                                    let st = grid[y * 3 + x];
                                    if st.is_empty() || expected == AIR { st.is_empty() && expected == AIR } else { takes(r.out.0, expected, st.item) }
                                })
                            })
                        })
                    })
                })
            }
        } else {
            let mut counts = std::collections::BTreeMap::new();
            for st in grid.iter().filter(|s| !s.is_empty()) {
                // (any wood counts as the planks or log the recipe names)
                let item = r.ins.iter().map(|&(i, _)| i).find(|&want| takes(r.out.0, want, st.item)).unwrap_or(st.item);
                *counts.entry(item).or_insert(0u32) += 1;
            }
            let mut expected = std::collections::BTreeMap::new();
            for &(item, n) in r.ins {
                *expected.entry(item).or_insert(0u32) += n;
            }
            !counts.is_empty() && counts == expected
        };
        matches.then_some(ri)
    })
}

pub fn craft(inv: &mut Inventory, size: usize, workbench: bool, to_bag: bool) -> bool {
    let Some(i) = recipe(&inv.grid, size, workbench) else {
        return false;
    };
    let out = crate::equipment::craft_output(&RECIPES[i], &inv.grid);
    let mut next = inv.clone();
    let left = if to_bag { next.pick_up(out) } else { stash_in(std::slice::from_mut(&mut next.held), out) };
    if !left.is_empty() {
        return false;
    }
    for st in &mut next.grid {
        if !st.is_empty() {
            st.n -= 1;
            if st.n == 0 {
                *st = Stack::default();
            }
        }
    }
    *inv = next;
    true
}

/// Even left-drag distribution or one per slot on right drag. Incompatible and
/// full stacks keep their contents; overflow stays on the cursor.
pub fn distribute(held: &mut Stack, slots: &mut [Stack], selected: &[usize], right: bool) {
    if held.is_empty() {
        return;
    }
    let mut indices = Vec::new();
    for &i in selected {
        if i < slots.len()
            && !indices.contains(&i)
            && (slots[i].is_empty() || (slots[i].item == held.item && slots[i].enchants == held.enchants && slots[i].wear == held.wear))
            && slots[i].n < max_stack(held.item)
        {
            indices.push(i);
        }
    }
    if !right {
        indices.truncate(held.n as usize);
    }
    let each = if right { 1 } else { held.n / indices.len().max(1) as u32 };
    for i in indices {
        let n = each.min(held.n).min(max_stack(held.item).saturating_sub(slots[i].n));
        if n > 0 {
            slots[i] = Stack { n: slots[i].n + n, ..*held };
            held.n -= n;
        }
    }
    if held.n == 0 {
        *held = Stack::default();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn grids_accept_offsets_and_mirrors_and_enforce_their_size() {
        let mut inv = Inventory::new();
        inv.grid[4] = Stack::new(LOG, 2);
        assert!(craft(&mut inv, 2, false, false));
        assert_eq!(inv.held, Stack::new(PLANKS, 4));
        inv.grid.fill(Stack::default());
        inv.grid[0] = Stack::new(COBBLE, 1);
        inv.grid[3] = Stack::new(STICK, 1);
        inv.grid[6] = Stack::new(STICK, 1);
        assert!(recipe(&inv.grid, 2, false).is_none());
        assert_eq!(RECIPES[recipe(&inv.grid, 3, true).unwrap()].out.0, STONE_SHOVEL);
        inv.grid[1] = Stack::new(DIRT, 1);
        assert!(recipe(&inv.grid, 3, true).is_none());
    }
    #[test]
    fn full_cursor_keeps_grid_and_dragging_conserves_every_item() {
        let mut inv = Inventory::new();
        inv.grid[0] = Stack::new(LOG, 1);
        inv.held = Stack::new(STONE, 99);
        assert!(!craft(&mut inv, 2, false, false));
        assert_eq!(inv.grid[0].n, 1);
        let mut held = Stack::new(PLANKS, 10);
        let mut slots = [Stack::default(); 3];
        distribute(&mut held, &mut slots, &[0, 1, 1, 2], false);
        assert_eq!((held.n, slots.map(|s| s.n)), (1, [3, 3, 3]));
        distribute(&mut held, &mut slots, &[0, 1, 2], true);
        assert!(held.is_empty());
        assert_eq!(slots.iter().map(|s| s.n).sum::<u32>(), 10);
        let mut held = Stack::new(IRON_PICK, 1);
        let mut slots = [Stack::default(); 2];
        distribute(&mut held, &mut slots, &[0, 1], false);
        assert!(held.is_empty());
        assert_eq!(slots[0], Stack::new(IRON_PICK, 1));
        assert!(slots[1].is_empty());
    }
    fn grid(cells: [Item; 9]) -> [Stack; 9] {
        cells.map(|i| if i == AIR { Stack::default() } else { Stack::new(i, 1) })
    }
    fn made(cells: [Item; 9], workbench: bool) -> Option<(Item, u32)> {
        recipe(&grid(cells), 3, workbench).map(|i| RECIPES[i].out)
    }
    #[test]
    fn minecraft_item_recipes_use_their_shapes() {
        let (w, p, g, n) = (CLOTH, PLANKS, GOLD_INGOT, GOLD_NUGGET);
        // a bed: wool over planks, in either half of the grid
        assert_eq!(made([w, w, w, p, p, p, 0, 0, 0], true), Some((BED, 1)));
        assert_eq!(made([0, 0, 0, w, w, w, p, p, p], true), Some((BED, 1)));
        assert_eq!(made([p, p, p, w, w, w, 0, 0, 0], true), None);
        assert_eq!(made([w, w, w, p, p, p, 0, 0, 0], false), None);
        // the old hide bed still crafts
        assert_eq!(made([p, p, p, HIDE, HIDE, HIDE, 0, 0, 0], true), Some((BED, 1)));
        assert_eq!(made([p, 0, p, 0, p, 0, 0, 0, 0], true), Some((BOWL, 4)));
        assert_eq!(made([g, g, g, g, APPLE, g, g, g, g], true), Some((GOLDEN_APPLE, 1)));
        assert_eq!(made([n, n, n, n, CARROT, n, n, n, n], true), Some((GOLDEN_CARROT, 1)));
        assert_eq!(made([g, g, g, g, g, g, g, g, APPLE], true), None);
        assert_eq!(made([n; 9], true), Some((GOLD_INGOT, 1)));
        assert_eq!(made([IRON_INGOT, 0, 0, 0, 0, 0, 0, 0, 0], false), Some((IRON_NUGGET, 9)));
        assert_eq!(made([BONE, 0, 0, 0, 0, 0, 0, 0, 0], false), Some((BONE_MEAL, 3)));
        assert_eq!(made([BONE_MEAL, 0, 0, 0, 0, 0, 0, 0, 0], false), Some((DYE, 1)));
        let s = SNOWBALL;
        assert_eq!(recipe(&grid([s, s, 0, s, s, 0, 0, 0, 0]), 2, false).map(|i| RECIPES[i].out), Some((SNOW, 1)));
        assert_eq!(made([MUSHROOM, MUSHROOM, BERRIES, BERRIES, BOWL, 0, 0, 0, 0], true), Some((STEW, 1)));
    }
    #[test]
    fn block_list_recipes_match_minecraft() {
        use crate::catalogue as c;
        let (i, p, f) = (IRON_INGOT, PLANKS, FURNACE);
        assert_eq!(made([i; 9], true), Some((IRON_BLOCK, 1)));
        assert_eq!(made([STONE, STONE, 0, STONE, STONE, 0, 0, 0, 0], false), Some((STONE_BRICK, 4)));
        assert_eq!(made([SAND, SAND, 0, SAND, SAND, 0, 0, 0, 0], false), Some((SANDSTONE, 1)));
        let ss = cat(c::SMOOTH_STONE);
        assert_eq!(made([i, i, i, i, f, i, ss, ss, ss], true), Some((cat(c::BLAST_FURNACE), 1)));
        assert_eq!(made([0, LOG, 0, LOG, f, LOG, 0, LOG, 0], true), Some((cat(c::SMOKER), 1)));
        let n = IRON_NUGGET;
        assert_eq!(made([n, n, n, n, TORCH, n, n, n, n], true), Some((cat(c::LANTERN), 1)));
        assert_eq!(made([p, p, p, BOOK, BOOK, BOOK, p, p, p], true), Some((cat(c::BOOKSHELF), 1)));
        assert_eq!(made([i, i, 0, p, p, 0, p, p, 0], true), Some((cat(c::SMITHING_TABLE), 1)));
        assert_eq!(made([0, i, 0, STONE, STONE, STONE, 0, 0, 0], true), Some((cat(c::STONECUTTER), 1)));
        let t = TERRACOTTA;
        assert_eq!(made([t, t, t, t, DYE + 14, t, t, t, t], true), Some((cat(c::TERRACOTTA + 14), 8)));
        assert_eq!(made([REED_ITEM, REED_ITEM, REED_ITEM, 0, 0, 0, 0, 0, 0], true), Some((PAPER, 3)));
        assert_eq!(made([BOOK, CRYSTAL, 0, 0, 0, 0, 0, 0, 0], true), None);
        // stonecutter recipes are the recipe book's, at the stonecutter, never the grid's
        assert_eq!(made([STONE, 0, 0, 0, 0, 0, 0, 0, 0], true), None);
        assert!(RECIPES.iter().any(|r| r.station == STONECUTTER && r.ins == [(STONE, 1)] && r.out == (SLAB + 3, 2)));
        for r in RECIPES {
            assert!(r.station == AIR || STATIONS.contains(&r.station), "{}", name(r.out.0));
            assert!(valid(r.out.0) && r.ins.iter().all(|&(i, n)| valid(i) && n > 0), "{}", name(r.out.0));
        }
    }
    #[test]
    fn phase_two_recipes_use_minecraft_shapes() {
        use crate::catalogue as c;
        let (s, g, i) = (STICK, GLASS, IRON_INGOT);
        assert_eq!(made([0, s, 0, s, COAL, s, LOG, LOG, LOG], true), Some((cat(c::CAMPFIRE), 1)));
        assert_eq!(made([i, i, i, i, i, i, 0, 0, 0], true), Some((cat(c::IRON_BARS), 16)));
        assert_eq!(made([g, g, g, g, DYE + 11, g, g, g, g], true), Some((cat(c::STAINED_GLASS_ROW + 11), 8)));
        let blue = cat(c::STAINED_GLASS_ROW + 11);
        assert_eq!(made([blue, blue, blue, blue, blue, blue, 0, 0, 0], true), Some((cat(c::STAINED_PANE_ROW + 11), 16)));
        let p = PANE;
        assert_eq!(made([p, p, p, p, DYE + 1, p, p, p, p], true), Some((cat(c::STAINED_PANE_ROW + 1), 8)));
        assert_eq!(recipe(&grid([CLOTH, CLOTH, 0, 0, 0, 0, 0, 0, 0]), 2, false).map(|i| RECIPES[i].out), Some((cat(c::CARPET_ROW), 3)));
        let w = WOOL_ITEM + 13;
        assert_eq!(recipe(&grid([w, w, 0, 0, 0, 0, 0, 0, 0]), 2, false).map(|i| RECIPES[i].out), Some((cat(c::CARPET_ROW + 14), 3)));
        let cb = COBBLE;
        assert_eq!(made([cb, cb, cb, cb, cb, cb, 0, 0, 0], true), Some((cat(c::WALL_ROW), 6)));
        assert!(RECIPES.iter().any(|r| r.station == STONECUTTER && r.ins == [(cb, 1)] && r.out == (cat(c::WALL_ROW), 1)));
    }
    #[test]
    fn block_list_stairs_and_slabs_round_trip_and_follow_minecraft() {
        use crate::catalogue as c;
        for i in MORE_SLAB_ITEM..MORE_SHAPES_END {
            let b = block_of(i);
            assert!(valid(i) && is_block(i) && name(i).ends_with(if i < MORE_STAIRS_ITEM { "Slab" } else { "Stairs" }), "{i}");
            assert!(crate::registry::valid(b) && is_shaped(b) && item_of(b) == i, "{i}");
            assert_eq!(is_slab(b), i < MORE_STAIRS_ITEM);
            assert_eq!(is_stairs(b), i >= MORE_STAIRS_ITEM);
        }
        // a stone slab: lower and upper halves, Minecraft's hardness, two make stone again
        let (low, high) = (crate::building::slab(8, false), crate::building::slab(8, true));
        assert!(!slab_top(low) && slab_top(high) && slab_mat(high) == 8 && SHAPE_FULL[8] == STONE);
        assert_eq!(crate::blockinfo::props(high).hardness, 1.5);
        let stairs = crate::building::stairs(16, 3, true);
        assert!(is_stairs(stairs) && stair_mat(stairs) == 16 && SHAPE_FULL[16] == c::block(c::SLATE_TILES));
        assert_eq!(shape_boxes(stairs, |_, _| AIR).1, 2);
        let s = STONE;
        assert_eq!(made([s, s, s, 0, 0, 0, 0, 0, 0], true), Some((MORE_SLAB_ITEM, 6)));
        assert_eq!(made([s, 0, 0, s, s, 0, s, s, s], true), Some((MORE_STAIRS_ITEM, 4)));
        let ss = cat(c::SMOOTH_STONE);
        assert_eq!(made([ss, ss, ss, 0, 0, 0, 0, 0, 0], true), Some((MORE_SLAB_ITEM + 9, 6)));
        assert!(!RECIPES.iter().any(|r| r.ins == [(ss, 6)] && stair_material(r.out.0).is_some()), "no smooth stone stairs, as in Minecraft");
        // concrete powder, and glazed terracotta from the furnace
        assert_eq!(made([SAND, SAND, SAND, SAND, GRAVEL, GRAVEL, GRAVEL, GRAVEL, DYE + 3], false), Some((cat(c::POWDER_ROW + 3), 8)));
        assert_eq!(crate::furnace::smelt(cat(c::TERRACOTTA + 9)), Some((cat(c::GLAZED_ROW + 9), 1)));
    }
    #[test]
    fn every_wood_has_its_pieces_and_states_stay_in_their_family() {
        use crate::building::*;
        for w in 0..5 {
            for piece in PIECES_ALL {
                let item = wood_item(w, piece);
                assert_eq!(wood_piece(item), Some((w, piece)), "wood {w} {piece:?}");
                assert!(valid(item) && name(item) != "?" && !desc(item).is_empty(), "wood {w} {piece:?}");
            }
            let door = door_base(w) + 2 + DOOR_HINGE_RIGHT;
            assert!(is_door(door) && is_door(door + DOOR_UPPER) && is_door(door ^ DOOR_OPEN));
            assert_eq!((door_wood(door) as usize, door_wood(door + DOOR_UPPER) as usize, door_item(door)), (w, w, wood_item(w, Piece::Door)));
            assert!(door_upper(door + DOOR_UPPER) && !door_upper(door) && door_open(door ^ DOOR_OPEN));
            let trap = trap_block(w) + 3 + TRAP_TOP;
            let opened = trap_base(trap) + (trap_bits(trap) ^ TRAP_OPEN);
            assert!(is_trapdoor(trap) && is_trapdoor(opened) && trap_open(opened) && !trap_open(trap));
            assert_eq!((trap_wood(opened), item_of(opened)), (w, wood_item(w, Piece::Trapdoor)));
            let gate = placed_gate(w, 2);
            assert!(is_fence_gate(gate) && is_fence_gate(gate ^ 2) && gate_open(gate ^ 2) && !gate_open(gate));
            assert_eq!((gate_wood(gate ^ 2), item_of(gate)), (w, wood_item(w, Piece::Gate)));
            let sign = sign_block(w, 3);
            assert!(is_sign(sign) && sign_wood(sign) == w && item_of(sign) == wood_item(w, Piece::Sign));
            let (stairs, slab) = (block_of(wood_item(w, Piece::Stairs)), block_of(wood_item(w, Piece::Slab)));
            assert!(is_stairs(stairs) && is_slab(slab) && SHAPE_FULL[slab_mat(slab) as usize] == planks_of(w));
            assert_eq!((item_of(stairs), item_of(slab)), (wood_item(w, Piece::Stairs), wood_item(w, Piece::Slab)));
            for b in [door, trap, gate, sign, stairs, slab] {
                assert!(crate::registry::valid(b), "wood {w} block {b}");
            }
        }
    }
    #[test]
    fn any_planks_make_tools_and_tables_but_each_wood_its_own_pieces() {
        use crate::catalogue as c;
        let (sp, ju) = (cat(c::planks_row(0)), cat(c::planks_row(1)));
        assert_eq!(recipe(&grid([sp, sp, 0, sp, sp, 0, 0, 0, 0]), 2, false).map(|i| RECIPES[i].out), Some((WORKBENCH, 1)));
        assert_eq!(made([ju, ju, ju, 0, STICK, 0, 0, STICK, 0], false), Some((WOOD_PICK, 1)));
        assert_eq!(made([sp, ju, sp, ju, 0, sp, ju, sp, ju], true), Some((CHEST, 1)));
        assert_eq!(made([sp, 0, 0, sp, sp, 0, sp, sp, sp], true), Some((wood_item(2, Piece::Stairs), 4)));
        assert_eq!(made([ju, ju, 0, ju, ju, 0, ju, ju, 0], true), Some((wood_item(3, Piece::Door), 3)));
        assert_eq!(made([ju, ju, ju, ju, ju, ju, 0, 0, 0], true), Some((wood_item(3, Piece::Trapdoor), 2)));
        let log = cat(c::log_row(2));
        assert_eq!(made([0, STICK, 0, STICK, COAL, STICK, log, log, log], true), Some((cat(c::CAMPFIRE), 1)));
        assert_eq!(recipe(&grid([log, 0, 0, 0, 0, 0, 0, 0, 0]), 2, false).map(|i| RECIPES[i].out), Some((cat(c::planks_row(2)), 4)));
        // the recipe book counts and spends any planks too
        let mut inv = Inventory::new();
        inv.add(sp, 5);
        inv.add(ju, 3);
        let chest = RECIPES.iter().find(|r| r.out.0 == CHEST).unwrap();
        assert!(inv.can_craft(chest) && inv.craft(chest));
        assert_eq!((inv.count(sp) + inv.count(ju), inv.count(CHEST)), (0, 1));
        assert_eq!(crate::furnace::smelt(log), Some((COAL, 1)));
    }
    #[test]
    fn grindstones_refund_levels_and_anvils_wear_out() {
        let mut inv = Inventory::new();
        inv.held = Stack { enchants: 2 | 1 << 2, ..Stack::new(IRON_PICK, 1) };
        // efficiency II (3 + 6 levels) and unbreaking I (3): half of 12 back
        assert_eq!(crate::equipment::grind(&mut inv), Ok(6));
        assert_eq!(inv.held.enchants, 0);
        assert!(crate::equipment::grind(&mut inv).is_err());
        use crate::catalogue::{block, CHIPPED_ANVIL, DAMAGED_ANVIL};
        let chipped = crate::equipment::worn_anvil(ANVIL);
        assert_eq!((chipped, crate::equipment::worn_anvil(chipped)), (block(CHIPPED_ANVIL), block(DAMAGED_ANVIL)));
        assert_eq!(crate::equipment::worn_anvil(block(DAMAGED_ANVIL)), AIR);
        assert!(crate::catalogue::is_anvil(chipped) && is_shaped(ANVIL) && is_shaped(chipped));
        assert_eq!(item_of(chipped), cat(CHIPPED_ANVIL));
    }
    #[test]
    fn new_items_are_items_and_keep_minecraft_rules() {
        for i in BOWL..PLAIN_END {
            assert!(valid(i) && is_block(i) == (i == REED_ITEM) && name(i) != "?" && !desc(i).is_empty(), "item {i}");
        }
        assert_eq!(max_stack(SNOWBALL), 16);
        assert_eq!((food(APPLE), food(GOLDEN_CARROT), food(ROTTEN_FLESH)), (4.0, 6.0, 4.0));
        // snow needs a shovel to give its snowballs
        assert_eq!(drop_for(SNOW, STONE_SHOVEL), SNOWBALL);
        assert_eq!(drop_for(SNOW, AIR), AIR);
        assert_eq!(item_of(SNOW), SNOW);
        assert_eq!(crate::furnace::smelt(IRON_SWORD), Some((IRON_NUGGET, 1)));
        assert_eq!(crate::furnace::smelt(GOLD_HELMET), Some((GOLD_NUGGET, 1)));
        assert_eq!(crate::furnace::smelt(STONE_PICK), None);
        assert!(icon(CHEST) != icon(PLANKS) && icon(BED) != icon(CLOTH));
    }
}
