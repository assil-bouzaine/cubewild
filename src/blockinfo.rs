//! Minecraft's block properties for every block: hardness (how long it takes to break),
//! blast resistance, the tool that breaks it fastest, and how long it burns as fuel.
//! Blocks of the Emberdeep take the values of their Nether counterparts.
use crate::building;
use crate::items::Kind;
use crate::world::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Props {
    /// INFINITY: cannot be broken
    pub hardness: f32,
    pub blast: f32,
    pub tool: Option<Kind>,
    /// seconds it burns in a furnace (0: no fuel)
    pub burn: f32,
}

const fn p(hardness: f32, blast: f32, tool: Option<Kind>, burn: f32) -> Props {
    Props { hardness, blast, tool, burn }
}
const PICK: Option<Kind> = Some(Kind::Pick);
const AXE: Option<Kind> = Some(Kind::Axe);
const SHOVEL: Option<Kind> = Some(Kind::Shovel);
const HOE: Option<Kind> = Some(Kind::Hoe);
const NONE: Option<Kind> = None;
const UNBREAKABLE: Props = p(f32::INFINITY, 3_600_000.0, NONE, 0.0);

pub fn props(b: Block) -> Props {
    if let Some(d) = crate::catalogue::def(b) {
        return p(d.hardness, d.blast, d.tool, d.burn);
    }
    if is_slab(b) || is_stairs(b) {
        let mat = SHAPE_FULL[if is_slab(b) { slab_mat(b) } else { stair_mat(b) } as usize];
        let full = props(mat);
        // (wooden slabs burn half as long as a plank)
        return if is_slab(b) { Props { burn: full.burn * 0.5, ..full } } else { full };
    }
    match b {
        AIR | WATER | BEDROCK | GATE => UNBREAKABLE,
        b if is_fluid(b) => UNBREAKABLE,
        GRASS | FARMLAND => p(0.6, 0.6, SHOVEL, 0.0),
        b if is_grassy(b) || crate::farming::moisture(b).is_some() => p(0.6, 0.6, SHOVEL, 0.0),
        DIRT | SAND | RED_SAND => p(0.5, 0.5, SHOVEL, 0.0),
        GRAVEL | CLAY => p(0.6, 0.6, SHOVEL, 0.0),
        SNOW => p(0.2, 0.1, SHOVEL, 0.0),
        STONE | STONE_BRICK => p(1.5, 6.0, PICK, 0.0),
        COBBLE | MOSSY_COBBLE | BRICK => p(2.0, 6.0, PICK, 0.0),
        SANDSTONE => p(0.8, 0.8, PICK, 0.0),
        TERRACOTTA | TERRACOTTA_PALE | TERRACOTTA_DARK => p(1.25, 4.2, PICK, 0.0),
        ICE => p(0.5, 0.5, PICK, 0.0),
        COAL_ORE | IRON_ORE | GOLD_ORE | CRYSTAL_ORE => p(3.0, 3.0, PICK, 0.0),
        GOLD_BLOCK => p(3.0, 6.0, PICK, 0.0),
        IRON_BLOCK => p(5.0, 6.0, PICK, 0.0),
        FURNACE => p(3.5, 3.5, PICK, 0.0),
        ANVIL => p(5.0, 1200.0, PICK, 0.0),
        ENCHANTING_TABLE => p(5.0, 1200.0, PICK, 0.0),
        LOG | BIRCH_LOG => p(2.0, 2.0, AXE, 15.0),
        PLANKS | BIRCH_PLANKS => p(2.0, 3.0, AXE, 15.0),
        WORKBENCH => p(2.5, 2.5, AXE, 15.0),
        b if building::is_chest(b) => p(2.5, 2.5, AXE, 15.0),
        b if is_door(b) => p(3.0, 3.0, AXE, 10.0),
        b if is_trapdoor(b) => p(3.0, 3.0, AXE, 15.0),
        b if is_ladder(b) => p(0.4, 0.4, AXE, 15.0),
        FENCE | BIRCH_FENCE => p(2.0, 3.0, AXE, 15.0),
        b if building::is_fence_gate(b) => p(2.0, 3.0, AXE, 15.0),
        b if building::is_sign(b) => p(1.0, 1.0, AXE, 10.0),
        b if building::is_bed(b) => p(0.2, 0.2, NONE, 0.0),
        b if is_leaves(b) => p(0.2, 0.2, HOE, 0.0),
        b if is_wool(b) || b == CLOTH => p(0.8, 0.8, NONE, 5.0),
        THATCH => p(0.5, 0.5, HOE, 0.0),
        LEATHER => p(0.8, 0.8, NONE, 0.0),
        GLASS | PANE => p(0.3, 0.3, NONE, 0.0),
        LAMP => p(0.3, 0.3, NONE, 0.0),
        CACTUS => p(0.4, 0.4, NONE, 0.0),
        b if is_sapling(b) || b == DEAD_BUSH => p(0.0, 0.0, NONE, 5.0),
        b if is_torch(b) || is_plant(b) || b == REEDS || b == BERRY_BUSH => p(0.0, 0.0, NONE, 0.0),
        // the Emberdeep, after the Nether
        CINDERSTONE => p(50.0, 1200.0, PICK, 0.0),
        ASHSTONE | EMBER_MOSS => p(0.4, 0.4, PICK, 0.0),
        ASH => p(0.5, 0.5, SHOVEL, 0.0),
        SCORIA => p(1.25, 4.2, PICK, 0.0),
        SMOULDER => p(0.5, 0.5, PICK, 0.0),
        CINDERWOOD => p(2.0, 2.0, AXE, 15.0),
        CINDER_PLANKS => p(2.0, 3.0, AXE, 15.0),
        GLOWCAP => p(1.0, 1.0, HOE, 0.0),
        SUNSHARD => p(0.3, 0.3, NONE, 0.0),
        EMBER_ORE => p(3.0, 3.0, PICK, 0.0),
        ASHSTONE_BRICK | FORT_BRICK | FORT_CRACKED => p(2.0, 6.0, PICK, 0.0),
        SLATE => p(3.0, 6.0, PICK, 0.0),
        PYRE_ORE => p(30.0, 1200.0, PICK, 0.0),
        _ => p(0.5, 0.5, NONE, 0.0),
    }
}

/// Minecraft's rule: a block that wants a pickaxe drops nothing without one (ice and
/// a few soft Nether-like blocks aside, which only want it for speed).
pub fn needs_pick(b: Block) -> bool {
    props(b).tool == PICK && b != ICE
}

/// "Pickaxe", "Axe", ... for tooltips
pub fn tool_name(k: Option<Kind>) -> &'static str {
    match k {
        Some(Kind::Pick) => "Pickaxe",
        Some(Kind::Axe) => "Axe",
        Some(Kind::Shovel) => "Shovel",
        Some(Kind::Hoe) => "Hoe",
        Some(Kind::Sword) => "Sword",
        _ => "Any tool",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn familiar_blocks_have_minecraft_values() {
        assert_eq!(props(STONE), p(1.5, 6.0, PICK, 0.0));
        assert_eq!(props(PLANKS).burn, 15.0);
        assert_eq!(props(SLAB).burn, 7.5);
        assert_eq!(props(SLAB + 2).hardness, 2.0);
        assert!(props(BEDROCK).hardness.is_infinite());
        assert_eq!(props(CINDERSTONE).blast, 1200.0);
        assert!(needs_pick(COBBLE) && !needs_pick(DIRT) && !needs_pick(ICE) && !needs_pick(LOG));
        assert_eq!(props(crate::catalogue::block(crate::catalogue::COAL_BLOCK)).burn, 800.0);
    }
    #[test]
    fn mining_takes_minecraft_times_and_stone_needs_a_pickaxe() {
        use crate::items::*;
        let close = |a: f32, b: f32| (a - b).abs() < 1e-4;
        // stone: 7.5 s by hand (and nothing drops), 1.125 s with a wooden pickaxe
        assert!(close(break_time(STONE, AIR), 7.5) && drop_for(STONE, AIR) == AIR);
        assert!(close(break_time(STONE, WOOD_PICK), 1.125) && drop_for(STONE, WOOD_PICK) == COBBLE);
        assert!(close(break_time(STONE, CRYSTAL_PICK), 0.28125));
        // logs: 3 s by hand, 0.5 s with an iron axe; dirt 0.75 s by hand
        assert!(close(break_time(LOG, AIR), 3.0) && close(break_time(LOG, IRON_AXE), 0.5) && drop_for(LOG, AIR) == LOG);
        assert!(close(break_time(DIRT, AIR), 0.75));
        // iron ore wants a stone pickaxe: slow and empty-handed with a wooden one
        assert!(close(break_time(IRON_ORE, WOOD_PICK), 7.5) && drop_for(IRON_ORE, WOOD_PICK) == AIR);
        assert!(break_time(BEDROCK, PYRE_PICK).is_infinite());
        assert!(break_time(TORCH, AIR) <= 0.05 && close(break_time(LEAVES, SHEARS), 0.05));
    }
}
