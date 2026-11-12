//! Shaped crafting and lossless cursor distribution. Recipes can be offset and
//! mirrored, as in Minecraft. One ingredient is consumed from each occupied cell.
use crate::items::{self::*, Kind};
use crate::world::*;

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
            TRAPDOOR => (3, vec![p, p, p, p, p, p]),
            BUCKET => (3, vec![p, 0, p, 0, p, 0]),
            BOAT => (3, vec![p, 0, p, p, p, p]),
            BREAD => (3, vec![p, p, p]),
            ANVIL_ITEM => (3, vec![IRON_BLOCK, IRON_BLOCK, IRON_BLOCK, 0, IRON_INGOT, 0, IRON_INGOT, IRON_INGOT, IRON_INGOT]),
            OAK_GATE | BIRCH_GATE => {
                let wood = r.ins[1].0;
                (3, vec![s, wood, s, s, wood, s])
            }
            SIGN_ITEM => (3, vec![p, p, p, p, p, p, 0, s, 0]),
            FENCE | BIRCH_FENCE => (3, vec![p, s, p, p, s, p]),
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
                                    (if st.is_empty() { AIR } else { st.item }) == expected
                                })
                            })
                        })
                    })
                })
            }
        } else {
            let mut counts = std::collections::BTreeMap::new();
            for st in grid.iter().filter(|s| !s.is_empty()) {
                *counts.entry(st.item).or_insert(0u32) += 1;
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
    fn new_items_are_items_and_keep_minecraft_rules() {
        for i in BOWL..ITEM_END {
            assert!(valid(i) && !is_block(i) && name(i) != "?" && !desc(i).is_empty(), "item {i}");
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
