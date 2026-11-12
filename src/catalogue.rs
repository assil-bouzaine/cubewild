//! The block catalogue: blocks added from Minecraft's block list, one row each, with
//! Minecraft's hardness, blast resistance, tool and burn time. Row `i` is world block
//! `BLOCK0 + i` and item `ITEM0 + i`; both are saved, so rows are only ever appended.
//! Textures are drawn here, procedurally, like every other texture in the game.
use crate::items::{Cat, Item, Kind};
use crate::world::Block;

pub const BLOCK0: Block = 448;
pub const ITEM0: Item = 1024;
/// room reserved in the registry and the item ids
pub const ROOM: usize = 256;
const _: () = assert!(DEFS.len() <= ROOM);

/// How a catalogue block is built: a full cube, or shaped (see `boxes`).
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Shape {
    Full,
    Anvil,
    Stonecutter,
    Grindstone,
    Lantern,
    /// the enchanting table: three quarters high
    Table,
    /// a full cube you see through (stained glass): drawn with the water and glass
    Glass,
    /// a flat pane through the middle of the cell, joining its neighbours (iron bars,
    /// stained glass panes): drawn like the glass pane
    Pane,
    /// a post with arms to its neighbours, too high to jump
    Wall,
    Carpet,
    HangingLantern,
    Campfire,
    /// joins like the oak fence (worked out in `world`)
    Fence,
}

/// A texture: an existing layer, or one drawn here from a pattern and a colour.
#[derive(Clone, Copy)]
pub enum Pat {
    Layer(u32),
    Draw(fn(i32, i32, [u8; 3]) -> [u8; 4], [u8; 3]),
}

#[derive(Clone, Copy)]
pub struct Def {
    pub name: &'static str,
    pub desc: &'static str,
    /// one texture: every face; two: ends and sides; three: top, sides, bottom; four:
    /// top, sides, bottom and a front on the ±Z faces (like the furnace)
    pub tex: &'static [Pat],
    /// a picture for the item, when a face does not show the block well
    pub icon: Option<Pat>,
    pub hardness: f32,
    pub blast: f32,
    pub tool: Option<Kind>,
    /// pickaxe tier needed for it to drop anything (1 any pickaxe ... 4 crystal)
    pub tier: u8,
    /// seconds it burns in a furnace (0: not a fuel)
    pub burn: f32,
    pub shape: Shape,
    pub light: u8,
    pub cat: Cat,
    /// textures of parts that do not wear the block's own (see `part_layer`)
    pub parts: &'static [Pat],
    /// a state of another row's block (a hanging lantern): it has no item of its own and
    /// drops that row's
    pub drop: Option<usize>,
}

const fn d(name: &'static str, desc: &'static str, tex: &'static [Pat], hardness: f32, blast: f32, tool: Option<Kind>, tier: u8, cat: Cat) -> Def {
    Def { name, desc, tex, icon: None, hardness, blast, tool, tier, burn: 0.0, shape: Shape::Full, light: 0, cat, parts: &[], drop: None }
}
const fn burns(mut def: Def, secs: f32) -> Def {
    def.burn = secs;
    def
}
const fn with_parts(mut def: Def, parts: &'static [Pat]) -> Def {
    def.parts = parts;
    def
}
const fn state_of(mut def: Def, row: usize) -> Def {
    def.drop = Some(row);
    def
}
const fn shaped(mut def: Def, shape: Shape, light: u8, icon: Option<Pat>) -> Def {
    def.shape = shape;
    def.light = light;
    def.icon = icon;
    def
}

const PICK: Option<Kind> = Some(Kind::Pick);
const AXE: Option<Kind> = Some(Kind::Axe);
const HOE: Option<Kind> = Some(Kind::Hoe);
const SHOVEL: Option<Kind> = Some(Kind::Shovel);
const B: Cat = Cat::Building;
const U: Cat = Cat::Utility;

// colours
const STONE_C: [u8; 3] = [128, 128, 128];
const SAND_C: [u8; 3] = [220, 206, 150];
const RED_SAND_C: [u8; 3] = [186, 100, 40];
const SLATE_C: [u8; 3] = [72, 72, 80];
const OAK_C: [u8; 3] = [172, 134, 82];
/// Minecraft's terracotta: dyed clay, muted towards brown
const TERRACOTTA_C: [[u8; 3]; 16] = [
    [210, 178, 162], [162, 84, 38], [150, 88, 108], [112, 108, 138], [186, 134, 36], [104, 118, 52], [162, 78, 78], [58, 42, 36],
    [136, 106, 98], [86, 92, 92], [118, 70, 86], [74, 60, 92], [78, 52, 36], [76, 82, 42], [142, 60, 46], [38, 24, 18],
];

macro_rules! terracotta {
    ($($i:literal $name:literal),*) => {
        [$(d($name, "Fired clay, dyed. Eight terracotta round a dye.", &[Pat::Draw(flat, TERRACOTTA_C[$i])], 1.25, 4.2, PICK, 1, B)),*]
    };
}
const TERRACOTTAS: [Def; 16] = terracotta!(0 "White Terracotta", 1 "Orange Terracotta", 2 "Magenta Terracotta", 3 "Light Blue Terracotta",
    4 "Yellow Terracotta", 5 "Lime Terracotta", 6 "Pink Terracotta", 7 "Gray Terracotta", 8 "Light Gray Terracotta", 9 "Cyan Terracotta",
    10 "Purple Terracotta", 11 "Blue Terracotta", 12 "Brown Terracotta", 13 "Green Terracotta", 14 "Red Terracotta", 15 "Black Terracotta");

const HEAD_LEN: usize = 25;
const TAIL_LEN: usize = 11;
const PHASE2_LEN: usize = 4;
const WALLS_LEN: usize = 11;
const OAK_LOG_SIDE: Pat = Pat::Layer(5);
const OAK_LOG_END: Pat = Pat::Layer(6);
use crate::world::COLOURS;

/// Phase 2: the campfire and the hanging state of the lantern, then iron bars.
const PHASE2: [Def; PHASE2_LEN] = [
    state_of(shaped(d("Hanging Lantern", "A lantern hung from the block above.", &[Pat::Draw(lantern, [56, 58, 66])], 3.5, 3.5, PICK, 1, U), Shape::HangingLantern, 15, None), LANTERN),
    with_parts(shaped(d("Campfire", "Cooks up to four raw foods put on it, slowly and with no fuel. Burns whoever stands in it.",
        &[OAK_LOG_END, OAK_LOG_SIDE], 2.0, 2.0, AXE, 0, U), Shape::Campfire, 15, Some(Pat::Draw(campfire_icon, [255, 170, 60]))), &[Pat::Draw(flame, [255, 170, 60])]),
    with_parts(shaped(d("Soul Campfire", "A campfire lit with ash from the Emberdeep: a dimmer, blue flame.",
        &[OAK_LOG_END, OAK_LOG_SIDE], 2.0, 2.0, AXE, 0, U), Shape::Campfire, 10, Some(Pat::Draw(campfire_icon, [90, 210, 230]))), &[Pat::Draw(flame, [90, 210, 230])]),
    shaped(d("Iron Bars", "Six iron ingots make sixteen. A grille that joins the walls beside it.", &[Pat::Draw(iron_bars, [150, 152, 160])], 5.0, 6.0, PICK, 1, B), Shape::Pane, 0, None),
];
const STAINED_GLASS: [Def; 16] = [
    shaped(d("White Stained Glass", "Glass with dye in it. Eight glass round a dye.", &[Pat::Draw(stained, COLOURS[0])], 0.3, 0.3, None, 0, B), Shape::Glass, 0, None),
    shaped(d("Orange Stained Glass", "Glass with dye in it. Eight glass round a dye.", &[Pat::Draw(stained, COLOURS[1])], 0.3, 0.3, None, 0, B), Shape::Glass, 0, None),
    shaped(d("Magenta Stained Glass", "Glass with dye in it. Eight glass round a dye.", &[Pat::Draw(stained, COLOURS[2])], 0.3, 0.3, None, 0, B), Shape::Glass, 0, None),
    shaped(d("Light Blue Stained Glass", "Glass with dye in it. Eight glass round a dye.", &[Pat::Draw(stained, COLOURS[3])], 0.3, 0.3, None, 0, B), Shape::Glass, 0, None),
    shaped(d("Yellow Stained Glass", "Glass with dye in it. Eight glass round a dye.", &[Pat::Draw(stained, COLOURS[4])], 0.3, 0.3, None, 0, B), Shape::Glass, 0, None),
    shaped(d("Lime Stained Glass", "Glass with dye in it. Eight glass round a dye.", &[Pat::Draw(stained, COLOURS[5])], 0.3, 0.3, None, 0, B), Shape::Glass, 0, None),
    shaped(d("Pink Stained Glass", "Glass with dye in it. Eight glass round a dye.", &[Pat::Draw(stained, COLOURS[6])], 0.3, 0.3, None, 0, B), Shape::Glass, 0, None),
    shaped(d("Gray Stained Glass", "Glass with dye in it. Eight glass round a dye.", &[Pat::Draw(stained, COLOURS[7])], 0.3, 0.3, None, 0, B), Shape::Glass, 0, None),
    shaped(d("Light Gray Stained Glass", "Glass with dye in it. Eight glass round a dye.", &[Pat::Draw(stained, COLOURS[8])], 0.3, 0.3, None, 0, B), Shape::Glass, 0, None),
    shaped(d("Cyan Stained Glass", "Glass with dye in it. Eight glass round a dye.", &[Pat::Draw(stained, COLOURS[9])], 0.3, 0.3, None, 0, B), Shape::Glass, 0, None),
    shaped(d("Purple Stained Glass", "Glass with dye in it. Eight glass round a dye.", &[Pat::Draw(stained, COLOURS[10])], 0.3, 0.3, None, 0, B), Shape::Glass, 0, None),
    shaped(d("Blue Stained Glass", "Glass with dye in it. Eight glass round a dye.", &[Pat::Draw(stained, COLOURS[11])], 0.3, 0.3, None, 0, B), Shape::Glass, 0, None),
    shaped(d("Brown Stained Glass", "Glass with dye in it. Eight glass round a dye.", &[Pat::Draw(stained, COLOURS[12])], 0.3, 0.3, None, 0, B), Shape::Glass, 0, None),
    shaped(d("Green Stained Glass", "Glass with dye in it. Eight glass round a dye.", &[Pat::Draw(stained, COLOURS[13])], 0.3, 0.3, None, 0, B), Shape::Glass, 0, None),
    shaped(d("Red Stained Glass", "Glass with dye in it. Eight glass round a dye.", &[Pat::Draw(stained, COLOURS[14])], 0.3, 0.3, None, 0, B), Shape::Glass, 0, None),
    shaped(d("Black Stained Glass", "Glass with dye in it. Eight glass round a dye.", &[Pat::Draw(stained, COLOURS[15])], 0.3, 0.3, None, 0, B), Shape::Glass, 0, None),
];
const STAINED_PANES: [Def; 16] = [
    shaped(d("White Stained Glass Pane", "A thin coloured window that joins the walls beside it.", &[Pat::Draw(stained, COLOURS[0])], 0.3, 0.3, None, 0, B), Shape::Pane, 0, None),
    shaped(d("Orange Stained Glass Pane", "A thin coloured window that joins the walls beside it.", &[Pat::Draw(stained, COLOURS[1])], 0.3, 0.3, None, 0, B), Shape::Pane, 0, None),
    shaped(d("Magenta Stained Glass Pane", "A thin coloured window that joins the walls beside it.", &[Pat::Draw(stained, COLOURS[2])], 0.3, 0.3, None, 0, B), Shape::Pane, 0, None),
    shaped(d("Light Blue Stained Glass Pane", "A thin coloured window that joins the walls beside it.", &[Pat::Draw(stained, COLOURS[3])], 0.3, 0.3, None, 0, B), Shape::Pane, 0, None),
    shaped(d("Yellow Stained Glass Pane", "A thin coloured window that joins the walls beside it.", &[Pat::Draw(stained, COLOURS[4])], 0.3, 0.3, None, 0, B), Shape::Pane, 0, None),
    shaped(d("Lime Stained Glass Pane", "A thin coloured window that joins the walls beside it.", &[Pat::Draw(stained, COLOURS[5])], 0.3, 0.3, None, 0, B), Shape::Pane, 0, None),
    shaped(d("Pink Stained Glass Pane", "A thin coloured window that joins the walls beside it.", &[Pat::Draw(stained, COLOURS[6])], 0.3, 0.3, None, 0, B), Shape::Pane, 0, None),
    shaped(d("Gray Stained Glass Pane", "A thin coloured window that joins the walls beside it.", &[Pat::Draw(stained, COLOURS[7])], 0.3, 0.3, None, 0, B), Shape::Pane, 0, None),
    shaped(d("Light Gray Stained Glass Pane", "A thin coloured window that joins the walls beside it.", &[Pat::Draw(stained, COLOURS[8])], 0.3, 0.3, None, 0, B), Shape::Pane, 0, None),
    shaped(d("Cyan Stained Glass Pane", "A thin coloured window that joins the walls beside it.", &[Pat::Draw(stained, COLOURS[9])], 0.3, 0.3, None, 0, B), Shape::Pane, 0, None),
    shaped(d("Purple Stained Glass Pane", "A thin coloured window that joins the walls beside it.", &[Pat::Draw(stained, COLOURS[10])], 0.3, 0.3, None, 0, B), Shape::Pane, 0, None),
    shaped(d("Blue Stained Glass Pane", "A thin coloured window that joins the walls beside it.", &[Pat::Draw(stained, COLOURS[11])], 0.3, 0.3, None, 0, B), Shape::Pane, 0, None),
    shaped(d("Brown Stained Glass Pane", "A thin coloured window that joins the walls beside it.", &[Pat::Draw(stained, COLOURS[12])], 0.3, 0.3, None, 0, B), Shape::Pane, 0, None),
    shaped(d("Green Stained Glass Pane", "A thin coloured window that joins the walls beside it.", &[Pat::Draw(stained, COLOURS[13])], 0.3, 0.3, None, 0, B), Shape::Pane, 0, None),
    shaped(d("Red Stained Glass Pane", "A thin coloured window that joins the walls beside it.", &[Pat::Draw(stained, COLOURS[14])], 0.3, 0.3, None, 0, B), Shape::Pane, 0, None),
    shaped(d("Black Stained Glass Pane", "A thin coloured window that joins the walls beside it.", &[Pat::Draw(stained, COLOURS[15])], 0.3, 0.3, None, 0, B), Shape::Pane, 0, None),
];
const CARPETS: [Def; 16] = [
    burns(shaped(d("White Carpet", "Two wool make three. A thin covering for floors.", &[Pat::Layer(48)], 0.1, 0.1, None, 0, B), Shape::Carpet, 0, None), 3.35),
    burns(shaped(d("Orange Carpet", "Two wool make three. A thin covering for floors.", &[Pat::Layer(76)], 0.1, 0.1, None, 0, B), Shape::Carpet, 0, None), 3.35),
    burns(shaped(d("Magenta Carpet", "Two wool make three. A thin covering for floors.", &[Pat::Layer(77)], 0.1, 0.1, None, 0, B), Shape::Carpet, 0, None), 3.35),
    burns(shaped(d("Light Blue Carpet", "Two wool make three. A thin covering for floors.", &[Pat::Layer(78)], 0.1, 0.1, None, 0, B), Shape::Carpet, 0, None), 3.35),
    burns(shaped(d("Yellow Carpet", "Two wool make three. A thin covering for floors.", &[Pat::Layer(79)], 0.1, 0.1, None, 0, B), Shape::Carpet, 0, None), 3.35),
    burns(shaped(d("Lime Carpet", "Two wool make three. A thin covering for floors.", &[Pat::Layer(80)], 0.1, 0.1, None, 0, B), Shape::Carpet, 0, None), 3.35),
    burns(shaped(d("Pink Carpet", "Two wool make three. A thin covering for floors.", &[Pat::Layer(81)], 0.1, 0.1, None, 0, B), Shape::Carpet, 0, None), 3.35),
    burns(shaped(d("Gray Carpet", "Two wool make three. A thin covering for floors.", &[Pat::Layer(82)], 0.1, 0.1, None, 0, B), Shape::Carpet, 0, None), 3.35),
    burns(shaped(d("Light Gray Carpet", "Two wool make three. A thin covering for floors.", &[Pat::Layer(83)], 0.1, 0.1, None, 0, B), Shape::Carpet, 0, None), 3.35),
    burns(shaped(d("Cyan Carpet", "Two wool make three. A thin covering for floors.", &[Pat::Layer(84)], 0.1, 0.1, None, 0, B), Shape::Carpet, 0, None), 3.35),
    burns(shaped(d("Purple Carpet", "Two wool make three. A thin covering for floors.", &[Pat::Layer(85)], 0.1, 0.1, None, 0, B), Shape::Carpet, 0, None), 3.35),
    burns(shaped(d("Blue Carpet", "Two wool make three. A thin covering for floors.", &[Pat::Layer(86)], 0.1, 0.1, None, 0, B), Shape::Carpet, 0, None), 3.35),
    burns(shaped(d("Brown Carpet", "Two wool make three. A thin covering for floors.", &[Pat::Layer(87)], 0.1, 0.1, None, 0, B), Shape::Carpet, 0, None), 3.35),
    burns(shaped(d("Green Carpet", "Two wool make three. A thin covering for floors.", &[Pat::Layer(88)], 0.1, 0.1, None, 0, B), Shape::Carpet, 0, None), 3.35),
    burns(shaped(d("Red Carpet", "Two wool make three. A thin covering for floors.", &[Pat::Layer(89)], 0.1, 0.1, None, 0, B), Shape::Carpet, 0, None), 3.35),
    burns(shaped(d("Black Carpet", "Two wool make three. A thin covering for floors.", &[Pat::Layer(90)], 0.1, 0.1, None, 0, B), Shape::Carpet, 0, None), 3.35),
];
const WALLS: [Def; WALLS_LEN] = [
    shaped(d("Cobblestone Wall", "Six blocks make six. Joins its neighbours and is too high to jump.", &[Pat::Layer(9)], 2.0, 6.0, PICK, 1, B), Shape::Wall, 0, None),
    shaped(d("Mossy Cobblestone Wall", "Six blocks make six. Joins its neighbours and is too high to jump.", &[Pat::Layer(47)], 2.0, 6.0, PICK, 1, B), Shape::Wall, 0, None),
    shaped(d("Stone Brick Wall", "Six blocks make six. Joins its neighbours and is too high to jump.", &[Pat::Layer(15)], 1.5, 6.0, PICK, 1, B), Shape::Wall, 0, None),
    shaped(d("Mossy Stone Brick Wall", "Six blocks make six. Joins its neighbours and is too high to jump.", &[Pat::Draw(mossy_bricks, STONE_C)], 1.5, 6.0, PICK, 1, B), Shape::Wall, 0, None),
    shaped(d("Brick Wall", "Six blocks make six. Joins its neighbours and is too high to jump.", &[Pat::Layer(10)], 2.0, 6.0, PICK, 1, B), Shape::Wall, 0, None),
    shaped(d("Sandstone Wall", "Six blocks make six. Joins its neighbours and is too high to jump.", &[Pat::Layer(20)], 0.8, 6.0, PICK, 1, B), Shape::Wall, 0, None),
    shaped(d("Red Sandstone Wall", "Six blocks make six. Joins its neighbours and is too high to jump.", &[Pat::Draw(banded, RED_SAND_C)], 0.8, 6.0, PICK, 1, B), Shape::Wall, 0, None),
    shaped(d("Slate Wall", "Six blocks make six. Joins its neighbours and is too high to jump.", &[Pat::Layer(crate::textures::EXTRA_TEX as u32)], 3.5, 6.0, PICK, 1, B), Shape::Wall, 0, None),
    shaped(d("Polished Slate Wall", "Six blocks make six. Joins its neighbours and is too high to jump.", &[Pat::Draw(smooth, SLATE_C)], 3.5, 6.0, PICK, 1, B), Shape::Wall, 0, None),
    shaped(d("Slate Brick Wall", "Six blocks make six. Joins its neighbours and is too high to jump.", &[Pat::Draw(small_bricks, SLATE_C)], 3.5, 6.0, PICK, 1, B), Shape::Wall, 0, None),
    shaped(d("Slate Tile Wall", "Six blocks make six. Joins its neighbours and is too high to jump.", &[Pat::Draw(tiles, SLATE_C)], 3.5, 6.0, PICK, 1, B), Shape::Wall, 0, None),
];

/// Minecraft's concrete: strong, flat dye colours
const CONCRETE_C: [[u8; 3]; 16] = [
    [208, 214, 215], [224, 98, 0], [168, 48, 158], [36, 136, 198], [240, 174, 22], [94, 168, 24], [212, 100, 142], [54, 58, 62],
    [124, 124, 114], [20, 118, 136], [100, 32, 156], [44, 46, 142], [96, 60, 32], [72, 90, 36], [142, 32, 32], [8, 10, 14],
];
/// the glazes: brighter than the plain terracottas they are fired from
const TERRACOTTA_GLAZE: [[u8; 3]; 16] = [
    [236, 240, 236], [230, 140, 50], [206, 90, 190], [90, 170, 220], [244, 200, 70], [140, 200, 60], [236, 150, 176], [90, 96, 100],
    [170, 176, 176], [50, 140, 150], [120, 50, 170], [50, 70, 160], [130, 90, 60], [90, 120, 50], [180, 50, 44], [36, 30, 36],
];
const CONCRETES: [Def; 16] = [
    d("White Concrete", "Concrete powder set by water. Smooth and bright.", &[Pat::Draw(concrete, CONCRETE_C[0])], 1.8, 1.8, PICK, 1, B),
    d("Orange Concrete", "Concrete powder set by water. Smooth and bright.", &[Pat::Draw(concrete, CONCRETE_C[1])], 1.8, 1.8, PICK, 1, B),
    d("Magenta Concrete", "Concrete powder set by water. Smooth and bright.", &[Pat::Draw(concrete, CONCRETE_C[2])], 1.8, 1.8, PICK, 1, B),
    d("Light Blue Concrete", "Concrete powder set by water. Smooth and bright.", &[Pat::Draw(concrete, CONCRETE_C[3])], 1.8, 1.8, PICK, 1, B),
    d("Yellow Concrete", "Concrete powder set by water. Smooth and bright.", &[Pat::Draw(concrete, CONCRETE_C[4])], 1.8, 1.8, PICK, 1, B),
    d("Lime Concrete", "Concrete powder set by water. Smooth and bright.", &[Pat::Draw(concrete, CONCRETE_C[5])], 1.8, 1.8, PICK, 1, B),
    d("Pink Concrete", "Concrete powder set by water. Smooth and bright.", &[Pat::Draw(concrete, CONCRETE_C[6])], 1.8, 1.8, PICK, 1, B),
    d("Gray Concrete", "Concrete powder set by water. Smooth and bright.", &[Pat::Draw(concrete, CONCRETE_C[7])], 1.8, 1.8, PICK, 1, B),
    d("Light Gray Concrete", "Concrete powder set by water. Smooth and bright.", &[Pat::Draw(concrete, CONCRETE_C[8])], 1.8, 1.8, PICK, 1, B),
    d("Cyan Concrete", "Concrete powder set by water. Smooth and bright.", &[Pat::Draw(concrete, CONCRETE_C[9])], 1.8, 1.8, PICK, 1, B),
    d("Purple Concrete", "Concrete powder set by water. Smooth and bright.", &[Pat::Draw(concrete, CONCRETE_C[10])], 1.8, 1.8, PICK, 1, B),
    d("Blue Concrete", "Concrete powder set by water. Smooth and bright.", &[Pat::Draw(concrete, CONCRETE_C[11])], 1.8, 1.8, PICK, 1, B),
    d("Brown Concrete", "Concrete powder set by water. Smooth and bright.", &[Pat::Draw(concrete, CONCRETE_C[12])], 1.8, 1.8, PICK, 1, B),
    d("Green Concrete", "Concrete powder set by water. Smooth and bright.", &[Pat::Draw(concrete, CONCRETE_C[13])], 1.8, 1.8, PICK, 1, B),
    d("Red Concrete", "Concrete powder set by water. Smooth and bright.", &[Pat::Draw(concrete, CONCRETE_C[14])], 1.8, 1.8, PICK, 1, B),
    d("Black Concrete", "Concrete powder set by water. Smooth and bright.", &[Pat::Draw(concrete, CONCRETE_C[15])], 1.8, 1.8, PICK, 1, B),
];
const POWDERS: [Def; 16] = [
    d("White Concrete Powder", "Four sand, four gravel and a dye make eight. Falls like sand, and sets into concrete where it touches water.", &[Pat::Draw(powder, CONCRETE_C[0])], 0.5, 0.5, SHOVEL, 0, B),
    d("Orange Concrete Powder", "Four sand, four gravel and a dye make eight. Falls like sand, and sets into concrete where it touches water.", &[Pat::Draw(powder, CONCRETE_C[1])], 0.5, 0.5, SHOVEL, 0, B),
    d("Magenta Concrete Powder", "Four sand, four gravel and a dye make eight. Falls like sand, and sets into concrete where it touches water.", &[Pat::Draw(powder, CONCRETE_C[2])], 0.5, 0.5, SHOVEL, 0, B),
    d("Light Blue Concrete Powder", "Four sand, four gravel and a dye make eight. Falls like sand, and sets into concrete where it touches water.", &[Pat::Draw(powder, CONCRETE_C[3])], 0.5, 0.5, SHOVEL, 0, B),
    d("Yellow Concrete Powder", "Four sand, four gravel and a dye make eight. Falls like sand, and sets into concrete where it touches water.", &[Pat::Draw(powder, CONCRETE_C[4])], 0.5, 0.5, SHOVEL, 0, B),
    d("Lime Concrete Powder", "Four sand, four gravel and a dye make eight. Falls like sand, and sets into concrete where it touches water.", &[Pat::Draw(powder, CONCRETE_C[5])], 0.5, 0.5, SHOVEL, 0, B),
    d("Pink Concrete Powder", "Four sand, four gravel and a dye make eight. Falls like sand, and sets into concrete where it touches water.", &[Pat::Draw(powder, CONCRETE_C[6])], 0.5, 0.5, SHOVEL, 0, B),
    d("Gray Concrete Powder", "Four sand, four gravel and a dye make eight. Falls like sand, and sets into concrete where it touches water.", &[Pat::Draw(powder, CONCRETE_C[7])], 0.5, 0.5, SHOVEL, 0, B),
    d("Light Gray Concrete Powder", "Four sand, four gravel and a dye make eight. Falls like sand, and sets into concrete where it touches water.", &[Pat::Draw(powder, CONCRETE_C[8])], 0.5, 0.5, SHOVEL, 0, B),
    d("Cyan Concrete Powder", "Four sand, four gravel and a dye make eight. Falls like sand, and sets into concrete where it touches water.", &[Pat::Draw(powder, CONCRETE_C[9])], 0.5, 0.5, SHOVEL, 0, B),
    d("Purple Concrete Powder", "Four sand, four gravel and a dye make eight. Falls like sand, and sets into concrete where it touches water.", &[Pat::Draw(powder, CONCRETE_C[10])], 0.5, 0.5, SHOVEL, 0, B),
    d("Blue Concrete Powder", "Four sand, four gravel and a dye make eight. Falls like sand, and sets into concrete where it touches water.", &[Pat::Draw(powder, CONCRETE_C[11])], 0.5, 0.5, SHOVEL, 0, B),
    d("Brown Concrete Powder", "Four sand, four gravel and a dye make eight. Falls like sand, and sets into concrete where it touches water.", &[Pat::Draw(powder, CONCRETE_C[12])], 0.5, 0.5, SHOVEL, 0, B),
    d("Green Concrete Powder", "Four sand, four gravel and a dye make eight. Falls like sand, and sets into concrete where it touches water.", &[Pat::Draw(powder, CONCRETE_C[13])], 0.5, 0.5, SHOVEL, 0, B),
    d("Red Concrete Powder", "Four sand, four gravel and a dye make eight. Falls like sand, and sets into concrete where it touches water.", &[Pat::Draw(powder, CONCRETE_C[14])], 0.5, 0.5, SHOVEL, 0, B),
    d("Black Concrete Powder", "Four sand, four gravel and a dye make eight. Falls like sand, and sets into concrete where it touches water.", &[Pat::Draw(powder, CONCRETE_C[15])], 0.5, 0.5, SHOVEL, 0, B),
];
const GLAZED: [Def; 16] = [
    d("White Glazed Terracotta", "White terracotta fired in a furnace.", &[Pat::Draw(glazed, TERRACOTTA_GLAZE[0])], 1.4, 1.4, PICK, 1, B),
    d("Orange Glazed Terracotta", "Orange terracotta fired in a furnace.", &[Pat::Draw(glazed, TERRACOTTA_GLAZE[1])], 1.4, 1.4, PICK, 1, B),
    d("Magenta Glazed Terracotta", "Magenta terracotta fired in a furnace.", &[Pat::Draw(glazed, TERRACOTTA_GLAZE[2])], 1.4, 1.4, PICK, 1, B),
    d("Light Blue Glazed Terracotta", "Light Blue terracotta fired in a furnace.", &[Pat::Draw(glazed, TERRACOTTA_GLAZE[3])], 1.4, 1.4, PICK, 1, B),
    d("Yellow Glazed Terracotta", "Yellow terracotta fired in a furnace.", &[Pat::Draw(glazed, TERRACOTTA_GLAZE[4])], 1.4, 1.4, PICK, 1, B),
    d("Lime Glazed Terracotta", "Lime terracotta fired in a furnace.", &[Pat::Draw(glazed, TERRACOTTA_GLAZE[5])], 1.4, 1.4, PICK, 1, B),
    d("Pink Glazed Terracotta", "Pink terracotta fired in a furnace.", &[Pat::Draw(glazed, TERRACOTTA_GLAZE[6])], 1.4, 1.4, PICK, 1, B),
    d("Gray Glazed Terracotta", "Gray terracotta fired in a furnace.", &[Pat::Draw(glazed, TERRACOTTA_GLAZE[7])], 1.4, 1.4, PICK, 1, B),
    d("Light Gray Glazed Terracotta", "Light Gray terracotta fired in a furnace.", &[Pat::Draw(glazed, TERRACOTTA_GLAZE[8])], 1.4, 1.4, PICK, 1, B),
    d("Cyan Glazed Terracotta", "Cyan terracotta fired in a furnace.", &[Pat::Draw(glazed, TERRACOTTA_GLAZE[9])], 1.4, 1.4, PICK, 1, B),
    d("Purple Glazed Terracotta", "Purple terracotta fired in a furnace.", &[Pat::Draw(glazed, TERRACOTTA_GLAZE[10])], 1.4, 1.4, PICK, 1, B),
    d("Blue Glazed Terracotta", "Blue terracotta fired in a furnace.", &[Pat::Draw(glazed, TERRACOTTA_GLAZE[11])], 1.4, 1.4, PICK, 1, B),
    d("Brown Glazed Terracotta", "Brown terracotta fired in a furnace.", &[Pat::Draw(glazed, TERRACOTTA_GLAZE[12])], 1.4, 1.4, PICK, 1, B),
    d("Green Glazed Terracotta", "Green terracotta fired in a furnace.", &[Pat::Draw(glazed, TERRACOTTA_GLAZE[13])], 1.4, 1.4, PICK, 1, B),
    d("Red Glazed Terracotta", "Red terracotta fired in a furnace.", &[Pat::Draw(glazed, TERRACOTTA_GLAZE[14])], 1.4, 1.4, PICK, 1, B),
    d("Black Glazed Terracotta", "Black terracotta fired in a furnace.", &[Pat::Draw(glazed, TERRACOTTA_GLAZE[15])], 1.4, 1.4, PICK, 1, B),
];

/// Spruce, jungle and acacia: log, planks (whose parts are the door's and trapdoor's
/// textures) and fence each.
const WOODS: [Def; 9] = [
    burns(d("Spruce Log", "Craft into four spruce planks, or burn into coal in a furnace.", &[Pat::Draw(log_end, [114, 84, 50]), Pat::Draw(log_bark, [62, 44, 26])], 2.0, 2.0, AXE, 0, B), 15.0),
    with_parts(burns(d("Spruce Planks", "Four from a spruce log. For spruce stairs, slabs, fences, doors and signs, and anything made of any planks.", &[Pat::Draw(planks, [114, 84, 50])], 2.0, 3.0, AXE, 0, B), 15.0),
        &[Pat::Draw(door_top, [114, 84, 50]), Pat::Draw(door_bottom, [114, 84, 50]), Pat::Draw(door_top_m, [114, 84, 50]), Pat::Draw(door_bottom_m, [114, 84, 50]), Pat::Draw(trapdoor, [114, 84, 50])]),
    shaped(burns(d("Spruce Fence", "Four planks and two sticks make three. Too high to jump.", &[Pat::Draw(planks, [114, 84, 50])], 2.0, 3.0, AXE, 0, B), 15.0), Shape::Fence, 0, Some(Pat::Draw(fence_icon, [114, 84, 50]))),
    burns(d("Jungle Log", "Craft into four jungle planks, or burn into coal in a furnace.", &[Pat::Draw(log_end, [160, 114, 78]), Pat::Draw(log_bark, [88, 70, 30])], 2.0, 2.0, AXE, 0, B), 15.0),
    with_parts(burns(d("Jungle Planks", "Four from a jungle log. For jungle stairs, slabs, fences, doors and signs, and anything made of any planks.", &[Pat::Draw(planks, [160, 114, 78])], 2.0, 3.0, AXE, 0, B), 15.0),
        &[Pat::Draw(door_top, [160, 114, 78]), Pat::Draw(door_bottom, [160, 114, 78]), Pat::Draw(door_top_m, [160, 114, 78]), Pat::Draw(door_bottom_m, [160, 114, 78]), Pat::Draw(trapdoor, [160, 114, 78])]),
    shaped(burns(d("Jungle Fence", "Four planks and two sticks make three. Too high to jump.", &[Pat::Draw(planks, [160, 114, 78])], 2.0, 3.0, AXE, 0, B), 15.0), Shape::Fence, 0, Some(Pat::Draw(fence_icon, [160, 114, 78]))),
    burns(d("Acacia Log", "Craft into four acacia planks, or burn into coal in a furnace.", &[Pat::Draw(log_end, [170, 92, 52]), Pat::Draw(log_bark, [104, 98, 88])], 2.0, 2.0, AXE, 0, B), 15.0),
    with_parts(burns(d("Acacia Planks", "Four from a acacia log. For acacia stairs, slabs, fences, doors and signs, and anything made of any planks.", &[Pat::Draw(planks, [170, 92, 52])], 2.0, 3.0, AXE, 0, B), 15.0),
        &[Pat::Draw(door_top, [170, 92, 52]), Pat::Draw(door_bottom, [170, 92, 52]), Pat::Draw(door_top_m, [170, 92, 52]), Pat::Draw(door_bottom_m, [170, 92, 52]), Pat::Draw(trapdoor, [170, 92, 52])]),
    shaped(burns(d("Acacia Fence", "Four planks and two sticks make three. Too high to jump.", &[Pat::Draw(planks, [170, 92, 52])], 2.0, 3.0, AXE, 0, B), 15.0), Shape::Fence, 0, Some(Pat::Draw(fence_icon, [170, 92, 52]))),
];

/// Joins the groups of rows into one table, at compile time.
const fn concat<const N: usize>(groups: &[&[Def]]) -> [Def; N] {
    let mut all = [d("", "", &[], 0.0, 0.0, None, 0, B); N];
    let (mut n, mut g) = (0, 0);
    while g < groups.len() {
        let mut i = 0;
        while i < groups[g].len() {
            all[n] = groups[g][i];
            n += 1;
            i += 1;
        }
        g += 1;
    }
    assert!(n == N);
    all
}

/// Every catalogue block, in id order. Append only.
pub const DEFS: &[Def] = &{
    let head: [Def; HEAD_LEN] = [
        burns(d("Block of Coal", "Nine coal packed tight. Burns for a long time.", &[Pat::Draw(metal, [44, 44, 48])], 5.0, 6.0, PICK, 1, B), 800.0),
        d("Block of Crystal", "Nine crystals. Shows off your wealth.", &[Pat::Draw(gem_block, [96, 220, 214])], 5.0, 6.0, PICK, 3, B),
        d("Block of Gem", "Nine gems, the villagers' money.", &[Pat::Draw(gem_block, [64, 200, 104])], 5.0, 6.0, PICK, 3, B),
        d("Block of Pyre", "Nine bars of the rarest metal. Lava cannot hurt it.", &[Pat::Draw(metal, [90, 64, 64])], 50.0, 1200.0, PICK, 4, B),
        d("Hay Bale", "Nine wheat, bound. Softens a fall.", &[Pat::Draw(hay_top, [206, 170, 52]), Pat::Draw(hay_side, [206, 170, 52])], 0.5, 0.5, HOE, 0, B),
        d("Bone Block", "Nine bone meal, packed.", &[Pat::Draw(bone_top, [230, 226, 206]), Pat::Draw(bone_side, [230, 226, 206])], 2.0, 2.0, PICK, 1, B),
        burns(d("Bookshelf", "Six planks and three books. Breaks into its books.", &[Pat::Layer(8), Pat::Draw(books, OAK_C)], 1.5, 1.5, AXE, 0, B), 15.0),
        d("Smooth Stone", "Stone smelted a second time.", &[Pat::Draw(smooth, STONE_C)], 2.0, 6.0, PICK, 1, B),
        d("Cracked Stone Bricks", "Stone bricks fired in a furnace.", &[Pat::Draw(cracked_bricks, STONE_C)], 1.5, 6.0, PICK, 1, B),
        d("Chiseled Stone Bricks", "Two stone brick slabs, carved.", &[Pat::Draw(chiseled, STONE_C)], 1.5, 6.0, PICK, 1, B),
        d("Mossy Stone Bricks", "Stone bricks with plant fibre grown over them.", &[Pat::Draw(mossy_bricks, STONE_C)], 1.5, 6.0, PICK, 1, B),
        d("Cut Sandstone", "Four sandstone, cut into panels.", &[Pat::Layer(20), Pat::Draw(cut, SAND_C)], 0.8, 0.8, PICK, 1, B),
        d("Chiseled Sandstone", "Two sandstone slabs, carved.", &[Pat::Layer(20), Pat::Draw(chiseled, SAND_C)], 0.8, 0.8, PICK, 1, B),
        d("Smooth Sandstone", "Sandstone smelted smooth.", &[Pat::Draw(flat, SAND_C)], 2.0, 6.0, PICK, 1, B),
        d("Red Sandstone", "Four red sand, pressed.", &[Pat::Draw(flat, RED_SAND_C), Pat::Draw(banded, RED_SAND_C)], 0.8, 0.8, PICK, 1, B),
        d("Cut Red Sandstone", "Four red sandstone, cut into panels.", &[Pat::Draw(flat, RED_SAND_C), Pat::Draw(cut, RED_SAND_C)], 0.8, 0.8, PICK, 1, B),
        d("Chiseled Red Sandstone", "Red sandstone, carved at the stonecutter.", &[Pat::Draw(flat, RED_SAND_C), Pat::Draw(chiseled, RED_SAND_C)], 0.8, 0.8, PICK, 1, B),
        d("Smooth Red Sandstone", "Red sandstone smelted smooth.", &[Pat::Draw(flat, RED_SAND_C)], 2.0, 6.0, PICK, 1, B),
        d("Packed Ice", "Nine ice, pressed. It does not melt.", &[Pat::Draw(ice, [150, 186, 236])], 0.5, 0.5, PICK, 0, B),
        d("Blue Ice", "Nine packed ice. The slipperiest of all.", &[Pat::Draw(ice, [110, 160, 240])], 2.8, 2.8, PICK, 0, B),
        d("Polished Slate", "Four slate, polished.", &[Pat::Draw(smooth, SLATE_C)], 3.5, 6.0, PICK, 1, B),
        d("Slate Bricks", "Four polished slate, laid as bricks.", &[Pat::Draw(small_bricks, SLATE_C)], 3.5, 6.0, PICK, 1, B),
        d("Slate Tiles", "Four slate bricks, cut into tiles.", &[Pat::Draw(tiles, SLATE_C)], 3.5, 6.0, PICK, 1, B),
        d("Chiseled Slate", "Two slate slabs, carved.", &[Pat::Draw(chiseled, SLATE_C)], 3.5, 6.0, PICK, 1, B),
        d("Cracked Slate Bricks", "Slate bricks fired in a furnace.", &[Pat::Draw(cracked_small_bricks, SLATE_C)], 3.5, 6.0, PICK, 1, B),
    ];
    let tail: [Def; TAIL_LEN] = [
        d("Smoker", "Cooks food twice as fast as a furnace, and nothing else.",
            &[Pat::Draw(smoker_top, [96, 92, 88]), Pat::Draw(smoker_side, [110, 84, 58]), Pat::Layer(9), Pat::Draw(smoker_front, [110, 84, 58])], 3.5, 3.5, PICK, 1, U),
        d("Blast Furnace", "Smelts ore and old metal gear twice as fast as a furnace.",
            &[Pat::Draw(smooth, STONE_C), Pat::Draw(blast_side, [104, 104, 108]), Pat::Draw(smooth, STONE_C), Pat::Draw(blast_front, [104, 104, 108])], 3.5, 3.5, PICK, 1, U),
        burns(d("Smithing Table", "Stand near it to upgrade crystal gear with pyre.",
            &[Pat::Draw(smithing_top, [56, 58, 66]), Pat::Draw(smithing_side, OAK_C), Pat::Layer(8), Pat::Draw(smithing_front, OAK_C)], 2.5, 2.5, AXE, 0, U), 15.0),
        shaped(d("Stonecutter", "Stand near it to cut stone into stairs, slabs and bricks, one for one.",
            &[Pat::Draw(cutter_top, STONE_C), Pat::Draw(cutter_side, STONE_C), Pat::Layer(9)], 3.5, 3.5, PICK, 1, U), Shape::Stonecutter, 0, None),
        shaped(d("Grindstone", "Stand near it to strip enchantments from gear and get some levels back.",
            &[Pat::Draw(grindstone, [136, 136, 136])], 2.0, 6.0, PICK, 1, U), Shape::Grindstone, 0, Some(Pat::Draw(grindstone_icon, [136, 136, 136]))),
        burns(d("Fletching Table", "A fletcher's bench. Villagers' work; it does nothing for you yet.",
            &[Pat::Draw(fletching_top, OAK_C), Pat::Draw(fletching_side, OAK_C), Pat::Layer(8), Pat::Draw(fletching_side, OAK_C)], 2.5, 2.5, AXE, 0, U), 15.0),
        burns(d("Cartography Table", "A mapmaker's desk. Villagers' work; it does nothing for you yet.",
            &[Pat::Draw(cartography_top, [92, 64, 40]), Pat::Draw(cartography_side, [92, 64, 40]), Pat::Layer(8), Pat::Draw(cartography_side, [92, 64, 40])], 2.5, 2.5, AXE, 0, U), 15.0),
        burns(d("Loom", "A weaver's loom. Villagers' work; it does nothing for you yet.",
            &[Pat::Draw(loom_top, OAK_C), Pat::Draw(loom_side, OAK_C), Pat::Layer(8), Pat::Draw(loom_front, OAK_C)], 2.5, 2.5, AXE, 0, U), 15.0),
        shaped(d("Chipped Anvil", "An anvil worn by repairs. It will not last much longer.", &[Pat::Draw(anvil, [70, 70, 74])], 5.0, 1200.0, PICK, 1, U), Shape::Anvil, 0, None),
        shaped(d("Damaged Anvil", "An anvil close to breaking.", &[Pat::Draw(anvil_damaged, [70, 70, 74])], 5.0, 1200.0, PICK, 1, U), Shape::Anvil, 0, None),
        shaped(d("Lantern", "Eight iron nuggets round a torch. A steady light that stands on a block.",
            &[Pat::Draw(lantern, [56, 58, 66])], 3.5, 3.5, PICK, 1, U), Shape::Lantern, 15, Some(Pat::Draw(lantern_icon, [56, 58, 66]))),
    ];
    concat::<{ HEAD_LEN + 16 + TAIL_LEN + PHASE2_LEN + 16 * 3 + WALLS_LEN + 16 * 3 + 9 }>(&[
        &head, &TERRACOTTAS, &tail, &PHASE2, &STAINED_GLASS, &STAINED_PANES, &CARPETS, &WALLS, &CONCRETES, &POWDERS, &GLAZED, &WOODS,
    ])
};

// row numbers, for recipes and behaviour
pub const COAL_BLOCK: usize = 0;
pub const CRYSTAL_BLOCK: usize = 1;
pub const GEM_BLOCK: usize = 2;
pub const PYRE_BLOCK: usize = 3;
pub const HAY_BALE: usize = 4;
pub const BONE_BLOCK: usize = 5;
pub const BOOKSHELF: usize = 6;
pub const SMOOTH_STONE: usize = 7;
pub const CRACKED_STONE_BRICKS: usize = 8;
pub const CHISELED_STONE_BRICKS: usize = 9;
pub const MOSSY_STONE_BRICKS: usize = 10;
pub const CUT_SANDSTONE: usize = 11;
pub const CHISELED_SANDSTONE: usize = 12;
pub const SMOOTH_SANDSTONE: usize = 13;
pub const RED_SANDSTONE: usize = 14;
pub const CUT_RED_SANDSTONE: usize = 15;
pub const CHISELED_RED_SANDSTONE: usize = 16;
pub const SMOOTH_RED_SANDSTONE: usize = 17;
pub const PACKED_ICE: usize = 18;
pub const BLUE_ICE: usize = 19;
pub const POLISHED_SLATE: usize = 20;
pub const SLATE_BRICKS: usize = 21;
pub const SLATE_TILES: usize = 22;
pub const CHISELED_SLATE: usize = 23;
pub const CRACKED_SLATE_BRICKS: usize = 24;
/// + colour (the order of `world::COLOURS`)
pub const TERRACOTTA: usize = 25;
pub const SMOKER: usize = 41;
pub const BLAST_FURNACE: usize = 42;
pub const SMITHING_TABLE: usize = 43;
pub const STONECUTTER: usize = 44;
pub const GRINDSTONE: usize = 45;
pub const FLETCHING_TABLE: usize = 46;
pub const CARTOGRAPHY_TABLE: usize = 47;
pub const LOOM: usize = 48;
pub const CHIPPED_ANVIL: usize = 49;
pub const DAMAGED_ANVIL: usize = 50;
pub const LANTERN: usize = 51;
pub const HANGING_LANTERN: usize = 52;
pub const CAMPFIRE: usize = 53;
pub const SOUL_CAMPFIRE: usize = 54;
pub const IRON_BARS: usize = 55;
/// + colour
pub const STAINED_GLASS_ROW: usize = 56;
pub const STAINED_PANE_ROW: usize = 72;
pub const CARPET_ROW: usize = 88;
/// cobblestone, mossy cobblestone, stone brick, mossy stone brick, brick, sandstone,
/// red sandstone, slate, polished slate, slate brick, slate tile
pub const WALL_ROW: usize = 104;
/// + colour
pub const CONCRETE_ROW: usize = 115;
pub const POWDER_ROW: usize = 131;
pub const GLAZED_ROW: usize = 147;
/// spruce, jungle, acacia: + 3 x wood, then + 0 log, + 1 planks, + 2 fence
pub const WOOD_ROW: usize = 163;
pub const fn log_row(wood: usize) -> usize {
    WOOD_ROW + 3 * wood
}
pub const fn planks_row(wood: usize) -> usize {
    WOOD_ROW + 3 * wood + 1
}
pub const fn fence_row(wood: usize) -> usize {
    WOOD_ROW + 3 * wood + 2
}
pub fn is_wood_log(b: Block) -> bool {
    row_of_block(b).is_some_and(|r| r >= WOOD_ROW && (r - WOOD_ROW) % 3 == 0 && r < WOOD_ROW + 9)
}
pub fn is_fence(b: Block) -> bool {
    shape_is(b, Shape::Fence)
}
/// Door (0 top, 1 bottom, 2 top mirrored, 3 bottom mirrored) and trapdoor (4) layers of
/// a new wood (0 spruce, 1 jungle, 2 acacia).
pub fn wood_part(wood: usize, k: usize) -> u32 {
    LAYOUT.1[planks_row(wood)].2[k]
}

pub const fn item(row: usize) -> Item {
    ITEM0 + row as Item
}
pub const fn block(row: usize) -> Block {
    BLOCK0 + row as Block
}
pub const ITEM_END: Item = ITEM0 + DEFS.len() as Item;

pub fn row_of_block(b: Block) -> Option<usize> {
    let i = b.checked_sub(BLOCK0)? as usize;
    (i < DEFS.len()).then_some(i)
}
pub fn row_of_item(i: Item) -> Option<usize> {
    let r = i.checked_sub(ITEM0)? as usize;
    (r < DEFS.len()).then_some(r)
}
pub fn def(b: Block) -> Option<&'static Def> {
    row_of_block(b).map(|r| &DEFS[r])
}
pub fn is_shaped(b: Block) -> bool {
    def(b).is_some_and(|d| !matches!(d.shape, Shape::Full | Shape::Glass))
}
fn shape_is(b: Block, shape: Shape) -> bool {
    def(b).is_some_and(|d| d.shape == shape)
}
pub fn is_glass(b: Block) -> bool {
    shape_is(b, Shape::Glass)
}
pub fn is_pane(b: Block) -> bool {
    shape_is(b, Shape::Pane)
}
pub fn is_wall(b: Block) -> bool {
    shape_is(b, Shape::Wall)
}
pub fn is_carpet(b: Block) -> bool {
    shape_is(b, Shape::Carpet)
}
pub fn is_campfire(b: Block) -> bool {
    shape_is(b, Shape::Campfire)
}
/// The item a catalogue block stands for (a hanging lantern is a lantern).
pub fn item_of_block(b: Block) -> Option<Item> {
    let r = row_of_block(b)?;
    Some(item(DEFS[r].drop.unwrap_or(r)))
}
/// Rows that are only states of another block have no item.
pub fn has_item(row: usize) -> bool {
    DEFS[row].drop.is_none()
}
pub fn is_anvil(b: Block) -> bool {
    b == crate::world::ANVIL || b == block(CHIPPED_ANVIL) || b == block(DAMAGED_ANVIL)
}

// ---------------------------------------------------------------- shapes

const P: f32 = 1.0 / 16.0;
const fn px(v: [i32; 6]) -> [f32; 6] {
    [v[0] as f32 * P, v[1] as f32 * P, v[2] as f32 * P, v[3] as f32 * P, v[4] as f32 * P, v[5] as f32 * P]
}

/// The boxes of a shaped block (`drawing` adds details that do not collide).
pub fn boxes(shape: Shape, drawing: bool) -> crate::world::Boxes {
    let mut v = [[0.0f32; 6]; 8];
    let list: &[[i32; 6]] = match shape {
        Shape::Full => &[[0, 0, 0, 16, 16, 16]],
        // a broad foot, a narrow waist and the long face along x
        Shape::Anvil => &[[2, 0, 2, 14, 4, 14], [3, 4, 4, 13, 5, 12], [4, 5, 6, 12, 10, 10], [0, 10, 3, 16, 16, 13]],
        Shape::Stonecutter if drawing => &[[0, 0, 0, 16, 9, 16], [1, 9, 7, 15, 15, 8]],
        Shape::Stonecutter => &[[0, 0, 0, 16, 9, 16]],
        // the stone wheel between two wooden legs
        Shape::Grindstone => &[[4, 4, 2, 12, 16, 14], [2, 0, 6, 4, 13, 10], [12, 0, 6, 14, 13, 10]],
        Shape::Lantern => &[[5, 0, 5, 11, 7, 11], [6, 7, 6, 10, 9, 10]],
        Shape::Table => &[[0, 0, 0, 16, 12, 16]],
        Shape::Carpet => &[[0, 0, 0, 16, 1, 16]],
        Shape::HangingLantern if drawing => &[[5, 1, 5, 11, 8, 11], [6, 8, 6, 10, 10, 10], [7, 10, 7, 9, 16, 9]],
        Shape::HangingLantern => &[[5, 1, 5, 11, 10, 11]],
        // two logs along z under two along x, and the flames crossed in the middle
        Shape::Campfire if drawing => {
            &[[1, 0, 0, 5, 4, 16], [11, 0, 0, 15, 4, 16], [0, 3, 1, 16, 7, 5], [0, 3, 11, 16, 7, 15], [3, 1, 7, 13, 15, 9], [7, 1, 3, 9, 15, 13]]
        }
        Shape::Campfire => &[[0, 0, 0, 16, 7, 16]],
        // (glass is a cube, panes and walls are worked out with their neighbours in `world`)
        Shape::Glass | Shape::Pane | Shape::Wall | Shape::Fence => &[[0, 0, 0, 16, 16, 16]],
    };
    for (k, b) in list.iter().enumerate() {
        v[k] = px(*b);
    }
    (v, list.len())
}

/// The texture layer of part `k` of a shaped catalogue block, if not its own.
pub fn part_layer(b: Block, shape: Shape, k: usize) -> Option<u32> {
    match (shape, k) {
        (Shape::Grindstone, 1..) => Some(8),
        (Shape::Stonecutter, 1) => Some(crate::world::CHEST_LATCH_LAYER),
        (Shape::HangingLantern, 2) => Some(crate::textures::NEW_TEX as u32),
        (Shape::Campfire, 4..) => row_of_block(b).and_then(|r| LAYOUT.1[r].2.first().copied()),
        _ => None,
    }
}

// ---------------------------------------------------------------- textures

/// Texture layers drawn here (not existing ones), in order: each row's faces, then its icon.
fn drawn() -> &'static [Pat] {
    &LAYOUT.0
}
/// The drawn patterns, and for every row the layers of its faces, its icon and its parts.
#[allow(clippy::type_complexity)]
static LAYOUT: std::sync::LazyLock<(Vec<Pat>, Vec<(Vec<u32>, Option<u32>, Vec<u32>)>)> = std::sync::LazyLock::new(|| {
    let mut drawn = Vec::new();
    let mut rows = Vec::new();
    let place = |p: Pat, drawn: &mut Vec<Pat>| match p {
        Pat::Layer(l) => l,
        Pat::Draw(..) => {
            drawn.push(p);
            (crate::textures::CAT_TEX + drawn.len() - 1) as u32
        }
    };
    for def in DEFS {
        let faces: Vec<u32> = def.tex.iter().map(|&p| place(p, &mut drawn)).collect();
        let icon = def.icon.map(|p| place(p, &mut drawn));
        let parts: Vec<u32> = def.parts.iter().map(|&p| place(p, &mut drawn)).collect();
        rows.push((faces, icon, parts));
    }
    (drawn, rows)
});
/// how many layers the catalogue draws (counted at compile time for the texture array)
pub const fn layer_count() -> usize {
    let mut n = 0;
    let mut i = 0;
    while i < DEFS.len() {
        let mut j = 0;
        while j < DEFS[i].tex.len() {
            if matches!(DEFS[i].tex[j], Pat::Draw(..)) {
                n += 1;
            }
            j += 1;
        }
        if matches!(DEFS[i].icon, Some(Pat::Draw(..))) {
            n += 1;
        }
        let mut j = 0;
        while j < DEFS[i].parts.len() {
            if matches!(DEFS[i].parts[j], Pat::Draw(..)) {
                n += 1;
            }
            j += 1;
        }
        i += 1;
    }
    n
}

/// Texture layer of face `face` (0 +X, 1 -X, 2 +Y, 3 -Y, 4 +Z, 5 -Z) of a catalogue block.
pub fn tex(b: Block, face: usize) -> u32 {
    let Some(r) = row_of_block(b) else { return 0 };
    let t = &LAYOUT.1[r].0;
    let at = |i: usize| t[i.min(t.len() - 1)];
    match (t.len(), face) {
        (1, _) => at(0),
        (2, 2 | 3) => at(0),
        (2, _) => at(1),
        (_, 2) => at(0),
        (_, 3) => at(2),
        (4.., 4 | 5) => at(3),
        _ => at(1),
    }
}

/// The icon of a catalogue item.
pub fn icon(i: Item) -> u32 {
    let r = row_of_item(i).unwrap();
    LAYOUT.1[r].1.unwrap_or_else(|| tex(block(r), if DEFS[r].tex.len() >= 4 { 4 } else { 0 }))
}

/// One pixel of catalogue layer `k` (`textures::CAT_TEX` + k).
pub fn pixel(k: usize, x: i32, y: i32) -> [u8; 4] {
    match drawn().get(k) {
        Some(&Pat::Draw(f, c)) => f(x, y, c),
        _ => [0, 0, 0, 0],
    }
}

fn hash(x: i32, y: i32, salt: u32) -> f32 {
    let mut v = salt.wrapping_mul(0x9e37_79b9) ^ (x as u32).wrapping_mul(0x85eb_ca6b) ^ (y as u32).wrapping_mul(0xc2b2_ae35).rotate_left(11);
    v ^= v >> 15;
    v = v.wrapping_mul(0x2c1b_3c6d);
    v ^= v >> 12;
    (v & 0xffff) as f32 / 65535.0
}
fn sh(c: [u8; 3], k: f32) -> [u8; 4] {
    let f = |v: u8| (v as f32 * k).clamp(0.0, 255.0) as u8;
    [f(c[0]), f(c[1]), f(c[2]), 255]
}
fn mix(a: [u8; 3], b: [u8; 3], t: f32) -> [u8; 3] {
    let f = |i: usize| (a[i] as f32 + (b[i] as f32 - a[i] as f32) * t) as u8;
    [f(0), f(1), f(2)]
}
const CLEAR: [u8; 4] = [0, 0, 0, 0];
const IRON: [u8; 3] = [196, 198, 206];
const DARK_IRON: [u8; 3] = [60, 62, 70];

/// fine grain over a flat colour
fn flat(x: i32, y: i32, c: [u8; 3]) -> [u8; 4] {
    sh(c, 0.9 + hash(x, y, 1) * 0.12 + hash(x / 3, y / 3, 2) * 0.05)
}
/// flat with a slightly lighter rim and a darker line inside it
fn smooth(x: i32, y: i32, c: [u8; 3]) -> [u8; 4] {
    if x == 0 || y == 0 || x == 15 || y == 15 {
        sh(c, 1.08)
    } else if x == 1 || y == 1 || x == 14 || y == 14 {
        sh(c, 0.82)
    } else {
        flat(x, y, c)
    }
}
/// sandstone-like: soft layers across
fn banded(x: i32, y: i32, c: [u8; 3]) -> [u8; 4] {
    let band = match y {
        0..=2 => 1.08,
        3 => 0.86,
        12 => 0.86,
        13..=15 => 0.95,
        _ => 1.0,
    };
    sh(c, band * (0.93 + hash(x, y, 3) * 0.08))
}
/// two tall panels with a seam between them
fn cut(x: i32, y: i32, c: [u8; 3]) -> [u8; 4] {
    if y == 0 || y == 15 || y == 7 {
        sh(c, 0.8)
    } else if y == 8 || y == 1 {
        sh(c, 1.08)
    } else {
        flat(x, y, c)
    }
}
/// a carved frame round a square boss
fn chiseled(x: i32, y: i32, c: [u8; 3]) -> [u8; 4] {
    let e = x.min(y).min(15 - x).min(15 - y);
    let k = match e {
        0 => 0.8,
        1 => 1.06,
        2 => 0.95,
        3 => 0.78,
        4 => 1.1,
        5 | 6 => 0.96,
        _ if (x + y) % 4 == 0 => 0.84,
        _ => 1.0,
    };
    sh(c, k * (0.94 + hash(x, y, 4) * 0.08))
}
/// stone bricks: two rows of bricks offset by half
fn brick_cell(x: i32, y: i32, h: i32) -> (bool, i32, i32) {
    let row = y / h;
    let off = if row % 2 == 0 { 0 } else { 8 };
    let bx = (x + off) % 16;
    let mortar = y % h == h - 1 || bx % 16 == 15 || (h < 8 && bx % 8 == 7);
    (mortar, row, (x + off) / if h < 8 { 8 } else { 16 })
}
fn bricks_of(x: i32, y: i32, c: [u8; 3], h: i32) -> [u8; 4] {
    let (mortar, row, col) = brick_cell(x, y, h);
    if mortar {
        sh(c, 0.62)
    } else {
        sh(c, (0.9 + hash(col, row, 5) * 0.12) * (0.94 + hash(x, y, 6) * 0.08))
    }
}
fn small_bricks(x: i32, y: i32, c: [u8; 3]) -> [u8; 4] {
    bricks_of(x, y, c, 4)
}
fn tiles(x: i32, y: i32, c: [u8; 3]) -> [u8; 4] {
    if x % 8 == 7 || y % 8 == 7 {
        sh(c, 0.6)
    } else {
        sh(c, (0.88 + hash(x / 8, y / 8, 7) * 0.14) * (0.94 + hash(x, y, 8) * 0.08))
    }
}
/// a jagged crack from top to bottom
fn crack(x: i32, y: i32) -> bool {
    let at = 6 + ((y as f32 * 0.9).sin() * 2.5) as i32 + (y > 9) as i32 * 3;
    x == at || (y % 5 == 2 && x == at + 1)
}
fn cracked_bricks(x: i32, y: i32, c: [u8; 3]) -> [u8; 4] {
    if crack(x, y) { sh(c, 0.45) } else { bricks_of(x, y, c, 8) }
}
fn cracked_small_bricks(x: i32, y: i32, c: [u8; 3]) -> [u8; 4] {
    if crack(x, y) { sh(c, 0.45) } else { bricks_of(x, y, c, 4) }
}
fn mossy_bricks(x: i32, y: i32, c: [u8; 3]) -> [u8; 4] {
    let moss = hash(x / 2, y / 2, 9) + if y < 5 { 0.35 } else { 0.0 } > 0.85;
    if moss { sh([82, 116, 46], 0.85 + hash(x, y, 10) * 0.25) } else { bricks_of(x, y, c, 8) }
}
/// metal and coal blocks: a bevelled plate
fn metal(x: i32, y: i32, c: [u8; 3]) -> [u8; 4] {
    let e = x.min(y).min(15 - x).min(15 - y);
    let k = if e == 0 { if x == 0 || y == 0 { 1.25 } else { 0.7 } } else { 0.95 + hash(x, y, 11) * 0.1 + if hash(x, y, 12) > 0.94 { 0.3 } else { 0.0 } };
    sh(c, k)
}
/// gem blocks: faceted plates with glints
fn gem_block(x: i32, y: i32, c: [u8; 3]) -> [u8; 4] {
    let e = x.min(y).min(15 - x).min(15 - y);
    if e == 0 {
        return sh(c, if x == 0 || y == 0 { 1.2 } else { 0.62 });
    }
    let facet = ((x / 4) + (y / 4)) % 2 == 0;
    let glint = (x + y) % 6 == 0 && (x - y).abs() < 6;
    sh(c, if glint { 1.3 } else if facet { 1.0 } else { 0.84 } + hash(x, y, 13) * 0.06)
}
fn hay_top(x: i32, y: i32, c: [u8; 3]) -> [u8; 4] {
    let r = (((x - 8) * (x - 8) + (y - 8) * (y - 8)) as f32).sqrt();
    if x == 0 || y == 0 || x == 15 || y == 15 {
        sh([120, 82, 40], 1.0)
    } else {
        sh(c, 0.82 + ((r * 1.7).sin() * 0.5 + 0.5) * 0.18 + hash(x, y, 14) * 0.08)
    }
}
fn hay_side(x: i32, y: i32, c: [u8; 3]) -> [u8; 4] {
    if matches!(y, 3 | 4 | 11 | 12) {
        sh([150, 40, 30], 0.85 + hash(x, y, 15) * 0.2)
    } else {
        sh(c, 0.82 + hash(x, 0, 16) * 0.2 + hash(x, y, 17) * 0.06)
    }
}
fn bone_top(x: i32, y: i32, c: [u8; 3]) -> [u8; 4] {
    let r = (((x - 8) * (x - 8) + (y - 8) * (y - 8)) as f32).sqrt();
    if r < 4.5 { sh([200, 190, 160], 0.9 + hash(x, y, 18) * 0.1) } else { sh(c, if r > 7.2 { 0.86 } else { 0.98 }) }
}
fn bone_side(x: i32, y: i32, c: [u8; 3]) -> [u8; 4] {
    sh(c, if x % 5 == 0 { 0.86 } else { 0.95 + hash(x, y, 19) * 0.06 })
}
const BOOK_COLOURS: [[u8; 3]; 6] = [[150, 40, 34], [44, 70, 140], [52, 110, 50], [160, 120, 40], [96, 50, 110], [120, 80, 50]];
fn books(x: i32, y: i32, c: [u8; 3]) -> [u8; 4] {
    // a planked frame with two shelves of books
    if y <= 1 || y >= 14 || y == 7 || y == 8 {
        return sh(c, if y == 1 || y == 8 { 0.7 } else { 0.95 + hash(x, y, 20) * 0.08 });
    }
    let shelf = (y > 7) as i32;
    let spine = (x + shelf * 3) / 2;
    if hash(spine, shelf, 21) < 0.12 {
        return sh([40, 28, 18], 1.0);
    }
    let top = if shelf == 0 { 2 } else { 9 } + (hash(spine, shelf, 22) * 2.0) as i32;
    if y < top {
        sh([40, 28, 18], 1.0)
    } else {
        let col = BOOK_COLOURS[(hash(spine, shelf, 23) * 6.0) as usize % 6];
        sh(col, if (x + shelf * 3) % 2 == 1 { 0.8 } else { 1.0 })
    }
}
fn ice(x: i32, y: i32, c: [u8; 3]) -> [u8; 4] {
    let streak = ((x + y * 2) % 11 == 0) as i32 as f32 * 0.15;
    sh(c, 0.92 + hash(x / 2, y / 2, 24) * 0.1 + streak)
}
fn cobble_like(x: i32, y: i32, c: [u8; 3]) -> [u8; 4] {
    let stone = hash(x / 4 + (y / 4) % 2, y / 4, 25);
    if (x + (y / 4) % 2 * 2) % 4 == 3 || y % 4 == 3 { sh(c, 0.62) } else { sh(c, 0.85 + stone * 0.2 + hash(x, y, 26) * 0.06) }
}
fn smoker_top(x: i32, y: i32, c: [u8; 3]) -> [u8; 4] {
    let e = x.min(y).min(15 - x).min(15 - y);
    if (5..=10).contains(&x) && (5..=10).contains(&y) {
        sh([30, 28, 26], 1.0 + hash(x, y, 27) * 0.2)
    } else if e <= 1 {
        sh(c, 0.75)
    } else {
        cobble_like(x, y, c)
    }
}
fn smoker_side(x: i32, y: i32, c: [u8; 3]) -> [u8; 4] {
    // a log-built body with an iron band
    if y == 3 || y == 12 {
        sh(DARK_IRON, 1.1)
    } else {
        sh(c, 0.85 + hash(x / 2, 0, 28) * 0.15 + hash(x, y, 29) * 0.06)
    }
}
fn smoker_front(x: i32, y: i32, c: [u8; 3]) -> [u8; 4] {
    if (4..=11).contains(&x) && (7..=13).contains(&y) {
        if y == 7 || x == 4 || x == 11 {
            sh(DARK_IRON, 1.0)
        } else {
            sh([40, 30, 24], 1.0 + hash(x, y, 30) * 0.2)
        }
    } else {
        smoker_side(x, y, c)
    }
}
fn blast_side(x: i32, y: i32, c: [u8; 3]) -> [u8; 4] {
    if y <= 1 || y >= 14 || x <= 1 || x >= 14 {
        sh(DARK_IRON, 1.15)
    } else {
        sh(c, 0.88 + hash(x, y, 31) * 0.1 + if (x + y) % 5 == 0 { -0.08 } else { 0.0 })
    }
}
fn blast_front(x: i32, y: i32, c: [u8; 3]) -> [u8; 4] {
    if (4..=11).contains(&x) && (6..=12).contains(&y) {
        if x % 2 == 0 { sh(DARK_IRON, 0.8) } else { sh([36, 32, 34], 1.0) }
    } else {
        blast_side(x, y, c)
    }
}
fn smithing_top(x: i32, y: i32, c: [u8; 3]) -> [u8; 4] {
    let e = x.min(y).min(15 - x).min(15 - y);
    sh(c, if e == 0 { 0.7 } else if e == 1 { 1.2 } else { 0.95 + hash(x, y, 32) * 0.1 })
}
fn smithing_side(x: i32, y: i32, c: [u8; 3]) -> [u8; 4] {
    if y <= 3 {
        sh(DARK_IRON, if y == 3 { 0.8 } else { 1.0 + hash(x, y, 33) * 0.1 })
    } else if x <= 1 || x >= 14 {
        sh(c, 0.72)
    } else {
        sh(c, 0.92 + hash(x, y / 3, 34) * 0.1)
    }
}
fn smithing_front(x: i32, y: i32, c: [u8; 3]) -> [u8; 4] {
    // a hammer hung on the front
    if (6..=9).contains(&x) && (6..=7).contains(&y) || (x == 8 || x == 7) && (8..=12).contains(&y) {
        if y <= 7 { sh(IRON, 0.8) } else { sh([110, 76, 40], 1.0) }
    } else {
        smithing_side(x, y, c)
    }
}
fn cutter_top(x: i32, y: i32, c: [u8; 3]) -> [u8; 4] {
    if (y == 7 || y == 8) && (1..=14).contains(&x) { sh(DARK_IRON, 1.0) } else { smooth(x, y, c) }
}
fn cutter_side(x: i32, y: i32, c: [u8; 3]) -> [u8; 4] {
    // (rows 7.. show: the block is 9/16 high)
    if y == 7 { sh(c, 1.12) } else if y <= 9 { sh(DARK_IRON, 0.9 + hash(x, y, 35) * 0.2) } else { flat(x, y, c) }
}
fn grindstone(x: i32, y: i32, c: [u8; 3]) -> [u8; 4] {
    sh(c, 0.88 + hash(x, y, 36) * 0.12 + if y % 4 == 0 { -0.08 } else { 0.0 })
}
fn grindstone_icon(x: i32, y: i32, c: [u8; 3]) -> [u8; 4] {
    let r = (((x as f32 - 7.5).powi(2) + (y as f32 - 6.5).powi(2)) as f32).sqrt();
    if r < 6.0 {
        sh(c, if r > 5.0 { 0.7 } else if r < 1.5 { 0.6 } else { 0.95 + hash(x, y, 37) * 0.1 })
    } else if (x == 2 || x == 3 || x == 12 || x == 13) && y >= 6 {
        sh(OAK_C, 0.8)
    } else {
        CLEAR
    }
}
fn fletching_top(x: i32, y: i32, c: [u8; 3]) -> [u8; 4] {
    let e = x.min(y).min(15 - x).min(15 - y);
    if e == 0 {
        sh(c, 0.7)
    } else if (x - y).abs() <= 0 && (3..=12).contains(&x) {
        sh([230, 230, 220], 1.0)
    } else {
        sh([220, 206, 160], 0.9 + hash(x, y, 38) * 0.1)
    }
}
fn fletching_side(x: i32, y: i32, c: [u8; 3]) -> [u8; 4] {
    if y <= 2 {
        sh([220, 206, 160], 0.9 + hash(x, y, 39) * 0.1)
    } else if x <= 1 || x >= 14 {
        sh(c, 0.7)
    } else if y >= 6 && (4..=11).contains(&x) && (x + y) % 4 == 0 {
        sh([200, 70, 40], 1.0)
    } else {
        sh(c, 0.92 + hash(x / 4, y, 40) * 0.1)
    }
}
fn cartography_top(x: i32, y: i32, c: [u8; 3]) -> [u8; 4] {
    let e = x.min(y).min(15 - x).min(15 - y);
    if e <= 1 {
        sh(c, if e == 0 { 0.8 } else { 1.0 })
    } else if hash(x / 3, y / 3, 41) > 0.6 {
        sh([110, 150, 80], 0.95)
    } else {
        sh([226, 214, 176], 0.95 + hash(x, y, 42) * 0.06)
    }
}
fn cartography_side(x: i32, y: i32, c: [u8; 3]) -> [u8; 4] {
    if y <= 2 {
        sh([226, 214, 176], 0.92)
    } else if x <= 1 || x >= 14 || y >= 14 {
        sh(c, 0.8)
    } else {
        sh(OAK_C, 0.9 + hash(x, y / 2, 43) * 0.1)
    }
}
fn loom_top(x: i32, y: i32, c: [u8; 3]) -> [u8; 4] {
    if (4..=11).contains(&x) { sh([200, 196, 186], if x % 2 == 0 { 1.0 } else { 0.86 }) } else { sh(c, 0.9 + hash(x, y, 44) * 0.1) }
}
fn loom_side(x: i32, y: i32, c: [u8; 3]) -> [u8; 4] {
    if x <= 1 || x >= 14 || y <= 1 { sh(c, 0.75) } else { sh(c, 0.92 + hash(x, y / 4, 45) * 0.1) }
}
fn loom_front(x: i32, y: i32, c: [u8; 3]) -> [u8; 4] {
    if (3..=12).contains(&x) && (3..=11).contains(&y) {
        sh(mix([200, 196, 186], [60, 90, 160], ((y - 3) as f32 / 8.0).min(1.0) * 0.5), if x % 2 == 0 { 1.0 } else { 0.9 })
    } else {
        loom_side(x, y, c)
    }
}
fn anvil(x: i32, y: i32, c: [u8; 3]) -> [u8; 4] {
    sh(c, 0.92 + hash(x, y, 46) * 0.12 + if (x + y) % 9 == 0 { 0.12 } else { 0.0 })
}
fn anvil_damaged(x: i32, y: i32, c: [u8; 3]) -> [u8; 4] {
    if crack(x, y) || crack(15 - y, x) { sh(c, 0.45) } else { anvil(x, y, c) }
}
/// the lantern's cage and glow, laid out where its boxes sample it
fn lantern(x: i32, y: i32, c: [u8; 3]) -> [u8; 4] {
    let frame = x == 5 || x == 10 || y == 9 || y == 15 || y <= 8;
    if frame || !(5..=10).contains(&x) {
        sh(c, 0.9 + hash(x, y, 47) * 0.2)
    } else {
        sh([255, 206, 110], 0.92 + hash(x, y, 48) * 0.1)
    }
}
/// tinted glass: a slightly stronger rim, a streak of light, and see-through
fn stained(x: i32, y: i32, c: [u8; 3]) -> [u8; 4] {
    let rim = x == 0 || y == 0 || x == 15 || y == 15;
    let streak = (x - y).abs() <= 1 && (3..=8).contains(&x) || (x - y == 6 && (8..=11).contains(&x));
    let k = if rim { 0.8 } else if streak { 1.25 } else { 1.0 };
    let mut px = sh(c, k);
    px[3] = if rim { 210 } else if streak { 150 } else { 110 };
    px
}
/// four upright bars and two rails, open between
fn iron_bars(x: i32, y: i32, c: [u8; 3]) -> [u8; 4] {
    let bar = matches!(x, 1 | 5 | 9 | 13) || matches!(x, 2 | 6 | 10 | 14);
    let rail = matches!(y, 0 | 1 | 14 | 15);
    if bar || rail { sh(c, if matches!(x, 2 | 6 | 10 | 14) { 0.8 } else { 1.0 + hash(x, y, 49) * 0.12 }) } else { CLEAR }
}
/// tongues of flame rising from the bottom of the layer (the flame boxes sample rows 1..15)
fn flame(x: i32, y: i32, c: [u8; 3]) -> [u8; 4] {
    let mid = (x as f32 - 7.5).abs();
    let height = 13.0 - mid * 1.6 + ((x as f32 * 1.7).sin() * 2.0);
    let up = 15 - y;
    if (up as f32) > height || up < 1 {
        return CLEAR;
    }
    let hot = 1.0 - up as f32 / height.max(1.0);
    let core = mix(c, [255, 250, 220], hot * 0.8);
    sh(core, 1.0)
}
fn campfire_icon(x: i32, y: i32, c: [u8; 3]) -> [u8; 4] {
    match y {
        11..=12 if (1..=14).contains(&x) => sh(OAK_C, if y == 11 { 0.9 } else { 0.7 }),
        13..=14 if (2..=5).contains(&x) || (10..=13).contains(&x) => sh([110, 80, 46], 1.0),
        2..=10 => flame(x, y + 5, c),
        _ => CLEAR,
    }
}
fn concrete(x: i32, y: i32, c: [u8; 3]) -> [u8; 4] {
    sh(c, 0.96 + hash(x, y, 50) * 0.06)
}
fn powder(x: i32, y: i32, c: [u8; 3]) -> [u8; 4] {
    let speck = hash(x, y, 51);
    let base = mix(c, [235, 230, 220], 0.22);
    sh(base, if speck > 0.85 { 0.78 } else if speck < 0.12 { 1.12 } else { 0.94 + speck * 0.08 })
}
/// a glazed tile: a pattern of its own for each colour, from a few motifs turned and mixed
fn glazed(x: i32, y: i32, c: [u8; 3]) -> [u8; 4] {
    let seed = (c[0] as u32) * 7 + (c[1] as u32) * 13 + (c[2] as u32) * 31;
    let (u, v) = (x as f32 - 7.5, y as f32 - 7.5);
    let motif = seed % 4;
    let (p, q) = if (seed >> 2) % 2 == 0 { (u, v) } else { (v, -u) };
    let ring = (p.abs().max(q.abs()) * 0.9 + (p + q).abs() * 0.15) as i32;
    let band = match motif {
        0 => (ring % 3 == 0) as i32,
        1 => ((p * 0.6).sin() * (q * 0.6).cos() > 0.25) as i32,
        2 => ((p - q).abs() < 2.0 || (p + q).abs() < 2.0) as i32 + (ring == 7) as i32,
        _ => (((p * p + q * q).sqrt() as i32) % 4 < 2) as i32,
    };
    let accent = mix(c, if c.iter().map(|&v| v as u32).sum::<u32>() > 420 { [60, 70, 90] } else { [240, 236, 220] }, 0.6);
    let edge = x == 0 || y == 0 || x == 15 || y == 15;
    if edge {
        sh(c, 0.75)
    } else if band > 0 {
        sh(accent, 0.95 + hash(x, y, 52) * 0.08)
    } else {
        sh(c, 0.95 + hash(x, y, 53) * 0.08)
    }
}
fn log_bark(x: i32, y: i32, c: [u8; 3]) -> [u8; 4] {
    let ridge = hash(x, 0, 54) > 0.55;
    sh(c, if ridge { 0.8 } else { 1.0 } + hash(x, y / 3, 55) * 0.12 - if (x + y / 4) % 5 == 0 { 0.1 } else { 0.0 })
}
fn log_end(x: i32, y: i32, c: [u8; 3]) -> [u8; 4] {
    let e = x.min(y).min(15 - x).min(15 - y);
    if e == 0 {
        return sh(c, 0.48);
    }
    let r = (((x as f32 - 7.5).powi(2) + (y as f32 - 7.5).powi(2)).sqrt()) as i32;
    sh(c, if r % 3 == 0 { 0.8 } else { 0.98 + hash(x, y, 56) * 0.06 })
}
fn planks(x: i32, y: i32, c: [u8; 3]) -> [u8; 4] {
    let board = y / 4;
    let seam = y % 4 == 3 || (x + board * 5) % 16 == 0;
    if seam { sh(c, 0.7) } else { sh(c, 0.9 + hash(board, x / 5, 57) * 0.12 + hash(x, y, 58) * 0.06) }
}
fn wood_f(c: [u8; 3]) -> [f32; 3] {
    [c[0] as f32, c[1] as f32, c[2] as f32]
}
fn door_top(x: i32, y: i32, c: [u8; 3]) -> [u8; 4] {
    crate::textures::door_pixel(wood_f(c), true, false, 59, x, y)
}
fn door_bottom(x: i32, y: i32, c: [u8; 3]) -> [u8; 4] {
    crate::textures::door_pixel(wood_f(c), false, false, 60, x, y)
}
fn door_top_m(x: i32, y: i32, c: [u8; 3]) -> [u8; 4] {
    crate::textures::door_pixel(wood_f(c), true, true, 59, x, y)
}
fn door_bottom_m(x: i32, y: i32, c: [u8; 3]) -> [u8; 4] {
    crate::textures::door_pixel(wood_f(c), false, true, 60, x, y)
}
fn trapdoor(x: i32, y: i32, c: [u8; 3]) -> [u8; 4] {
    crate::textures::trapdoor_pixel(wood_f(c), x, y)
}
/// a fence for the item: two posts and two rails
fn fence_icon(x: i32, y: i32, c: [u8; 3]) -> [u8; 4] {
    if matches!(x, 2..=4 | 11..=13) && y >= 1 {
        planks(x, y, c)
    } else if matches!(y, 4 | 5 | 10 | 11) {
        sh(c, 0.82)
    } else {
        CLEAR
    }
}
fn lantern_icon(x: i32, y: i32, c: [u8; 3]) -> [u8; 4] {
    match (x, y) {
        (7..=8, 1..=2) | (6..=9, 3..=4) => sh(c, 1.0),
        (4..=11, 5..=14) if x == 4 || x == 11 || y == 5 || y == 14 => sh(c, 0.9),
        (5..=10, 6..=13) => sh([255, 206, 110], 1.0 - (y - 6) as f32 * 0.03),
        _ => CLEAR,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn walls_panes_glass_and_carpets_take_their_shapes() {
        use crate::world::*;
        let wall = block(WALL_ROW);
        let along_x = |dx: i32, _dz: i32| if dx != 0 { wall } else { AIR };
        // a straight run has no post: two arms meeting in the middle, 1.5 high to collide
        let (v, n) = shape_boxes(wall, along_x);
        assert_eq!((n, v[0][4]), (2, 1.5));
        // a corner, or a wall on its own, has its post
        assert_eq!(shape_boxes(wall, |dx, dz| if dx == 1 || dz == 1 { STONE } else { AIR }).1, 3);
        assert_eq!(shape_boxes(wall, |_, _| AIR).1, 1);
        let pane = block(STAINED_PANE_ROW + 3);
        assert!(is_pane(pane) && is_pane(block(IRON_BARS)) && !is_opaque(pane) && is_solid(pane));
        assert_eq!(shape_boxes(pane, |dx, _| if dx != 0 { STONE } else { AIR }).1, 1);
        let glass = block(STAINED_GLASS_ROW + 14);
        assert!(!is_opaque(glass) && !is_shaped(glass) && is_solid(glass));
        let carpet = block(CARPET_ROW + 2);
        assert_eq!(shape_boxes(carpet, |_, _| AIR).0[0][4], 1.0 / 16.0);
        assert!(is_light(block(CAMPFIRE)) && crate::lighting::emission(block(SOUL_CAMPFIRE)) == 10);
        assert_eq!(shape_boxes(block(CAMPFIRE), |_, _| AIR).0[0][4], 7.0 / 16.0);
    }
    #[test]
    fn rows_fit_their_ranges_and_draw_visible_textures() {
        assert!(DEFS.len() <= ROOM);
        assert_eq!(DEFS[TERRACOTTA + 15].name, "Black Terracotta");
        assert_eq!((DEFS[SMOKER].name, DEFS[LANTERN].name), ("Smoker", "Lantern"));
        assert_eq!((DEFS[CAMPFIRE].name, DEFS[IRON_BARS].name), ("Campfire", "Iron Bars"));
        assert_eq!(DEFS[STAINED_GLASS_ROW + 15].name, "Black Stained Glass");
        assert_eq!(DEFS[STAINED_PANE_ROW].name, "White Stained Glass Pane");
        assert_eq!(DEFS[CARPET_ROW + 14].name, "Red Carpet");
        assert_eq!((DEFS[WALL_ROW].name, DEFS[WALL_ROW + 10].name), ("Cobblestone Wall", "Slate Tile Wall"));
        assert_eq!((DEFS[CONCRETE_ROW].name, DEFS[POWDER_ROW + 15].name), ("White Concrete", "Black Concrete Powder"));
        assert_eq!(DEFS[GLAZED_ROW + 15].name, "Black Glazed Terracotta");
        assert_eq!((DEFS[log_row(0)].name, DEFS[planks_row(1)].name, DEFS[fence_row(2)].name), ("Spruce Log", "Jungle Planks", "Acacia Fence"));
        assert_eq!(DEFS.len(), WOOD_ROW + 9);
        assert_eq!(item_of_block(block(HANGING_LANTERN)), Some(item(LANTERN)));
        assert_eq!(drawn().len(), layer_count());
        for (r, def) in DEFS.iter().enumerate() {
            assert!(!def.name.is_empty() && !def.desc.is_empty() && !def.tex.is_empty(), "row {r}");
            assert!(crate::registry::valid(block(r)) && crate::items::valid(item(r)) == has_item(r));
            for f in 0..6 {
                assert!((tex(block(r), f) as usize) < crate::textures::LAYERS);
            }
        }
        for k in 0..layer_count() {
            let shown = (0..256).filter(|i| pixel(k, i % 16, i / 16)[3] > 0).count();
            assert!(shown > 30, "layer {k}");
        }
    }
}
