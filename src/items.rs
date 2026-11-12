//! Items, the slot inventory, crafting recipes, block drops and mining times.

use crate::world::*;

/// Block items share their id with the block. Other items start at 64.
pub type Item = u16;
pub const STICK: Item = 64;
pub const MEAT: Item = 65;
pub const HIDE: Item = 66;
pub const WOOD_PICK: Item = 67;
pub const STONE_PICK: Item = 68;
pub const STONE_AXE: Item = 69;
pub const SPEAR: Item = 70;
pub const COAL: Item = 71;
pub const IRON_INGOT: Item = 72;
pub const CRYSTAL: Item = 73;
pub const COOKED_MEAT: Item = 74;
pub const IRON_PICK: Item = 75;
pub const IRON_AXE: Item = 76;
pub const IRON_SWORD: Item = 77;
pub const CRYSTAL_PICK: Item = 78;
pub const FLINT: Item = 79;
pub const GOLD_INGOT: Item = 80;
pub const FIBER: Item = 81;
pub const BERRIES: Item = 82;
pub const STEW: Item = 83;
pub const LEATHER_CHEST: Item = 84;
pub const IRON_CHEST: Item = 85;
pub const FLINT_KNIFE: Item = 86;
pub const GOLD_PICK: Item = 87;
pub const WOOD_AXE: Item = 88;
pub const GOLD_AXE: Item = 89;
pub const CRYSTAL_AXE: Item = 90;
pub const WOOD_SHOVEL: Item = 91;
pub const STONE_SHOVEL: Item = 92;
pub const IRON_SHOVEL: Item = 93;
pub const GOLD_SHOVEL: Item = 94;
pub const CRYSTAL_SHOVEL: Item = 95;
pub const WOOD_HOE: Item = 96;
pub const STONE_HOE: Item = 97;
pub const IRON_HOE: Item = 98;
pub const WOOD_SWORD: Item = 99;
pub const STONE_SWORD: Item = 100;
pub const GOLD_SWORD: Item = 101;
pub const CRYSTAL_SWORD: Item = 102;
pub const SHEARS: Item = 103;
pub const BUCKET: Item = 104;
pub const WATER_BUCKET: Item = 105;
pub const FISHING_ROD: Item = 106;
pub const RAW_FISH: Item = 107;
pub const COOKED_FISH: Item = 108;
pub const BOW: Item = 109;
pub const ARROW: Item = 110;
pub const COMPASS: Item = 111;
pub const CLOCK: Item = 112;
pub const SEEDS: Item = 113;
pub const WHEAT: Item = 114;
pub const BREAD: Item = 115;
pub const LEATHER_HELMET: Item = 116;
pub const LEATHER_LEGS: Item = 117;
pub const LEATHER_BOOTS: Item = 118;
pub const IRON_HELMET: Item = 119;
pub const IRON_LEGS: Item = 120;
pub const IRON_BOOTS: Item = 121;
pub const GOLD_HELMET: Item = 122;
pub const GOLD_CHEST: Item = 123;
pub const GOLD_LEGS: Item = 124;
pub const GOLD_BOOTS: Item = 125;
pub const CRYSTAL_HELMET: Item = 126;
pub const CRYSTAL_CHEST: Item = 127;
pub const CRYSTAL_LEGS: Item = 128;
pub const CRYSTAL_BOOTS: Item = 129;
pub const OAK_DOOR: Item = 130;
pub const BIRCH_DOOR: Item = 131;
pub const LADDER: Item = 132;
pub const TRAPDOOR: Item = 133;
/// oak, birch, cobblestone, stone brick (the order of `SHAPE_FULL`)
pub const STAIRS: Item = 134;
pub const BOAT: Item = 138;
/// dyes, one per colour of `COLOURS` (+ colour)
pub const DYE: Item = 139;
/// from skeletons: tames wolves, grinds into white dye
pub const BONE: Item = 155;
/// what villagers trade in
pub const GEM: Item = 156;
/// dyed wool (+ colour - 1; white wool is `CLOTH`)
pub const WOOL_ITEM: Item = 157;
/// the blocks of the Emberdeep (+ n of `world::CINDERSTONE` + n)
pub const DEEP_ITEM: Item = 172;
/// lights an Ember Gate
pub const STRIKER: Item = 184;
/// from ember ore: a fuel that burns a long time
pub const EMBER: Item = 185;
/// slate, fortress bricks (whole, cracked), pyre ore (+ n of `world::SLATE` + n)
pub const EXTRA_ITEM: Item = 186;
/// the rare metal of the Emberdeep, and the best gear there is
pub const PYRE_INGOT: Item = 190;
pub const PYRE_PICK: Item = 191;
pub const PYRE_AXE: Item = 192;
pub const PYRE_SHOVEL: Item = 193;
pub const PYRE_SWORD: Item = 194;
pub const PYRE_HELMET: Item = 195;
pub const PYRE_CHEST: Item = 196;
pub const PYRE_LEGS: Item = 197;
pub const PYRE_BOOTS: Item = 198;
/// oak, birch, pine, jungle, acacia saplings (+ n of `world::SAPLING` + n)
pub const SAPLING_ITEM: Item = 199;
pub const LAVA_BUCKET: Item = 204;
pub const ANVIL_ITEM: Item = 205;
pub const ENCHANTING_ITEM: Item = 206;
pub const SHIELD: Item = 207;
pub const CARROT: Item = 208;
pub const POTATO: Item = 209;
pub const BAKED_POTATO: Item = 210;
pub const OAK_GATE: Item = 211;
pub const BIRCH_GATE: Item = 212;
pub const SIGN_ITEM: Item = 213;
pub const EXTRA_SLAB: Item = 214;
pub const EXTRA_STAIRS: Item = 218;
// Minecraft items added from the wiki's item list (see DEVELOPMENT.md); icons from
// `textures::MORE_ICONS` + (item - BOWL).
pub const BOWL: Item = 222;
pub const APPLE: Item = 223;
pub const GOLDEN_APPLE: Item = 224;
pub const IRON_NUGGET: Item = 225;
pub const GOLD_NUGGET: Item = 226;
pub const GOLDEN_CARROT: Item = 227;
pub const BONE_MEAL: Item = 228;
pub const ROTTEN_FLESH: Item = 229;
pub const SNOWBALL: Item = 230;
pub const PAPER: Item = 231;
pub const BOOK: Item = 232;
pub const CLAY_BALL: Item = 233;
/// a fired clay brick (the block of four is `BRICK`)
pub const BRICK_ITEM: Item = 234;
/// reeds (Minecraft's sugar cane): planted by water, made into paper
pub const REED_ITEM: Item = 235;
/// end of the plain items with icons in `textures::MORE_ICONS`
pub const PLAIN_END: Item = 236;
/// the catalogue's block items (`catalogue::ITEM0`..) come last
pub const ITEM_END: Item = crate::catalogue::ITEM_END;
pub const fn cat(row: usize) -> Item {
    crate::catalogue::item(row)
}
/// slots in a chest
pub const CHEST_SLOTS: usize = 27;
/// first texture layer holding item icons
const ICON_BASE: u32 = crate::textures::ICON_BASE as u32;
pub const ICON_HEART: u32 = ICON_BASE + 100;
pub const ICON_FOOD: u32 = ICON_BASE + 101;
pub const ICON_AIR: u32 = ICON_BASE + 102;

pub fn is_block(i: Item) -> bool {
    if crate::catalogue::row_of_item(i).is_some() || i == REED_ITEM {
        return true;
    }
    (OAK_GATE..BOWL).contains(&i) || (ANVIL_ITEM..=ENCHANTING_ITEM).contains(&i) || (i != AIR && i < 64) || is_wool_item(i) || (DEEP_ITEM..DEEP_ITEM + DEEP_BLOCKS).contains(&i) || is_extra_item(i) || is_sapling_item(i)
}

pub fn is_sapling_item(i: Item) -> bool {
    (SAPLING_ITEM..SAPLING_ITEM + 5).contains(&i)
}

pub fn is_extra_item(i: Item) -> bool {
    (EXTRA_ITEM..EXTRA_ITEM + EXTRA_BLOCKS).contains(&i)
}

pub fn is_wool_item(i: Item) -> bool {
    (WOOL_ITEM..WOOL_ITEM + 15).contains(&i)
}

/// The block an item puts down (most block items are their own block).
pub fn slab_material(i:Item)->Option<Block> {if (SLAB..SLAB_END).contains(&i) {Some(i-SLAB)} else if (EXTRA_SLAB..EXTRA_SLAB+4).contains(&i) {Some(4+i-EXTRA_SLAB)} else {None}}
pub fn stair_material(i:Item)->Option<Block> {if (STAIRS..STAIRS+4).contains(&i) {Some(i-STAIRS)} else if (EXTRA_STAIRS..EXTRA_STAIRS+4).contains(&i) {Some(4+i-EXTRA_STAIRS)} else {None}}
pub fn block_of(i: Item) -> Block {
    if let Some(r) = crate::catalogue::row_of_item(i) {
        return crate::catalogue::block(r);
    }
    if i == REED_ITEM {
        return REEDS;
    }
    if let Some(mat)=slab_material(i) {return crate::building::slab(mat,false);}
    if let Some(mat)=stair_material(i) {return crate::building::stairs(mat,0,false);}
    if i==OAK_GATE || i==BIRCH_GATE {return crate::building::FENCE_GATE+(i-OAK_GATE)*4;}
    if i==SIGN_ITEM {return crate::building::SIGN;}
    if i == ANVIL_ITEM { ANVIL } else if i == ENCHANTING_ITEM { ENCHANTING_TABLE } else if is_wool_item(i) {
        WOOL + (i - WOOL_ITEM)
    } else if (DEEP_ITEM..DEEP_ITEM + DEEP_BLOCKS).contains(&i) {
        CINDERSTONE + (i - DEEP_ITEM)
    } else if is_extra_item(i) {
        SLATE + (i - EXTRA_ITEM)
    } else if is_sapling_item(i) {
        SAPLING + (i - SAPLING_ITEM)
    } else {
        i
    }
}

/// The colour of a dye item.
pub fn dye_colour(i: Item) -> Option<u8> {
    (DYE..DYE + 16).contains(&i).then(|| (i - DYE) as u8)
}

pub fn valid(i: Item) -> bool {
    (i >= GRASS && i <= LAST_BLOCK && i != WATER && i != BEDROCK && !(WHEAT_0..=WHEAT_3).contains(&i))
        || (ICE..=FLOWER_BLUE).contains(&i)
        || (64..PLAIN_END).contains(&i)
        || crate::catalogue::row_of_item(i).is_some()
}

pub fn name(i: Item) -> &'static str {
    if let Some(r) = crate::catalogue::row_of_item(i) {
        return crate::catalogue::DEFS[r].name;
    }
    match i {
        PAPER => "Paper",
        BOOK => "Book",
        CLAY_BALL => "Clay Ball",
        BRICK_ITEM => "Brick",
        REED_ITEM => "Reeds",
        OAK_GATE => "Oak Fence Gate", BIRCH_GATE => "Birch Fence Gate", SIGN_ITEM => "Sign",
        BOWL => "Bowl",
        APPLE => "Apple",
        GOLDEN_APPLE => "Golden Apple",
        IRON_NUGGET => "Iron Nugget",
        GOLD_NUGGET => "Gold Nugget",
        GOLDEN_CARROT => "Golden Carrot",
        BONE_MEAL => "Bone Meal",
        ROTTEN_FLESH => "Rotten Flesh",
        SNOWBALL => "Snowball",
        EXTRA_SLAB => "Brick Slab", i if i==EXTRA_SLAB+1 => "Sandstone Slab", i if i==EXTRA_SLAB+2 => "Slate Slab", i if i==EXTRA_SLAB+3 => "Cinderwood Slab",
        EXTRA_STAIRS => "Brick Stairs", i if i==EXTRA_STAIRS+1 => "Sandstone Stairs", i if i==EXTRA_STAIRS+2 => "Slate Stairs", i if i==EXTRA_STAIRS+3 => "Cinderwood Stairs",
        CARROT => "Carrot",
        POTATO => "Potato",
        BAKED_POTATO => "Baked Potato",
        ANVIL_ITEM => "Anvil",
        ENCHANTING_ITEM => "Enchanting Table",
        SHIELD => "Shield",
        GRASS => "Grass Block",
        ICE => "Ice",
        RED_SAND => "Red Sand",
        TERRACOTTA => "Terracotta",
        TERRACOTTA_PALE => "Pale Terracotta",
        TERRACOTTA_DARK => "Dark Terracotta",
        FLOWER_BLUE => "Cornflower",
        DIRT => "Dirt",
        STONE => "Stone",
        SAND => "Sand",
        LOG => "Oak Log",
        LEAVES => "Oak Leaves",
        PLANKS => "Oak Planks",
        COBBLE => "Cobblestone",
        BRICK => "Bricks",
        SNOW => "Snow Block",
        GLASS => "Glass",
        STONE_BRICK => "Stone Bricks",
        LEATHER => "Leather Block",
        COAL_ORE => "Coal Ore",
        IRON_ORE => "Iron Ore",
        CRYSTAL_ORE => "Crystal Ore",
        SANDSTONE => "Sandstone",
        CACTUS => "Cactus",
        WORKBENCH => "Crafting Table",
        FURNACE => "Furnace",
        TORCH => "Torch",
        LAMP => "Crystal Lamp",
        BED => "Bed",
        BIRCH_LOG => "Birch Log",
        BIRCH_LEAVES => "Birch Leaves",
        PINE_LEAVES => "Pine Needles",
        GRAVEL => "Gravel",
        CLAY => "Clay",
        GOLD_ORE => "Gold Ore",
        TALL_GRASS => "Short Grass",
        FLOWER_RED => "Poppy",
        FLOWER_YELLOW => "Dandelion",
        MUSHROOM => "Mushroom",
        BERRY_BUSH => "Berry Bush",
        DEAD_BUSH => "Dead Bush",
        BIRCH_PLANKS => "Birch Planks",
        GOLD_BLOCK => "Gold Block",
        IRON_BLOCK => "Iron Block",
        MOSSY_COBBLE => "Mossy Cobblestone",
        CLOTH => "White Wool",
        THATCH => "Thatch",
        FARMLAND => "Farmland",
        PANE => "Glass Pane",
        SLAB => "Oak Slab",
        51 => "Birch Slab",
        52 => "Cobblestone Slab",
        53 => "Stone Brick Slab",
        FENCE => "Oak Fence",
        BIRCH_FENCE => "Birch Fence",
        CHEST => "Chest",
        LADDER => "Ladder",
        TRAPDOOR => "Trapdoor",
        STAIRS => "Oak Stairs",
        135 => "Birch Stairs",
        136 => "Cobblestone Stairs",
        137 => "Stone Brick Stairs",
        OAK_DOOR => "Oak Door",
        BIRCH_DOOR => "Birch Door",
        STICK => "Stick",
        MEAT => "Raw Meat",
        HIDE => "Hide",
        WOOD_PICK => "Wooden Pickaxe",
        STONE_PICK => "Stone Pickaxe",
        STONE_AXE => "Stone Axe",
        SPEAR => "Spear",
        COAL => "Coal",
        IRON_INGOT => "Iron Ingot",
        CRYSTAL => "Crystal",
        COOKED_MEAT => "Cooked Meat",
        IRON_PICK => "Iron Pickaxe",
        IRON_AXE => "Iron Axe",
        IRON_SWORD => "Iron Sword",
        CRYSTAL_PICK => "Crystal Pickaxe",
        FLINT => "Flint",
        GOLD_INGOT => "Gold Ingot",
        FIBER => "Plant Fibre",
        BERRIES => "Berries",
        STEW => "Forest Stew",
        LEATHER_CHEST => "Leather Tunic",
        IRON_CHEST => "Iron Chestplate",
        FLINT_KNIFE => "Flint Knife",
        GOLD_PICK => "Golden Pickaxe",
        WOOD_AXE => "Wooden Axe",
        GOLD_AXE => "Golden Axe",
        CRYSTAL_AXE => "Crystal Axe",
        WOOD_SHOVEL => "Wooden Shovel",
        STONE_SHOVEL => "Stone Shovel",
        IRON_SHOVEL => "Iron Shovel",
        GOLD_SHOVEL => "Golden Shovel",
        CRYSTAL_SHOVEL => "Crystal Shovel",
        WOOD_HOE => "Wooden Hoe",
        STONE_HOE => "Stone Hoe",
        IRON_HOE => "Iron Hoe",
        WOOD_SWORD => "Wooden Sword",
        STONE_SWORD => "Stone Sword",
        GOLD_SWORD => "Golden Sword",
        CRYSTAL_SWORD => "Crystal Sword",
        SHEARS => "Shears",
        BUCKET => "Bucket",
        WATER_BUCKET => "Water Bucket",
        BOAT => "Boat",
        BONE => "Bone",
        STRIKER => "Fire Striker",
        EMBER => "Ember",
        i if i == DEEP_ITEM => "Cinderstone",
        i if i == DEEP_ITEM + 1 => "Ashstone",
        i if i == DEEP_ITEM + 2 => "Ash",
        i if i == DEEP_ITEM + 3 => "Scoria",
        i if i == DEEP_ITEM + 4 => "Smoulder Rock",
        i if i == DEEP_ITEM + 5 => "Ember Moss",
        i if i == DEEP_ITEM + 6 => "Cinderwood",
        i if i == DEEP_ITEM + 7 => "Glowcap",
        i if i == DEEP_ITEM + 8 => "Cinder Planks",
        i if i == DEEP_ITEM + 9 => "Sunshard",
        i if i == DEEP_ITEM + 10 => "Ember Ore",
        i if i == DEEP_ITEM + 11 => "Ashstone Bricks",
        GEM => "Gem",
        i if i == EXTRA_ITEM => "Deep Slate",
        i if i == EXTRA_ITEM + 1 => "Fortress Bricks",
        i if i == EXTRA_ITEM + 2 => "Cracked Fortress Bricks",
        i if i == EXTRA_ITEM + 3 => "Pyre Ore",
        PYRE_INGOT => "Pyre Ingot",
        i if i == SAPLING_ITEM => "Oak Sapling",
        i if i == SAPLING_ITEM + 1 => "Birch Sapling",
        i if i == SAPLING_ITEM + 2 => "Pine Sapling",
        i if i == SAPLING_ITEM + 3 => "Jungle Sapling",
        i if i == SAPLING_ITEM + 4 => "Acacia Sapling",
        LAVA_BUCKET => "Lava Bucket",
        PYRE_PICK => "Pyre Pickaxe",
        PYRE_AXE => "Pyre Axe",
        PYRE_SHOVEL => "Pyre Shovel",
        PYRE_SWORD => "Pyre Sword",
        PYRE_HELMET => "Pyre Helmet",
        PYRE_CHEST => "Pyre Chestplate",
        PYRE_LEGS => "Pyre Leggings",
        PYRE_BOOTS => "Pyre Boots",
        i if i == DYE + 0 => "White Dye",
        i if i == DYE + 1 => "Orange Dye",
        i if i == DYE + 2 => "Magenta Dye",
        i if i == DYE + 3 => "Light Blue Dye",
        i if i == DYE + 4 => "Yellow Dye",
        i if i == DYE + 5 => "Lime Dye",
        i if i == DYE + 6 => "Pink Dye",
        i if i == DYE + 7 => "Grey Dye",
        i if i == DYE + 8 => "Light Grey Dye",
        i if i == DYE + 9 => "Cyan Dye",
        i if i == DYE + 10 => "Purple Dye",
        i if i == DYE + 11 => "Blue Dye",
        i if i == DYE + 12 => "Brown Dye",
        i if i == DYE + 13 => "Green Dye",
        i if i == DYE + 14 => "Red Dye",
        i if i == DYE + 15 => "Black Dye",
        i if i == WOOL_ITEM + 0 => "Orange Wool",
        i if i == WOOL_ITEM + 1 => "Magenta Wool",
        i if i == WOOL_ITEM + 2 => "Light Blue Wool",
        i if i == WOOL_ITEM + 3 => "Yellow Wool",
        i if i == WOOL_ITEM + 4 => "Lime Wool",
        i if i == WOOL_ITEM + 5 => "Pink Wool",
        i if i == WOOL_ITEM + 6 => "Grey Wool",
        i if i == WOOL_ITEM + 7 => "Light Grey Wool",
        i if i == WOOL_ITEM + 8 => "Cyan Wool",
        i if i == WOOL_ITEM + 9 => "Purple Wool",
        i if i == WOOL_ITEM + 10 => "Blue Wool",
        i if i == WOOL_ITEM + 11 => "Brown Wool",
        i if i == WOOL_ITEM + 12 => "Green Wool",
        i if i == WOOL_ITEM + 13 => "Red Wool",
        i if i == WOOL_ITEM + 14 => "Black Wool",
        FISHING_ROD => "Fishing Rod",
        RAW_FISH => "Raw Fish",
        COOKED_FISH => "Cooked Fish",
        BOW => "Bow",
        ARROW => "Arrow",
        COMPASS => "Compass",
        CLOCK => "Clock",
        SEEDS => "Seeds",
        WHEAT => "Wheat",
        BREAD => "Bread",
        LEATHER_HELMET => "Leather Cap",
        LEATHER_LEGS => "Leather Trousers",
        LEATHER_BOOTS => "Leather Boots",
        IRON_HELMET => "Iron Helmet",
        IRON_LEGS => "Iron Leggings",
        IRON_BOOTS => "Iron Boots",
        GOLD_HELMET => "Golden Helmet",
        GOLD_CHEST => "Golden Chestplate",
        GOLD_LEGS => "Golden Leggings",
        GOLD_BOOTS => "Golden Boots",
        CRYSTAL_HELMET => "Crystal Helmet",
        CRYSTAL_CHEST => "Crystal Chestplate",
        CRYSTAL_LEGS => "Crystal Leggings",
        CRYSTAL_BOOTS => "Crystal Boots",
        _ => "?",
    }
}

/// Texture-array layer used as the item's icon.
pub fn icon(i: Item) -> u32 {
    if crate::catalogue::row_of_item(i).is_some() { return crate::catalogue::icon(i); }
    if (BOWL..PLAIN_END).contains(&i) { return (crate::textures::MORE_ICONS + (i - BOWL) as usize) as u32; }
    if i == CHEST { return CHEST_ICON_LAYER; }
    if i == BED { return BED_ICON_LAYER; }
    if (CARROT..=BAKED_POTATO).contains(&i) { return crate::textures::NEW_TEX as u32 + 12 + (i - CARROT) as u32; }
    if i == SHIELD { return crate::textures::NEW_TEX as u32 + 2; }
    if (SLAB..=BIRCH_FENCE).contains(&i) {
        // a picture of the shape rather than a face of it
        crate::textures::SHAPE_ICONS as u32 + (i - SLAB) as u32
    } else if i == LADDER {
        LADDER_LAYER
    } else if i == TRAPDOOR {
        TRAP_LAYER
    } else if i == STRIKER {
        ICON_BASE + 93
    } else if i == EMBER {
        ICON_BASE + 94
    } else if (PYRE_INGOT..SAPLING_ITEM).contains(&i) {
        crate::textures::PYRE_ICONS as u32 + (i - PYRE_INGOT) as u32
    } else if i == LAVA_BUCKET {
        crate::textures::LAVA_BUCKET_ICON as u32
    } else if is_wool_item(i) || is_extra_item(i) || is_sapling_item(i) || (DEEP_ITEM..DEEP_ITEM + DEEP_BLOCKS).contains(&i) {
        tex_for(block_of(i), if block_of(i) == EMBER_MOSS { 2 } else { 0 })
    } else if is_block(i) {
        tex_for(block_of(i), if i == FURNACE { 4 } else { 0 })
    } else {
        ICON_BASE + (i - 64) as u32
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum Mat {
    Wood,
    Stone,
    Iron,
    Gold,
    Crystal,
    Leather,
    Pyre,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Kind {
    Pick,
    Axe,
    Shovel,
    Hoe,
    Sword,
    Helmet,
    Chest,
    Legs,
    Boots,
}

/// Tools, weapons and armour that come in several materials.
pub fn gear(i: Item) -> Option<(Kind, Mat)> {
    Some(match i {
        WOOD_PICK => (Kind::Pick, Mat::Wood),
        WOOD_AXE => (Kind::Axe, Mat::Wood),
        WOOD_SHOVEL => (Kind::Shovel, Mat::Wood),
        WOOD_HOE => (Kind::Hoe, Mat::Wood),
        WOOD_SWORD => (Kind::Sword, Mat::Wood),
        STONE_PICK => (Kind::Pick, Mat::Stone),
        STONE_AXE => (Kind::Axe, Mat::Stone),
        STONE_SHOVEL => (Kind::Shovel, Mat::Stone),
        STONE_HOE => (Kind::Hoe, Mat::Stone),
        STONE_SWORD => (Kind::Sword, Mat::Stone),
        IRON_PICK => (Kind::Pick, Mat::Iron),
        IRON_AXE => (Kind::Axe, Mat::Iron),
        IRON_SHOVEL => (Kind::Shovel, Mat::Iron),
        IRON_HOE => (Kind::Hoe, Mat::Iron),
        IRON_SWORD => (Kind::Sword, Mat::Iron),
        GOLD_PICK => (Kind::Pick, Mat::Gold),
        GOLD_AXE => (Kind::Axe, Mat::Gold),
        GOLD_SHOVEL => (Kind::Shovel, Mat::Gold),
        GOLD_SWORD => (Kind::Sword, Mat::Gold),
        CRYSTAL_PICK => (Kind::Pick, Mat::Crystal),
        CRYSTAL_AXE => (Kind::Axe, Mat::Crystal),
        CRYSTAL_SHOVEL => (Kind::Shovel, Mat::Crystal),
        CRYSTAL_SWORD => (Kind::Sword, Mat::Crystal),
        LEATHER_HELMET => (Kind::Helmet, Mat::Leather),
        LEATHER_CHEST => (Kind::Chest, Mat::Leather),
        LEATHER_LEGS => (Kind::Legs, Mat::Leather),
        LEATHER_BOOTS => (Kind::Boots, Mat::Leather),
        IRON_HELMET => (Kind::Helmet, Mat::Iron),
        IRON_CHEST => (Kind::Chest, Mat::Iron),
        IRON_LEGS => (Kind::Legs, Mat::Iron),
        IRON_BOOTS => (Kind::Boots, Mat::Iron),
        GOLD_HELMET => (Kind::Helmet, Mat::Gold),
        GOLD_CHEST => (Kind::Chest, Mat::Gold),
        GOLD_LEGS => (Kind::Legs, Mat::Gold),
        GOLD_BOOTS => (Kind::Boots, Mat::Gold),
        CRYSTAL_HELMET => (Kind::Helmet, Mat::Crystal),
        CRYSTAL_CHEST => (Kind::Chest, Mat::Crystal),
        CRYSTAL_LEGS => (Kind::Legs, Mat::Crystal),
        CRYSTAL_BOOTS => (Kind::Boots, Mat::Crystal),
        PYRE_PICK => (Kind::Pick, Mat::Pyre),
        PYRE_AXE => (Kind::Axe, Mat::Pyre),
        PYRE_SHOVEL => (Kind::Shovel, Mat::Pyre),
        PYRE_SWORD => (Kind::Sword, Mat::Pyre),
        PYRE_HELMET => (Kind::Helmet, Mat::Pyre),
        PYRE_CHEST => (Kind::Chest, Mat::Pyre),
        PYRE_LEGS => (Kind::Legs, Mat::Pyre),
        PYRE_BOOTS => (Kind::Boots, Mat::Pyre),
        _ => return None,
    })
}

fn tool(i: Item, kind: Kind) -> Option<Mat> {
    gear(i).filter(|g| g.0 == kind).map(|g| g.1)
}

/// Pickaxe strength: 0 none, 1 wood and gold, 2 stone, 3 iron, 4 crystal.
fn pick_tier(i: Item) -> u8 {
    match tool(i, Kind::Pick) {
        Some(Mat::Wood | Mat::Gold) => 1,
        Some(Mat::Stone) => 2,
        Some(Mat::Iron) => 3,
        Some(Mat::Crystal) => 4,
        Some(Mat::Pyre) => 5,
        _ => 0,
    }
}

/// How many uses an item survives (0 = never wears out).
pub fn max_wear(i: Item) -> u16 {
    if i == SHIELD { return 336; }
    if i == STRIKER {
        return 64;
    }
    match gear(i) {
        Some((Kind::Pick | Kind::Axe | Kind::Shovel | Kind::Hoe | Kind::Sword, m)) => match m {
            Mat::Wood | Mat::Leather => 60,
            Mat::Stone => 130,
            Mat::Iron => 250,
            Mat::Gold => 40,
            Mat::Crystal => 900,
            Mat::Pyre => 2000,
        },
        Some((kind, m)) => {
            let base = match kind { Kind::Helmet => 11, Kind::Chest => 16, Kind::Legs => 15, _ => 13 };
            base * match m { Mat::Leather | Mat::Wood => 5, Mat::Gold => 7, Mat::Iron | Mat::Stone => 15, Mat::Crystal => 33, Mat::Pyre => 37 }
        },
        None => match i {
            SPEAR => 80,
            FLINT_KNIFE => 50,
            SHEARS => 200,
            FISHING_ROD => 64,
            BOW => 200,
            _ => 0,
        },
    }
}

/// Hunger restored by eating an item (0 = not food).
pub fn food(i: Item) -> f32 {
    match i {
        CARROT => 3.0,
        POTATO => 1.0,
        BAKED_POTATO => 5.0,
        MEAT | RAW_FISH => 2.5,
        BERRIES => 1.5,
        BREAD => 5.0,
        COOKED_FISH => 6.0,
        COOKED_MEAT => 7.0,
        STEW => 9.0,
        APPLE | GOLDEN_APPLE | ROTTEN_FLESH => 4.0,
        GOLDEN_CARROT => 6.0,
        _ => 0.0,
    }
}

/// Armour slot (0 head .. 3 feet) and the share of damage the piece absorbs.
pub fn armor(i: Item) -> Option<(usize, f32)> {
    let (kind, mat) = gear(i)?;
    let (slot, share) = match kind {
        Kind::Helmet => (0, 0.15),
        Kind::Chest => (1, 0.4),
        Kind::Legs => (2, 0.3),
        Kind::Boots => (3, 0.15),
        _ => return None,
    };
    let set = match mat {
        Mat::Leather => 0.25,
        Mat::Gold => 0.38,
        Mat::Iron => 0.5,
        Mat::Pyre => 0.8,
        _ => 0.64,
    };
    Some((slot, share * set))
}

pub fn max_stack(i: Item) -> u32 {
    if gear(i).is_some() || max_wear(i) > 0 || matches!(i, BED | BUCKET | WATER_BUCKET | LAVA_BUCKET | COMPASS | CLOCK | BOAT) {
        1
    } else if i == SNOWBALL {
        16
    } else {
        99
    }
}

/// Short type label shown in tooltips.
pub fn kind(i: Item) -> &'static str {
    if let Some(r) = crate::catalogue::row_of_item(i) {
        return if crate::catalogue::DEFS[r].cat == Cat::Utility { "Utility block" } else { "Block" };
    }
    match gear(i) {
        Some((Kind::Sword, _)) => "Weapon",
        Some((Kind::Helmet | Kind::Chest | Kind::Legs | Kind::Boots, _)) => "Armour",
        Some(_) => "Tool",
        None => match i {
            SHIELD => "Shield",
            SPEAR | FLINT_KNIFE | BOW | ARROW | SNOWBALL => "Weapon",
            SHEARS | BUCKET | WATER_BUCKET | LAVA_BUCKET | FISHING_ROD | COMPASS | CLOCK | BOAT => "Tool",
            i if is_sapling_item(i) => "Plant",
            ANVIL_ITEM | ENCHANTING_ITEM | OAK_GATE | BIRCH_GATE | SIGN_ITEM | WORKBENCH | FURNACE | BED | TORCH | LAMP | OAK_DOOR | BIRCH_DOOR | CHEST | LADDER | TRAPDOOR => "Utility block",
            STAIRS..=137 => "Block",
            i if food(i) > 0.0 => "Food",
            i if is_plant(i) => "Plant",
            i if is_block(i) => "Block",
            _ => "Material",
        },
    }
}

/// One line of help for the tooltip.
pub fn desc(i: Item) -> &'static str {
    match gear(i) {
        Some((Kind::Pick, _)) => return "Breaks stone and ore. Better ones mine rarer ores.",
        Some((Kind::Axe, _)) => return "Chops logs and planks quickly. A fair weapon too.",
        Some((Kind::Shovel, _)) => return "Digs dirt, sand, gravel, clay and snow quickly.",
        Some((Kind::Hoe, _)) => return "Right click grass or dirt to till it for seeds.",
        Some((Kind::Sword, _)) => return "Made for fighting.",
        Some(_) => return "Wear it in its armour slot to take less damage.",
        None => {}
    }
    if let Some(r) = crate::catalogue::row_of_item(i) {
        return crate::catalogue::DEFS[r].desc;
    }
    match i {
        PAPER => "Three reeds make three sheets. For books and maps.",
        BOOK => "Three paper and a hide. For bookshelves and enchanting tables.",
        CLAY_BALL => "Smelt it into a brick. Four make a block of clay.",
        BRICK_ITEM => "Four make a block of bricks.",
        REED_ITEM => "Plant it on sand, dirt or grass beside water. Three make paper.",
        GRASS | DIRT => "Soft ground. A shovel digs it fastest.",
        ICE => "Frozen water from the cold seas and rivers.",
        RED_SAND => "The rust-coloured sand of the badlands.",
        TERRACOTTA | TERRACOTTA_PALE | TERRACOTTA_DARK => "Hard baked clay from the banded cliffs of the badlands.",
        STONE => "Smooth stone, smelted from cobblestone.",
        SAND => "Smelt it into glass or pack it into sandstone.",
        LOG | BIRCH_LOG => "Craft into planks, or cook it into coal in a furnace.",
        LEAVES | BIRCH_LEAVES | PINE_LEAVES => "Light foliage. Sometimes hides a stick.",
        PLANKS | BIRCH_PLANKS => "The base of most early crafting.",
        COBBLE => "Rough stone for tools, furnaces and walls.",
        BRICK => "Fired clay. A sturdy building block.",
        SNOW => "Packed snow from the mountain tops.",
        GLASS => "Lets you see out while keeping wolves out.",
        STONE_BRICK | SANDSTONE | MOSSY_COBBLE | THATCH | CLOTH | LEATHER => "A decorative building block.",
        COAL_ORE => "Drops coal when mined.",
        IRON_ORE => "Needs a stone pickaxe. Smelt it in a furnace.",
        CRYSTAL_ORE => "Deep underground. Needs an iron pickaxe.",
        GOLD_ORE => "Deep underground. Needs an iron pickaxe.",
        CACTUS => "A prickly desert plant.",
        WORKBENCH => "Stand near it to craft on a 3 x 3 grid and use advanced recipes.",
        FURNACE => "Smelts ore and cooks food. Needs fuel: coal, wood or sticks.",
        TORCH => "Place it on a block or wall to light the dark.",
        LAMP => "A bright, steady light.",
        BED => "Right click to set your spawn and sleep through the night.",
        GRAVEL => "Loose stones from the sea bed. May hide flint.",
        CLAY => "Found under water. Breaks into four clay balls; smelts into terracotta.",
        TALL_GRASS => "Gives plant fibre and sometimes seeds when cut.",
        FLOWER_RED | FLOWER_YELLOW | FLOWER_BLUE => "Brightens up a home.",
        MUSHROOM => "Grows in the woods. Good in a stew.",
        BERRY_BUSH => "Break it to gather berries.",
        DEAD_BUSH => "Dry twigs. Breaks into sticks.",
        GOLD_BLOCK | IRON_BLOCK => "Solid metal. Shows off your wealth.",
        FARMLAND => "Tilled soil. Plant seeds on it.",
        PANE => "A thin window that joins the walls beside it.",
        OAK_DOOR | BIRCH_DOOR => "Right click to open or close. Monsters cannot open it.",
        SLAB..=53 => "Half a block high. Two of a kind make a full block again.",
        FENCE | BIRCH_FENCE => "Too high to jump: keeps animals in and monsters out.",
        CHEST => "Right click to store 27 stacks. Break it to spill them.",
        LADDER => "Hangs on a wall. Walk into it to climb, sneak to hold on.",
        TRAPDOOR => "A hatch for a hole in the floor. Right click to open or close.",
        STAIRS..=137 => "Walk up without jumping. Place high for upside down.",
        STICK => "Handles for tools, torches and weapons.",
        MEAT | RAW_FISH => "Restores a little hunger. Better cooked.",
        COOKED_MEAT | COOKED_FISH => "A hearty meal.",
        BERRIES => "A quick snack.",
        BREAD => "Baked from three wheat. Filling.",
        STEW => "The best meal there is. You keep the bowl.",
        BOWL => "Holds stew. Made from three planks.",
        APPLE => "Sometimes falls from oak leaves.",
        GOLDEN_APPLE => "Heals as well as it feeds. Can be eaten when full.",
        IRON_NUGGET | GOLD_NUGGET => "Nine make an ingot. Smelt old metal gear to get them back.",
        GOLDEN_CARROT => "A filling, golden snack.",
        BONE_MEAL => "Right click crops, saplings or grass to make them grow. Also white dye.",
        ROTTEN_FLESH => "Dropped by zombies. Edible, but it will likely make you queasy.",
        SNOWBALL => "Right click to throw. Knocks creatures back. Four pack into snow.",
        HIDE => "Animal skin for beds, leather and armour.",
        SPEAR => "A long reach keeps wolves at a distance.",
        FLINT_KNIFE => "Cheap and quick to make.",
        COAL => "Fuel for the furnace and for torches.",
        IRON_INGOT => "Forge it into tools, weapons and armour.",
        GOLD_INGOT => "Soft and shiny. Golden tools are fast but brittle.",
        CRYSTAL => "A rare gem for lamps and the best gear.",
        FLINT => "A sharp stone for knives and arrowheads.",
        FIBER => "For cloth, thatch, bowstrings and fishing line.",
        SHEARS => "Snips leaves and grass in an instant, and keeps the grass.",
        BUCKET => "Right click water or lava to scoop it up.",
        WATER_BUCKET => "Right click to pour the water out.",
        BOAT => "Right click water to put it in. Right click it to get in, Shift to get out.",
        BONE => "Wolves love them. Grinds into bone meal.",
        STRIKER => "Right click a finished cinderstone frame to open an Ember Gate.",
        EMBER => "Burns in a furnace for a very long time.",
        i if i == DEEP_ITEM => "Forms where water meets lava. Build an Ember Gate's frame of it.",
        i if i == DEEP_ITEM + 9 => "Glowing crystal from the roof of the Emberdeep.",
        i if i == DEEP_ITEM + 2 => "Soft ash. It slows you down.",
        i if i == DEEP_ITEM + 4 => "Hot to stand on.",
        GEM => "Villagers trade in these.",
        i if i == EXTRA_ITEM => "The dark rock far below the old bedrock. Rich in ore.",
        i if i == EXTRA_ITEM + 1 || i == EXTRA_ITEM + 2 => "From the ruined fortresses of the Emberdeep.",
        i if i == EXTRA_ITEM + 3 => "Rare, deep in the Emberdeep by the lava. Needs a crystal pickaxe. Smelt it.",
        PYRE_INGOT => "The rarest metal. Forge it into the best gear there is.",
        i if is_sapling_item(i) => "Plant it on grass or dirt and in time a tree grows.",
        LAVA_BUCKET => "Right click to pour it out. Burns in a furnace for a very long time.",
        i if dye_colour(i).is_some() => "Dyes wool and sheep. Mix two to get a new colour.",
        i if is_wool_item(i) => "Soft, coloured building block. Shear sheep or dye white wool.",
        FISHING_ROD => "Hold right click on water and wait for a bite.",
        SHIELD => "Hold right click to raise it and block attacks from the front.",
        BOW => "Hold right click to draw, then release to shoot. A full draw takes one second.",
        ANVIL_ITEM => "Right click to repair gear with experience and matching material.",
        ENCHANTING_ITEM => "Right click to enchant equipment with levels and crystal.",
        CARROT | POTATO => "Eat it, or plant it on farmland near water and light.",
        OAK_GATE | BIRCH_GATE => "Right click to open or close. Keeps animals inside a fence.",
        SIGN_ITEM => "Place to write; look to read; right click to edit.",
        ARROW => "Ammunition for the bow.",
        COMPASS => "Carry it to see your heading and the way to your spawn.",
        CLOCK => "Carry it to see the time of day.",
        SEEDS => "Plant on farmland. Grows into wheat.",
        WHEAT => "Three make a loaf of bread.",
        _ => "",
    }
}

/// Recipe book tabs.
#[derive(Clone, Copy, PartialEq)]
pub enum Cat {
    All,
    Tools,
    Combat,
    Armour,
    Utility,
    Building,
    Materials,
    Food,
}

/// (tab, label, icon item)
pub const CATS: [(Cat, &str, Item); 8] = [
    (Cat::All, "All recipes", WORKBENCH),
    (Cat::Tools, "Tools", STONE_PICK),
    (Cat::Combat, "Weapons", IRON_SWORD),
    (Cat::Armour, "Armour", IRON_CHEST),
    (Cat::Utility, "Utility", TORCH),
    (Cat::Building, "Building blocks", BRICK),
    (Cat::Materials, "Materials", IRON_INGOT),
    (Cat::Food, "Food", COOKED_MEAT),
];

pub struct Recipe {
    pub out: (Item, u32),
    pub ins: &'static [(Item, u32)],
    /// block that must stand within a few steps (AIR = craft anywhere)
    pub station: Block,
    pub cat: Cat,
}

const fn r(cat: Cat, out: (Item, u32), ins: &'static [(Item, u32)], station: Block) -> Recipe {
    Recipe { out, ins, station, cat }
}

use crate::catalogue as c;
const fn cb(row: usize) -> Block {
    c::block(row)
}
const SMITHING: Block = cb(c::SMITHING_TABLE);
pub const STONECUTTER: Block = cb(c::STONECUTTER);
/// the stations whose recipes the recipe book offers (besides crafting anywhere)
pub const STATIONS: [Block; 4] = [WORKBENCH, FURNACE, SMITHING, STONECUTTER];
const fn tc(colour: usize) -> Item {
    cat(c::TERRACOTTA + colour)
}

pub const RECIPES: &[Recipe] = &[
    r(Cat::Materials, (PLANKS, 4), &[(LOG, 1)], AIR),
    r(Cat::Materials, (BIRCH_PLANKS, 4), &[(BIRCH_LOG, 1)], AIR),
    r(Cat::Materials, (STICK, 4), &[(PLANKS, 2)], AIR),
    r(Cat::Materials, (STICK, 4), &[(BIRCH_PLANKS, 2)], AIR),
    r(Cat::Materials, (FLINT, 1), &[(GRAVEL, 3)], AIR),
    r(Cat::Utility, (WORKBENCH, 1), &[(PLANKS, 4)], AIR),
    r(Cat::Utility, (TORCH, 4), &[(COAL, 1), (STICK, 1)], AIR),
    r(Cat::Utility, (FURNACE, 1), &[(COBBLE, 8)], WORKBENCH),
    r(Cat::Utility, (BED, 1), &[(CLOTH, 3), (PLANKS, 3)], WORKBENCH),
    r(Cat::Utility, (BED, 1), &[(CLOTH, 3), (BIRCH_PLANKS, 3)], WORKBENCH),
    r(Cat::Utility, (BED, 1), &[(WOOL_ITEM + 13, 3), (PLANKS, 3)], WORKBENCH),
    r(Cat::Utility, (BED, 1), &[(PLANKS, 3), (HIDE, 3)], WORKBENCH),
    r(Cat::Utility, (LAMP, 1), &[(CRYSTAL, 1), (GLASS, 1)], WORKBENCH),
    r(Cat::Utility, (CHEST, 1), &[(PLANKS, 8)], WORKBENCH),
    r(Cat::Utility, (CHEST, 1), &[(BIRCH_PLANKS, 8)], WORKBENCH),
    r(Cat::Utility, (LADDER, 3), &[(STICK, 7)], WORKBENCH),
    r(Cat::Utility, (TRAPDOOR, 2), &[(PLANKS, 6)], WORKBENCH),
    r(Cat::Utility, (BUCKET, 1), &[(IRON_INGOT, 3)], WORKBENCH),
    r(Cat::Utility, (BOAT, 1), &[(PLANKS, 5)], WORKBENCH),
    r(Cat::Utility, (STRIKER, 1), &[(FLINT, 1), (IRON_INGOT, 1)], AIR),
    r(Cat::Materials, (DEEP_ITEM + 8, 4), &[(DEEP_ITEM + 6, 1)], AIR),
    r(Cat::Materials, (STICK, 4), &[(DEEP_ITEM + 8, 2)], AIR),
    r(Cat::Building, (DEEP_ITEM + 11, 4), &[(DEEP_ITEM + 1, 4)], WORKBENCH),
    r(Cat::Utility, (LAMP, 1), &[(DEEP_ITEM + 9, 2), (GLASS, 1)], WORKBENCH),
    r(Cat::Materials, (DYE + 14, 2), &[(FLOWER_RED, 1)], AIR),
    r(Cat::Materials, (DYE + 4, 2), &[(FLOWER_YELLOW, 1)], AIR),
    r(Cat::Materials, (DYE + 11, 2), &[(FLOWER_BLUE, 1)], AIR),
    r(Cat::Materials, (DYE + 15, 1), &[(COAL, 1)], AIR),
    r(Cat::Materials, (BONE_MEAL, 3), &[(BONE, 1)], AIR),
    r(Cat::Materials, (DYE + 0, 1), &[(BONE_MEAL, 1)], AIR),
    r(Cat::Materials, (IRON_NUGGET, 9), &[(IRON_INGOT, 1)], AIR),
    r(Cat::Materials, (IRON_INGOT, 1), &[(IRON_NUGGET, 9)], WORKBENCH),
    r(Cat::Materials, (GOLD_NUGGET, 9), &[(GOLD_INGOT, 1)], AIR),
    r(Cat::Materials, (GOLD_INGOT, 1), &[(GOLD_NUGGET, 9)], WORKBENCH),
    r(Cat::Building, (SNOW, 1), &[(SNOWBALL, 4)], AIR),
    r(Cat::Utility, (BOWL, 4), &[(PLANKS, 3)], WORKBENCH),
    r(Cat::Utility, (BOWL, 4), &[(BIRCH_PLANKS, 3)], WORKBENCH),
    r(Cat::Materials, (DYE + 13, 1), &[(CACTUS, 1)], AIR),
    r(Cat::Materials, (DYE + 12, 1), &[(MUSHROOM, 1)], AIR),
    r(Cat::Materials, (DYE + 1, 2), &[(DYE + 14, 1), (DYE + 4, 1)], AIR),
    r(Cat::Materials, (DYE + 10, 2), &[(DYE + 14, 1), (DYE + 11, 1)], AIR),
    r(Cat::Materials, (DYE + 6, 2), &[(DYE + 14, 1), (DYE + 0, 1)], AIR),
    r(Cat::Materials, (DYE + 3, 2), &[(DYE + 11, 1), (DYE + 0, 1)], AIR),
    r(Cat::Materials, (DYE + 5, 2), &[(DYE + 13, 1), (DYE + 0, 1)], AIR),
    r(Cat::Materials, (DYE + 9, 2), &[(DYE + 11, 1), (DYE + 13, 1)], AIR),
    r(Cat::Materials, (DYE + 7, 2), &[(DYE + 15, 1), (DYE + 0, 1)], AIR),
    r(Cat::Materials, (DYE + 8, 2), &[(DYE + 7, 1), (DYE + 0, 1)], AIR),
    r(Cat::Materials, (DYE + 2, 2), &[(DYE + 10, 1), (DYE + 6, 1)], AIR),
    r(Cat::Building, (WOOL_ITEM + 0, 1), &[(CLOTH, 1), (DYE + 1, 1)], AIR),
    r(Cat::Building, (WOOL_ITEM + 1, 1), &[(CLOTH, 1), (DYE + 2, 1)], AIR),
    r(Cat::Building, (WOOL_ITEM + 2, 1), &[(CLOTH, 1), (DYE + 3, 1)], AIR),
    r(Cat::Building, (WOOL_ITEM + 3, 1), &[(CLOTH, 1), (DYE + 4, 1)], AIR),
    r(Cat::Building, (WOOL_ITEM + 4, 1), &[(CLOTH, 1), (DYE + 5, 1)], AIR),
    r(Cat::Building, (WOOL_ITEM + 5, 1), &[(CLOTH, 1), (DYE + 6, 1)], AIR),
    r(Cat::Building, (WOOL_ITEM + 6, 1), &[(CLOTH, 1), (DYE + 7, 1)], AIR),
    r(Cat::Building, (WOOL_ITEM + 7, 1), &[(CLOTH, 1), (DYE + 8, 1)], AIR),
    r(Cat::Building, (WOOL_ITEM + 8, 1), &[(CLOTH, 1), (DYE + 9, 1)], AIR),
    r(Cat::Building, (WOOL_ITEM + 9, 1), &[(CLOTH, 1), (DYE + 10, 1)], AIR),
    r(Cat::Building, (WOOL_ITEM + 10, 1), &[(CLOTH, 1), (DYE + 11, 1)], AIR),
    r(Cat::Building, (WOOL_ITEM + 11, 1), &[(CLOTH, 1), (DYE + 12, 1)], AIR),
    r(Cat::Building, (WOOL_ITEM + 12, 1), &[(CLOTH, 1), (DYE + 13, 1)], AIR),
    r(Cat::Building, (WOOL_ITEM + 13, 1), &[(CLOTH, 1), (DYE + 14, 1)], AIR),
    r(Cat::Building, (WOOL_ITEM + 14, 1), &[(CLOTH, 1), (DYE + 15, 1)], AIR),
    r(Cat::Utility, (BOAT, 1), &[(BIRCH_PLANKS, 5)], WORKBENCH),
    r(Cat::Utility, (COMPASS, 1), &[(IRON_INGOT, 4), (CRYSTAL, 1)], WORKBENCH),
    r(Cat::Utility, (CLOCK, 1), &[(GOLD_INGOT, 4), (CRYSTAL, 1)], WORKBENCH),
    r(Cat::Tools, (WOOD_PICK, 1), &[(PLANKS, 3), (STICK, 2)], AIR),
    r(Cat::Tools, (STONE_PICK, 1), &[(COBBLE, 3), (STICK, 2)], AIR),
    r(Cat::Tools, (IRON_PICK, 1), &[(IRON_INGOT, 3), (STICK, 2)], WORKBENCH),
    r(Cat::Tools, (GOLD_PICK, 1), &[(GOLD_INGOT, 3), (STICK, 2)], WORKBENCH),
    r(Cat::Tools, (CRYSTAL_PICK, 1), &[(CRYSTAL, 3), (STICK, 2)], WORKBENCH),
    r(Cat::Tools, (WOOD_AXE, 1), &[(PLANKS, 3), (STICK, 2)], AIR),
    r(Cat::Tools, (STONE_AXE, 1), &[(COBBLE, 3), (STICK, 2)], AIR),
    r(Cat::Tools, (IRON_AXE, 1), &[(IRON_INGOT, 3), (STICK, 2)], WORKBENCH),
    r(Cat::Tools, (GOLD_AXE, 1), &[(GOLD_INGOT, 3), (STICK, 2)], WORKBENCH),
    r(Cat::Tools, (CRYSTAL_AXE, 1), &[(CRYSTAL, 3), (STICK, 2)], WORKBENCH),
    r(Cat::Tools, (WOOD_SHOVEL, 1), &[(PLANKS, 1), (STICK, 2)], AIR),
    r(Cat::Tools, (STONE_SHOVEL, 1), &[(COBBLE, 1), (STICK, 2)], AIR),
    r(Cat::Tools, (IRON_SHOVEL, 1), &[(IRON_INGOT, 1), (STICK, 2)], WORKBENCH),
    r(Cat::Tools, (GOLD_SHOVEL, 1), &[(GOLD_INGOT, 1), (STICK, 2)], WORKBENCH),
    r(Cat::Tools, (CRYSTAL_SHOVEL, 1), &[(CRYSTAL, 1), (STICK, 2)], WORKBENCH),
    r(Cat::Tools, (WOOD_HOE, 1), &[(PLANKS, 2), (STICK, 2)], AIR),
    r(Cat::Tools, (STONE_HOE, 1), &[(COBBLE, 2), (STICK, 2)], AIR),
    r(Cat::Tools, (IRON_HOE, 1), &[(IRON_INGOT, 2), (STICK, 2)], WORKBENCH),
    r(Cat::Tools, (SHEARS, 1), &[(IRON_INGOT, 2)], WORKBENCH),
    r(Cat::Tools, (FISHING_ROD, 1), &[(STICK, 3), (FIBER, 2)], AIR),
    r(Cat::Combat, (FLINT_KNIFE, 1), &[(FLINT, 1), (STICK, 1)], AIR),
    r(Cat::Combat, (SPEAR, 1), &[(COBBLE, 1), (STICK, 2)], AIR),
    r(Cat::Combat, (WOOD_SWORD, 1), &[(PLANKS, 2), (STICK, 1)], AIR),
    r(Cat::Combat, (STONE_SWORD, 1), &[(COBBLE, 2), (STICK, 1)], AIR),
    r(Cat::Combat, (IRON_SWORD, 1), &[(IRON_INGOT, 2), (STICK, 1)], WORKBENCH),
    r(Cat::Combat, (GOLD_SWORD, 1), &[(GOLD_INGOT, 2), (STICK, 1)], WORKBENCH),
    r(Cat::Combat, (CRYSTAL_SWORD, 1), &[(CRYSTAL, 2), (STICK, 1)], WORKBENCH),
    r(Cat::Combat, (BOW, 1), &[(STICK, 3), (FIBER, 3)], AIR),
    r(Cat::Combat, (ARROW, 4), &[(FLINT, 1), (STICK, 1)], AIR),
    r(Cat::Armour, (LEATHER_HELMET, 1), &[(HIDE, 5)], WORKBENCH),
    r(Cat::Armour, (LEATHER_CHEST, 1), &[(HIDE, 8)], WORKBENCH),
    r(Cat::Armour, (LEATHER_LEGS, 1), &[(HIDE, 7)], WORKBENCH),
    r(Cat::Armour, (LEATHER_BOOTS, 1), &[(HIDE, 4)], WORKBENCH),
    r(Cat::Armour, (IRON_HELMET, 1), &[(IRON_INGOT, 5)], WORKBENCH),
    r(Cat::Armour, (IRON_CHEST, 1), &[(IRON_INGOT, 8)], WORKBENCH),
    r(Cat::Armour, (IRON_LEGS, 1), &[(IRON_INGOT, 7)], WORKBENCH),
    r(Cat::Armour, (IRON_BOOTS, 1), &[(IRON_INGOT, 4)], WORKBENCH),
    r(Cat::Armour, (GOLD_HELMET, 1), &[(GOLD_INGOT, 5)], WORKBENCH),
    r(Cat::Armour, (GOLD_CHEST, 1), &[(GOLD_INGOT, 8)], WORKBENCH),
    r(Cat::Armour, (GOLD_LEGS, 1), &[(GOLD_INGOT, 7)], WORKBENCH),
    r(Cat::Armour, (GOLD_BOOTS, 1), &[(GOLD_INGOT, 4)], WORKBENCH),
    r(Cat::Armour, (CRYSTAL_HELMET, 1), &[(CRYSTAL, 5)], WORKBENCH),
    r(Cat::Armour, (CRYSTAL_CHEST, 1), &[(CRYSTAL, 8)], WORKBENCH),
    r(Cat::Armour, (CRYSTAL_LEGS, 1), &[(CRYSTAL, 7)], WORKBENCH),
    r(Cat::Armour, (CRYSTAL_BOOTS, 1), &[(CRYSTAL, 4)], WORKBENCH),
    // pyre gear is crystal gear forged over with the rare metal
    r(Cat::Armour, (PYRE_HELMET, 1), &[(CRYSTAL_HELMET, 1), (PYRE_INGOT, 3)], SMITHING),
    r(Cat::Armour, (PYRE_CHEST, 1), &[(CRYSTAL_CHEST, 1), (PYRE_INGOT, 5)], SMITHING),
    r(Cat::Armour, (PYRE_LEGS, 1), &[(CRYSTAL_LEGS, 1), (PYRE_INGOT, 4)], SMITHING),
    r(Cat::Armour, (PYRE_BOOTS, 1), &[(CRYSTAL_BOOTS, 1), (PYRE_INGOT, 2)], SMITHING),
    r(Cat::Tools, (PYRE_PICK, 1), &[(CRYSTAL_PICK, 1), (PYRE_INGOT, 3)], SMITHING),
    r(Cat::Tools, (PYRE_AXE, 1), &[(CRYSTAL_AXE, 1), (PYRE_INGOT, 3)], SMITHING),
    r(Cat::Tools, (PYRE_SHOVEL, 1), &[(CRYSTAL_SHOVEL, 1), (PYRE_INGOT, 1)], SMITHING),
    r(Cat::Combat, (PYRE_SWORD, 1), &[(CRYSTAL_SWORD, 1), (PYRE_INGOT, 2)], SMITHING),
    r(Cat::Building, (EXTRA_ITEM + 1, 4), &[(EXTRA_ITEM + 2, 4)], WORKBENCH),
    r(Cat::Food, (BREAD, 1), &[(WHEAT, 3)], AIR),
    r(Cat::Food, (STEW, 1), &[(MUSHROOM, 2), (BERRIES, 2), (BOWL, 1)], WORKBENCH),
    r(Cat::Food, (GOLDEN_APPLE, 1), &[(GOLD_INGOT, 8), (APPLE, 1)], WORKBENCH),
    r(Cat::Food, (GOLDEN_CARROT, 1), &[(GOLD_NUGGET, 8), (CARROT, 1)], WORKBENCH),
    r(Cat::Building, (STONE_BRICK, 4), &[(STONE, 4)], AIR),
    r(Cat::Building, (MOSSY_COBBLE, 1), &[(COBBLE, 1), (FIBER, 1)], AIR),
    r(Cat::Building, (SANDSTONE, 1), &[(SAND, 4)], AIR),
    r(Cat::Building, (THATCH, 2), &[(FIBER, 3)], AIR),
    r(Cat::Building, (CLOTH, 1), &[(FIBER, 4)], AIR),
    r(Cat::Building, (LEATHER, 1), &[(HIDE, 2)], AIR),
    r(Cat::Building, (PANE, 16), &[(GLASS, 6)], WORKBENCH),
    r(Cat::Building, (OAK_DOOR, 3), &[(PLANKS, 6)], WORKBENCH),
    r(Cat::Building, (BIRCH_DOOR, 3), &[(BIRCH_PLANKS, 6)], WORKBENCH),
    r(Cat::Building, (STAIRS, 4), &[(PLANKS, 6)], WORKBENCH),
    r(Cat::Building, (STAIRS + 1, 4), &[(BIRCH_PLANKS, 6)], WORKBENCH),
    r(Cat::Building, (STAIRS + 2, 4), &[(COBBLE, 6)], WORKBENCH),
    r(Cat::Building, (STAIRS + 3, 4), &[(STONE_BRICK, 6)], WORKBENCH),
    r(Cat::Building, (SLAB, 6), &[(PLANKS, 3)], WORKBENCH),
    r(Cat::Building, (SLAB + 1, 6), &[(BIRCH_PLANKS, 3)], WORKBENCH),
    r(Cat::Building, (SLAB + 2, 6), &[(COBBLE, 3)], WORKBENCH),
    r(Cat::Building, (SLAB + 3, 6), &[(STONE_BRICK, 3)], WORKBENCH),
    r(Cat::Building, (FENCE, 3), &[(PLANKS, 4), (STICK, 2)], WORKBENCH),
    r(Cat::Building, (BIRCH_FENCE, 3), &[(BIRCH_PLANKS, 4), (STICK, 2)], WORKBENCH),
    r(Cat::Building, (IRON_BLOCK, 1), &[(IRON_INGOT, 9)], WORKBENCH),
    r(Cat::Building, (GOLD_BLOCK, 1), &[(GOLD_INGOT, 9)], WORKBENCH),
    r(Cat::Materials, (IRON_INGOT, 9), &[(IRON_BLOCK, 1)], AIR),
    r(Cat::Materials, (GOLD_INGOT, 9), &[(GOLD_BLOCK, 1)], AIR),
    r(Cat::Utility, (ANVIL_ITEM, 1), &[(IRON_BLOCK, 3), (IRON_INGOT, 4)], WORKBENCH),
    r(Cat::Utility, (ENCHANTING_ITEM, 1), &[(BOOK, 1), (CRYSTAL, 2), (DEEP_ITEM, 4)], WORKBENCH),
    r(Cat::Combat, (SHIELD, 1), &[(PLANKS, 6), (IRON_INGOT, 1)], WORKBENCH),

    r(Cat::Building,(OAK_GATE,1),&[(STICK,4),(PLANKS,2)],WORKBENCH),
    r(Cat::Building,(BIRCH_GATE,1),&[(STICK,4),(BIRCH_PLANKS,2)],WORKBENCH),
    r(Cat::Building,(SIGN_ITEM,3),&[(PLANKS,6),(STICK,1)],WORKBENCH),
    r(Cat::Building,(EXTRA_SLAB+0,6),&[(BRICK,3)],WORKBENCH),
    r(Cat::Building,(EXTRA_STAIRS+0,4),&[(BRICK,6)],WORKBENCH),
    r(Cat::Building,(EXTRA_SLAB+1,6),&[(SANDSTONE,3)],WORKBENCH),
    r(Cat::Building,(EXTRA_STAIRS+1,4),&[(SANDSTONE,6)],WORKBENCH),
    r(Cat::Building,(EXTRA_SLAB+2,6),&[(EXTRA_ITEM,3)],WORKBENCH),
    r(Cat::Building,(EXTRA_STAIRS+2,4),&[(EXTRA_ITEM,6)],WORKBENCH),
    r(Cat::Building,(EXTRA_SLAB+3,6),&[(DEEP_ITEM+8,3)],WORKBENCH),
    r(Cat::Building,(EXTRA_STAIRS+3,4),&[(DEEP_ITEM+8,6)],WORKBENCH),
    // ---- from Minecraft's block list
    r(Cat::Materials, (PAPER, 3), &[(REED_ITEM, 3)], WORKBENCH),
    r(Cat::Materials, (BOOK, 1), &[(PAPER, 3), (HIDE, 1)], AIR),
    r(Cat::Building, (CLAY, 1), &[(CLAY_BALL, 4)], AIR),
    r(Cat::Building, (BRICK, 1), &[(BRICK_ITEM, 4)], AIR),
    r(Cat::Building, (cat(c::COAL_BLOCK), 1), &[(COAL, 9)], WORKBENCH),
    r(Cat::Materials, (COAL, 9), &[(cat(c::COAL_BLOCK), 1)], AIR),
    r(Cat::Building, (cat(c::CRYSTAL_BLOCK), 1), &[(CRYSTAL, 9)], WORKBENCH),
    r(Cat::Materials, (CRYSTAL, 9), &[(cat(c::CRYSTAL_BLOCK), 1)], AIR),
    r(Cat::Building, (cat(c::GEM_BLOCK), 1), &[(GEM, 9)], WORKBENCH),
    r(Cat::Materials, (GEM, 9), &[(cat(c::GEM_BLOCK), 1)], AIR),
    r(Cat::Building, (cat(c::PYRE_BLOCK), 1), &[(PYRE_INGOT, 9)], WORKBENCH),
    r(Cat::Materials, (PYRE_INGOT, 9), &[(cat(c::PYRE_BLOCK), 1)], AIR),
    r(Cat::Building, (cat(c::HAY_BALE), 1), &[(WHEAT, 9)], WORKBENCH),
    r(Cat::Food, (WHEAT, 9), &[(cat(c::HAY_BALE), 1)], AIR),
    r(Cat::Building, (cat(c::BONE_BLOCK), 1), &[(BONE_MEAL, 9)], WORKBENCH),
    r(Cat::Materials, (BONE_MEAL, 9), &[(cat(c::BONE_BLOCK), 1)], AIR),
    r(Cat::Building, (cat(c::BOOKSHELF), 1), &[(PLANKS, 6), (BOOK, 3)], WORKBENCH),
    r(Cat::Building, (cat(c::BOOKSHELF), 1), &[(BIRCH_PLANKS, 6), (BOOK, 3)], WORKBENCH),
    r(Cat::Building, (cat(c::CHISELED_STONE_BRICKS), 1), &[(SLAB + 3, 2)], AIR),
    r(Cat::Building, (cat(c::MOSSY_STONE_BRICKS), 1), &[(STONE_BRICK, 1), (FIBER, 1)], AIR),
    r(Cat::Building, (cat(c::CUT_SANDSTONE), 4), &[(SANDSTONE, 4)], AIR),
    r(Cat::Building, (cat(c::CHISELED_SANDSTONE), 1), &[(EXTRA_SLAB + 1, 2)], AIR),
    r(Cat::Building, (cat(c::RED_SANDSTONE), 1), &[(RED_SAND, 4)], AIR),
    r(Cat::Building, (cat(c::CUT_RED_SANDSTONE), 4), &[(cat(c::RED_SANDSTONE), 4)], AIR),
    r(Cat::Building, (cat(c::PACKED_ICE), 1), &[(ICE, 9)], WORKBENCH),
    r(Cat::Building, (cat(c::BLUE_ICE), 1), &[(cat(c::PACKED_ICE), 9)], WORKBENCH),
    r(Cat::Building, (cat(c::POLISHED_SLATE), 4), &[(EXTRA_ITEM, 4)], AIR),
    r(Cat::Building, (cat(c::SLATE_BRICKS), 4), &[(cat(c::POLISHED_SLATE), 4)], AIR),
    r(Cat::Building, (cat(c::SLATE_TILES), 4), &[(cat(c::SLATE_BRICKS), 4)], AIR),
    r(Cat::Building, (cat(c::CHISELED_SLATE), 1), &[(EXTRA_SLAB + 2, 2)], AIR),
    r(Cat::Building, (tc(0), 8), &[(TERRACOTTA, 8), (DYE + 0, 1)], WORKBENCH),
    r(Cat::Building, (tc(1), 8), &[(TERRACOTTA, 8), (DYE + 1, 1)], WORKBENCH),
    r(Cat::Building, (tc(2), 8), &[(TERRACOTTA, 8), (DYE + 2, 1)], WORKBENCH),
    r(Cat::Building, (tc(3), 8), &[(TERRACOTTA, 8), (DYE + 3, 1)], WORKBENCH),
    r(Cat::Building, (tc(4), 8), &[(TERRACOTTA, 8), (DYE + 4, 1)], WORKBENCH),
    r(Cat::Building, (tc(5), 8), &[(TERRACOTTA, 8), (DYE + 5, 1)], WORKBENCH),
    r(Cat::Building, (tc(6), 8), &[(TERRACOTTA, 8), (DYE + 6, 1)], WORKBENCH),
    r(Cat::Building, (tc(7), 8), &[(TERRACOTTA, 8), (DYE + 7, 1)], WORKBENCH),
    r(Cat::Building, (tc(8), 8), &[(TERRACOTTA, 8), (DYE + 8, 1)], WORKBENCH),
    r(Cat::Building, (tc(9), 8), &[(TERRACOTTA, 8), (DYE + 9, 1)], WORKBENCH),
    r(Cat::Building, (tc(10), 8), &[(TERRACOTTA, 8), (DYE + 10, 1)], WORKBENCH),
    r(Cat::Building, (tc(11), 8), &[(TERRACOTTA, 8), (DYE + 11, 1)], WORKBENCH),
    r(Cat::Building, (tc(12), 8), &[(TERRACOTTA, 8), (DYE + 12, 1)], WORKBENCH),
    r(Cat::Building, (tc(13), 8), &[(TERRACOTTA, 8), (DYE + 13, 1)], WORKBENCH),
    r(Cat::Building, (tc(14), 8), &[(TERRACOTTA, 8), (DYE + 14, 1)], WORKBENCH),
    r(Cat::Building, (tc(15), 8), &[(TERRACOTTA, 8), (DYE + 15, 1)], WORKBENCH),
    r(Cat::Utility, (cat(c::SMOKER), 1), &[(FURNACE, 1), (LOG, 4)], WORKBENCH),
    r(Cat::Utility, (cat(c::SMOKER), 1), &[(FURNACE, 1), (BIRCH_LOG, 4)], WORKBENCH),
    r(Cat::Utility, (cat(c::BLAST_FURNACE), 1), &[(IRON_INGOT, 5), (FURNACE, 1), (cat(c::SMOOTH_STONE), 3)], WORKBENCH),
    r(Cat::Utility, (cat(c::SMITHING_TABLE), 1), &[(IRON_INGOT, 2), (PLANKS, 4)], WORKBENCH),
    r(Cat::Utility, (cat(c::STONECUTTER), 1), &[(IRON_INGOT, 1), (STONE, 3)], WORKBENCH),
    r(Cat::Utility, (cat(c::GRINDSTONE), 1), &[(STICK, 2), (SLAB + 2, 1), (PLANKS, 2)], WORKBENCH),
    r(Cat::Utility, (cat(c::FLETCHING_TABLE), 1), &[(FLINT, 2), (PLANKS, 4)], WORKBENCH),
    r(Cat::Utility, (cat(c::CARTOGRAPHY_TABLE), 1), &[(PAPER, 2), (PLANKS, 4)], WORKBENCH),
    r(Cat::Utility, (cat(c::LOOM), 1), &[(FIBER, 2), (PLANKS, 2)], WORKBENCH),
    r(Cat::Utility, (cat(c::LANTERN), 1), &[(IRON_NUGGET, 8), (TORCH, 1)], WORKBENCH),
    // ---- the stonecutter: one block in, its pieces out (Minecraft's yields)
    r(Cat::Building, (STONE_BRICK, 1), &[(STONE, 1)], STONECUTTER),
    r(Cat::Building, (STAIRS + 3, 1), &[(STONE, 1)], STONECUTTER),
    r(Cat::Building, (SLAB + 3, 2), &[(STONE, 1)], STONECUTTER),
    r(Cat::Building, (cat(c::CHISELED_STONE_BRICKS), 1), &[(STONE, 1)], STONECUTTER),
    r(Cat::Building, (STAIRS + 3, 1), &[(STONE_BRICK, 1)], STONECUTTER),
    r(Cat::Building, (SLAB + 3, 2), &[(STONE_BRICK, 1)], STONECUTTER),
    r(Cat::Building, (cat(c::CHISELED_STONE_BRICKS), 1), &[(STONE_BRICK, 1)], STONECUTTER),
    r(Cat::Building, (STAIRS + 2, 1), &[(COBBLE, 1)], STONECUTTER),
    r(Cat::Building, (SLAB + 2, 2), &[(COBBLE, 1)], STONECUTTER),
    r(Cat::Building, (EXTRA_STAIRS, 1), &[(BRICK, 1)], STONECUTTER),
    r(Cat::Building, (EXTRA_SLAB, 2), &[(BRICK, 1)], STONECUTTER),
    r(Cat::Building, (EXTRA_STAIRS + 1, 1), &[(SANDSTONE, 1)], STONECUTTER),
    r(Cat::Building, (EXTRA_SLAB + 1, 2), &[(SANDSTONE, 1)], STONECUTTER),
    r(Cat::Building, (cat(c::CUT_SANDSTONE), 1), &[(SANDSTONE, 1)], STONECUTTER),
    r(Cat::Building, (cat(c::CHISELED_SANDSTONE), 1), &[(SANDSTONE, 1)], STONECUTTER),
    r(Cat::Building, (cat(c::CUT_RED_SANDSTONE), 1), &[(cat(c::RED_SANDSTONE), 1)], STONECUTTER),
    r(Cat::Building, (cat(c::CHISELED_RED_SANDSTONE), 1), &[(cat(c::RED_SANDSTONE), 1)], STONECUTTER),
    r(Cat::Building, (EXTRA_STAIRS + 2, 1), &[(EXTRA_ITEM, 1)], STONECUTTER),
    r(Cat::Building, (EXTRA_SLAB + 2, 2), &[(EXTRA_ITEM, 1)], STONECUTTER),
    r(Cat::Building, (cat(c::POLISHED_SLATE), 1), &[(EXTRA_ITEM, 1)], STONECUTTER),
    r(Cat::Building, (cat(c::SLATE_BRICKS), 1), &[(EXTRA_ITEM, 1)], STONECUTTER),
    r(Cat::Building, (cat(c::SLATE_TILES), 1), &[(EXTRA_ITEM, 1)], STONECUTTER),
    r(Cat::Building, (cat(c::CHISELED_SLATE), 1), &[(EXTRA_ITEM, 1)], STONECUTTER),
    r(Cat::Building, (cat(c::SLATE_BRICKS), 1), &[(cat(c::POLISHED_SLATE), 1)], STONECUTTER),
    r(Cat::Building, (cat(c::SLATE_TILES), 1), &[(cat(c::POLISHED_SLATE), 1)], STONECUTTER),
    r(Cat::Building, (cat(c::SLATE_TILES), 1), &[(cat(c::SLATE_BRICKS), 1)], STONECUTTER),
];

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Stack {
    pub item: Item,
    pub n: u32,
    /// uses spent, for tools
    pub wear: u16,
    pub enchants: u8,
}

impl Stack {
    pub fn new(item: Item, n: u32) -> Stack {
        Stack { item, n, wear: 0, enchants: 0 }
    }

    pub fn persist_wear(self) -> u32 { self.wear as u32 | (self.enchants as u32) << 16 }

    pub fn is_empty(&self) -> bool {
        self.n == 0
    }

    /// Remaining durability, 0..1 (None for items that never wear).
    pub fn condition(&self) -> Option<f32> {
        let max = max_wear(self.item);
        (max > 0 && !self.is_empty()).then(|| 1.0 - self.wear as f32 / max as f32)
    }
}

/// Left click on a slot with `held` on the mouse: pick up, put down, merge or swap.
/// `accepts` is false where the slot will not take the held item.
pub fn click_stack(held: &mut Stack, slot: &mut Stack, accepts: bool) {
    let s = *slot;
    if held.is_empty() {
        *held = s;
        *slot = Stack::default();
    } else if !accepts {
    } else if !s.is_empty() && s.item == held.item && max_stack(s.item) > 1 {
        let k = held.n.min(max_stack(s.item).saturating_sub(s.n));
        slot.n += k;
        *held = if held.n > k { Stack::new(held.item, held.n - k) } else { Stack::default() };
    } else {
        *slot = *held;
        *held = s;
    }
}

/// Right click on a slot: take half of it, or put down one of the held stack.
pub fn right_click_stack(held: &mut Stack, slot: &mut Stack, accepts: bool) {
    let s = *slot;
    if held.is_empty() {
        if !s.is_empty() {
            let k = (s.n + 1) / 2;
            *held = Stack { n: k, ..s };
            *slot = if s.n > k { Stack { n: s.n - k, ..s } } else { Stack::default() };
        }
    } else if max_stack(held.item) == 1 {
        if s.is_empty() {
            click_stack(held, slot, accepts);
        }
    } else if accepts && (s.is_empty() || (s.item == held.item && s.n < max_stack(s.item))) {
        *slot = Stack { n: s.n + 1, ..*held };
        held.n -= 1;
        if held.n == 0 {
            *held = Stack::default();
        }
    }
}

/// Moves a whole stack (keeping its wear) into `slots`: tops up matching stacks, then
/// fills empty slots. Returns what did not fit.
pub fn stash_in(slots: &mut [Stack], st: Stack) -> Stack {
    if st.is_empty() {
        return st;
    }
    let max = max_stack(st.item);
    let mut n = st.n;
    if max > 1 {
        for s in slots.iter_mut().filter(|s| !s.is_empty() && s.item == st.item) {
            let k = n.min(max.saturating_sub(s.n));
            s.n += k;
            n -= k;
        }
    }
    for s in slots.iter_mut().filter(|s| s.is_empty()) {
        if n == 0 {
            break;
        }
        let k = n.min(max);
        *s = Stack { n: k, ..st };
        n -= k;
    }
    if n > 0 { Stack { n, ..st } } else { Stack::default() }
}

/// slots 0..9 are the hotbar, 9..36 the backpack, 36..40 armour (head, chest, legs, feet)
pub const HOTBAR: usize = 9;
pub const BAG_END: usize = 36;
pub const ARMOR: usize = 36;
pub const SLOTS: usize = 40;

#[derive(Clone)]
pub struct Inventory {
    pub slots: [Stack; SLOTS],
    /// the stack picked up with the mouse in the inventory screen
    pub held: Stack,
    pub grid: [Stack; 9],
}

impl Inventory {
    pub fn new() -> Inventory {
        Inventory { slots: [Stack::default(); SLOTS], held: Stack::default(), grid: [Stack::default(); 9] }
    }

    /// Cursor contents have their own record, even when every bag slot is occupied.
    pub fn save_stacks(&self) -> String {
        let mut text = String::from("slots");
        for (i, st) in self.slots.iter().enumerate().filter(|(_, st)| !st.is_empty()) {
            text += &format!(" {}:{}:{}:{}", i, st.item, st.n, st.persist_wear());
        }
        text += &format!("\nheld {} {} {}\n", self.held.item, self.held.n, self.held.persist_wear());
        if self.grid.iter().any(|s| !s.is_empty()) {
        text.push_str("grid");
        for (i, st) in self.grid.iter().enumerate().filter(|(_, s)| !s.is_empty()) { text += &format!(" {}:{}:{}:{}", i, st.item, st.n, st.persist_wear()); }
        text.push('\n');
        }
        text
    }

    /// Item in a hotbar slot (AIR if empty).
    pub fn hot(&self, slot: usize) -> Item {
        self.slots[slot].item
    }

    /// Share of damage that gets through the armour worn.
    pub fn armor_factor(&self) -> f32 {
        let base = 1.0 - self.slots[ARMOR..].iter().filter_map(|s| armor(s.item)).map(|a| a.1).sum::<f32>();
        let protection: u32 = self.slots[ARMOR..].iter().filter(|s| !s.is_empty()).map(|&s| crate::equipment::Enchant::Protection.level(s) as u32).sum();
        base * (1.0 - 0.04 * protection.min(20) as f32)
    }

    pub fn is_empty(&self) -> bool {
        self.slots.iter().all(|s| s.is_empty())
    }

    /// Total carried in the hotbar and backpack.
    pub fn count(&self, i: Item) -> u32 {
        self.slots[..BAG_END].iter().filter(|s| s.item == i).map(|s| s.n).sum()
    }

    /// True if the item is anywhere on the player.
    pub fn has(&self, i: Item) -> bool {
        self.count(i) > 0
    }

    fn put(&mut self, slot: usize, i: Item, n: u32) -> u32 {
        let s = &mut self.slots[slot];
        if s.is_empty() || s.item == i {
            let k = n.min(max_stack(i).saturating_sub(s.n));
            if k > 0 {
                *s = Stack { item: i, n: s.n + k, wear: s.wear, enchants: s.enchants };
            }
            n - k
        } else {
            n
        }
    }

    /// Adds to the slots in `range`: tops up matching stacks, then fills empty slots.
    fn add_to(&mut self, range: std::ops::Range<usize>, i: Item, mut n: u32) -> u32 {
        for slot in range.clone() {
            if n > 0 && self.slots[slot].item == i && !self.slots[slot].is_empty() {
                n = self.put(slot, i, n);
            }
        }
        for slot in range {
            if n > 0 && self.slots[slot].is_empty() {
                n = self.put(slot, i, n);
            }
        }
        n
    }

    /// Moves a whole stack (keeping its wear) into `range`. Returns what did not fit.
    fn stash(&mut self, range: std::ops::Range<usize>, st: Stack) -> Stack {
        stash_in(&mut self.slots[range], st)
    }

    /// Picks a stack up into the hotbar and backpack. Returns what did not fit.
    pub fn pick_up(&mut self, st: Stack) -> Stack {
        if valid(st.item) { self.stash(0..BAG_END, st) } else { Stack::default() }
    }

    /// Adds items. Returns how many did not fit.
    pub fn add(&mut self, i: Item, n: u32) -> u32 {
        if !valid(i) || n == 0 {
            return 0;
        }
        self.add_to(0..BAG_END, i, n)
    }

    /// Removes `n` of an item, emptying the backpack before the hotbar.
    pub fn take(&mut self, i: Item, mut n: u32) -> bool {
        if self.count(i) < n {
            return false;
        }
        for slot in (0..BAG_END).rev() {
            let s = &mut self.slots[slot];
            if s.item == i && n > 0 {
                let k = n.min(s.n);
                s.n -= k;
                n -= k;
                if s.n == 0 {
                    *s = Stack::default();
                }
            }
        }
        true
    }

    /// Removes one item from a specific slot.
    pub fn take_slot(&mut self, slot: usize) -> bool {
        let s = &mut self.slots[slot];
        if s.is_empty() {
            return false;
        }
        s.n -= 1;
        if s.n == 0 {
            *s = Stack::default();
        }
        true
    }

    /// Spends one use of the tool in `slot`. Returns true if it just broke.
    #[cfg(test)]
    pub fn wear_tool(&mut self, slot: usize) -> bool {
        let s = &mut self.slots[slot];
        let max = max_wear(s.item);
        if max == 0 || s.is_empty() {
            return false;
        }
        s.wear += 1;
        if s.wear >= max {
            *s = Stack::default();
            return true;
        }
        false
    }

    pub fn can_craft(&self, r: &Recipe) -> bool {
        r.ins.iter().all(|&(i, n)| self.count(i) >= n)
    }

    /// Crafts once into the inventory. Nothing changes unless the entire result fits.
    pub fn craft(&mut self, r: &Recipe) -> bool {
        if !self.can_craft(r) {
            return false;
        }
        let mut next = self.clone();
        for &(i, n) in r.ins {
            if !next.take(i, n) {
                return false;
            }
        }
        let left = next.pick_up(crate::equipment::craft_output(r,&self.slots));
        if !stash_in(std::slice::from_mut(&mut next.held), left).is_empty() {
            return false;
        }
        *self = next;
        true
    }

    /// Crafts once onto the mouse, like taking the result out of a crafting grid.
    pub fn craft_to_hand(&mut self, r: &Recipe) -> bool {
        let fits = if self.held.is_empty() {
            r.out.1 <= max_stack(r.out.0)
        } else {
            self.held.item == r.out.0 && r.out.1 <= max_stack(r.out.0).saturating_sub(self.held.n)
        };
        if !fits {
            return self.craft(r);
        }
        if !self.can_craft(r) {
            return false;
        }
        let output=crate::equipment::craft_output(r,&self.slots);
        for &(i, n) in r.ins {
            self.take(i, n);
        }
        self.held = Stack { n:self.held.n+r.out.1,..output };
        true
    }

    fn accepts(slot: usize, i: Item) -> bool {
        slot < ARMOR || armor(i).is_some_and(|a| a.0 == slot - ARMOR)
    }

    /// Left click: pick up, drop, merge or swap the held stack.
    pub fn click(&mut self, slot: usize) {
        let ok = Self::accepts(slot, self.held.item);
        click_stack(&mut self.held, &mut self.slots[slot], ok);
    }

    /// Right click: take half a stack, or drop a single held item.
    pub fn right_click(&mut self, slot: usize) {
        let ok = Self::accepts(slot, self.held.item);
        right_click_stack(&mut self.held, &mut self.slots[slot], ok);
    }

    /// Shift click: send a stack to the other half of the inventory (armour goes on).
    pub fn quick_move(&mut self, slot: usize) {
        let s = self.slots[slot];
        if s.is_empty() {
            return;
        }
        self.slots[slot] = Stack::default();
        let worn = armor(s.item).map(|a| ARMOR + a.0).filter(|&a| slot < ARMOR && self.slots[a].is_empty());
        let left = if let Some(a) = worn {
            self.slots[a] = s;
            Stack::default()
        } else if slot < HOTBAR {
            self.stash(HOTBAR..BAG_END, s)
        } else if slot < BAG_END {
            self.stash(0..HOTBAR, s)
        } else {
            self.stash(0..BAG_END, s)
        };
        if !left.is_empty() {
            self.slots[slot] = left;
        }
    }

    /// Puts the mouse stack back when the inventory closes.
    pub fn stow_held(&mut self) {
        for i in 0..9 { self.grid[i] = self.stash(0..BAG_END, self.grid[i]); }
        self.held = self.stash(0..BAG_END, self.held);
    }

    /// Merges and orders the backpack: blocks first, then tools and materials.
    pub fn sort_bag(&mut self) {
        let mut all: Vec<Stack> = self.slots[HOTBAR..BAG_END].iter().copied().filter(|s| !s.is_empty()).collect();
        all.sort_by_key(|s| (s.item, s.wear));
        for s in &mut self.slots[HOTBAR..BAG_END] {
            *s = Stack::default();
        }
        for s in all {
            self.stash(HOTBAR..BAG_END, s);
        }
    }
}

/// The item that places (and drops from) a door block.
pub fn door_item(b: Block) -> Item {
    OAK_DOOR + door_wood(b)
}

/// The item a block in the world stands for (middle click, and what it drops).
pub fn item_of(b: Block) -> Item {
    if let Some(r) = crate::catalogue::row_of_block(b) {
        return crate::catalogue::item(r);
    }
    if crate::building::is_fence_gate(b) {return OAK_GATE+((b-crate::building::FENCE_GATE)>>2);}
    if crate::building::is_sign(b) {return SIGN_ITEM;}
    if is_slab(b) && slab_mat(b)>=4 {return EXTRA_SLAB+slab_mat(b)-4;}
    if is_stairs(b) && stair_mat(b)>=4 {return EXTRA_STAIRS+stair_mat(b)-4;}
    if let Some((item, _)) = crate::farming::harvest(b) { return item; }
    if crate::farming::moisture(b).is_some() { return DIRT; }
    match b {
        ANVIL => ANVIL_ITEM,
        ENCHANTING_TABLE => ENCHANTING_ITEM,
        b if is_door(b) => door_item(b),
        b if is_slab(b) => SLAB + slab_mat(b),
        b if is_stairs(b) => STAIRS + stair_mat(b),
        b if is_ladder(b) => LADDER,
        b if is_trapdoor(b) => TRAPDOOR,
        b if is_torch(b) => TORCH,
        b if (WOOL..WOOL_END).contains(&b) => WOOL_ITEM + (b - WOOL),
        b if is_deep_block(b) => DEEP_ITEM + (b - CINDERSTONE),
        b if is_extra_block(b) => EXTRA_ITEM + (b - SLATE),
        b if is_sapling(b) => SAPLING_ITEM + (b - SAPLING),
        b if is_gate(b) => AIR,
        LOOT_CHEST => CHEST,
        b if crate::building::is_chest(b) => CHEST,
        b if crate::building::is_bed(b) => BED,
        b if is_grassy(b) => GRASS,
        JUNGLE_LEAVES | ACACIA_LEAVES => LEAVES,
        SNOWY_LEAVES => PINE_LEAVES,
        FERN | DRY_TUFT => TALL_GRASS,
        REEDS => REED_ITEM,
        b => b,
    }
}

/// Pickaxe tier a block needs before it drops anything.
pub fn tier_needed(b: Block) -> u8 {
    if let Some(d) = crate::catalogue::def(b) {
        return d.tier;
    }
    match b {
        IRON_ORE => 2,
        CRYSTAL_ORE | GOLD_ORE => 3,
        // (gates are built of something only the best pick cuts)
        CINDERSTONE | PYRE_ORE => 4,
        // (as in Minecraft, stone and the like give nothing to a bare hand)
        b if crate::blockinfo::needs_pick(b) => 1,
        _ => 0,
    }
}

/// What a block gives when broken with `tool` (AIR = nothing).
pub fn drop_for(b: Block, tool: Item) -> Item {
    if let Some((item, _)) = crate::farming::harvest(b) { return item; }
    if crate::farming::moisture(b).is_some() { return DIRT; }
    if pick_tier(tool) < tier_needed(b) {
        return AIR;
    }
    match b {
        STONE => COBBLE,
        FARMLAND => DIRT,
        b if is_grassy(b) => DIRT,
        REEDS => REED_ITEM,
        CLAY => CLAY_BALL,
        COAL_ORE => COAL,
        EMBER_ORE => EMBER,
        CRYSTAL_ORE => CRYSTAL,
        BERRY_BUSH => BERRIES,
        DEAD_BUSH => STICK,
        // snow packs back into snowballs, but only a shovel gathers them
        SNOW if matches!(gear(tool), Some((Kind::Shovel, _))) => SNOWBALL,
        SNOW => AIR,
        b if is_tuft(b) => {
            if tool == SHEARS {
                TALL_GRASS
            } else {
                FIBER
            }
        }
        WHEAT_0 | WHEAT_1 | WHEAT_2 => SEEDS,
        WHEAT_3 => WHEAT,
        b if is_fluid(b) => AIR,
        BEDROCK | AIR => AIR,
        b => item_of(b),
    }
}

/// Minecraft's tool speeds: how many times faster than a bare hand.
fn speed(m: Mat) -> f32 {
    match m {
        Mat::Wood => 2.0,
        Mat::Stone => 4.0,
        Mat::Iron => 6.0,
        Mat::Crystal => 8.0,
        Mat::Pyre => 9.0,
        Mat::Gold => 12.0,
        Mat::Leather => 1.0,
    }
}

/// Seconds of holding the button needed to break `b` with `tool` in hand, by
/// Minecraft's rule: hardness x 1.5 when the block can be harvested (x 5 when it
/// cannot), divided by the speed of the right tool.
pub fn break_time(b: Block, tool: Item) -> f32 {
    let props = crate::blockinfo::props(b);
    if !props.hardness.is_finite() {
        return f32::INFINITY;
    }
    if props.hardness <= 0.0 {
        return 0.05;
    }
    let mut fast = props.tool.and_then(|kind| self::tool(tool, kind)).map_or(1.0, speed);
    if tool == SHEARS {
        if is_leaves(b) {
            fast = 15.0;
        } else if is_wool(b) || b == CLOTH {
            fast = 5.0;
        }
    }
    let harvest = pick_tier(tool) >= tier_needed(b);
    (props.hardness * if harvest { 1.5 } else { 5.0 } / fast).max(0.05)
}

/// (damage, reach) of a melee hit with `tool` in hand.
pub fn weapon(tool: Item) -> (i32, f32) {
    match gear(tool) {
        Some((Kind::Sword, m)) => (
            match m {
                Mat::Stone => 4,
                Mat::Iron => 5,
                Mat::Crystal => 7,
                Mat::Pyre => 9,
                _ => 3,
            },
            4.6,
        ),
        Some((Kind::Axe, m)) => (
            match m {
                Mat::Iron => 3,
                Mat::Crystal => 4,
                Mat::Pyre => 6,
                _ => 2,
            },
            4.3,
        ),
        _ => match tool {
            SPEAR => (2, 5.6),
            FLINT_KNIFE => (2, 4.3),
            _ => (1, 4.3),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slots_stack_split_and_move() {
        let mut inv = Inventory::new();
        assert_eq!(inv.add(COBBLE, 150), 0);
        assert_eq!((inv.slots[0].n, inv.slots[1].n), (99, 51));
        assert_eq!(inv.count(COBBLE), 150);
        // tools do not stack
        inv.add(IRON_PICK, 2);
        assert_eq!((inv.slots[2].n, inv.slots[3].n), (1, 1));
        // right click takes half, left click drops it elsewhere
        inv.right_click(1);
        assert_eq!((inv.held.n, inv.slots[1].n), (26, 25));
        inv.click(20);
        assert!(inv.held.is_empty());
        assert_eq!(inv.slots[20], Stack::new(COBBLE, 26));
        // merging overflows back onto the mouse
        inv.click(20);
        inv.click(0);
        assert_eq!((inv.slots[0].n, inv.held.n), (99, 26));
        inv.stow_held();
        assert_eq!(inv.count(COBBLE), 150);
        // the armour slot only takes armour
        inv.click(2);
        inv.click(ARMOR);
        assert_eq!(inv.held.item, IRON_PICK);
        inv.stow_held();
        inv.add(IRON_CHEST, 1);
        let at = inv.slots.iter().position(|s| s.item == IRON_CHEST).unwrap();
        inv.quick_move(at);
        assert_eq!(inv.slots[ARMOR + 1].item, IRON_CHEST);
        assert!((inv.armor_factor() - 0.8).abs() < 0.001);
        // taking drains the backpack before the hotbar; crafting consumes and produces
        assert!(inv.take(COBBLE, 60));
        assert_eq!(inv.slots[0].n, 90);
        inv.add(STICK, 2);
        let r = RECIPES.iter().find(|r| r.out.0 == STONE_PICK).unwrap();
        assert!(inv.craft(r));
        assert_eq!((inv.count(COBBLE), inv.count(STICK), inv.count(STONE_PICK)), (87, 0, 1));
        // tools wear out and break
        let at = inv.slots.iter().position(|s| s.item == STONE_PICK).unwrap();
        for _ in 0..max_wear(STONE_PICK) - 1 {
            assert!(!inv.wear_tool(at));
        }
        assert!(inv.wear_tool(at));
        assert_eq!(inv.count(STONE_PICK), 0);
        inv.sort_bag();
        assert_eq!(inv.count(COBBLE), 87);
    }

    #[test]
    fn chest_slots_and_pick_up() {
        let mut inv = Inventory::new();
        let mut chest = [Stack::default(); CHEST_SLOTS];
        inv.add(COBBLE, 70);
        // carry a stack across with the mouse, then split one off with a right click
        inv.click(0);
        click_stack(&mut inv.held, &mut chest[3], true);
        assert!(inv.held.is_empty());
        assert_eq!(chest[3], Stack::new(COBBLE, 70));
        right_click_stack(&mut inv.held, &mut chest[3], true);
        assert_eq!((inv.held.n, chest[3].n), (35, 35));
        right_click_stack(&mut inv.held, &mut chest[4], true);
        assert_eq!((inv.held.n, chest[4].n), (34, 1));
        inv.stow_held();
        // shift-click style moves keep a tool's wear
        let pick = Stack { item: IRON_PICK, n: 1, wear: 9, enchants: 0 };
        assert!(stash_in(&mut chest, pick).is_empty());
        assert!(chest.contains(&pick));
        assert_eq!(stash_in(&mut chest, Stack::new(COBBLE, 100)).n, 0);
        assert_eq!(chest.iter().filter(|s| s.item == COBBLE).map(|s| s.n).sum::<u32>(), 136);
        // a full inventory hands back what it cannot take
        let mut full = Inventory::new();
        for s in &mut full.slots[..BAG_END] {
            *s = Stack::new(DIRT, 99);
        }
        full.slots[5].n = 90;
        assert_eq!(full.pick_up(Stack::new(DIRT, 20)).n, 11);
        assert_eq!(full.pick_up(pick), pick);
    }

    #[test]
    fn crafting_without_output_space_keeps_every_item() {
        let r = &RECIPES[0]; // one log -> four planks
        for to_hand in [false, true] {
            for held in [Stack::new(STONE, 1), Stack::new(PLANKS, 99), Stack::new(PLANKS, 97)] {
                let mut inv = Inventory::new();
                inv.slots[..BAG_END].fill(Stack::new(DIRT, 99));
                inv.slots[0] = Stack::new(LOG, 2);
                inv.held = held;
                let before = inv.clone();
                assert!(!if to_hand { inv.craft_to_hand(r) } else { inv.craft(r) });
                assert_eq!(inv.slots, before.slots);
                assert_eq!(inv.held, before.held);
            }
        }
        // Even partially placing the output must roll back if the rest cannot fit.
        let mut inv = Inventory::new();
        inv.slots[..BAG_END].fill(Stack::new(DIRT, 99));
        inv.slots[0] = Stack::new(LOG, 2);
        inv.slots[1] = Stack::new(PLANKS, 97);
        inv.held = Stack::new(STONE, 1);
        let before = inv.clone();
        assert!(!inv.craft(r));
        assert_eq!(inv.slots, before.slots);
        assert_eq!(inv.held, before.held);
    }

    #[test]
    fn crafting_uses_freed_slots_and_splits_output_within_stack_limits() {
        let r = &RECIPES[0];
        let mut inv = Inventory::new();
        inv.slots[..BAG_END].fill(Stack::new(DIRT, 99));
        inv.slots[0] = Stack::new(LOG, 1);
        inv.held = Stack::new(STONE, 1);
        assert!(inv.craft(r));
        assert_eq!(inv.slots[0], Stack::new(PLANKS, 4));
        assert_eq!(inv.held, Stack::new(STONE, 1));

        inv.slots[0] = Stack::new(LOG, 3);
        inv.slots[1] = Stack::new(PLANKS, 97);
        inv.held = Stack::new(PLANKS, 97);
        assert!(inv.craft_to_hand(r));
        assert_eq!(inv.count(LOG), 2);
        assert_eq!(inv.slots[1], Stack::new(PLANKS, 99));
        assert_eq!(inv.held, Stack::new(PLANKS, 99));
        assert!(!inv.craft(r));
        assert_eq!(inv.count(LOG), 2);
    }

    #[test]
    fn crafting_to_hand_preserves_ingredients_when_missing() {
        let mut inv = Inventory::new();
        inv.held = Stack::new(PLANKS, 95);
        assert!(!inv.craft_to_hand(&RECIPES[0]));
        assert_eq!(inv.held, Stack::new(PLANKS, 95));
        inv.add(LOG, 2);
        assert!(inv.craft_to_hand(&RECIPES[0]));
        assert_eq!(inv.held, Stack::new(PLANKS, 99));
        assert_eq!(inv.count(LOG), 1);
    }

    #[test]
    fn bulk_crafting_stops_before_overflow_or_losing_ingredients() {
        let mut inv = Inventory::new();
        inv.slots[..BAG_END].fill(Stack::new(DIRT, 99));
        inv.slots[0] = Stack::new(LOG, 99);
        let mut crafts = 0;
        while inv.craft(&RECIPES[0]) {
            crafts += 1;
            assert!(crafts < 25);
        }
        assert_eq!(crafts, 24);
        assert_eq!(inv.count(LOG), 75);
        assert_eq!(inv.held, Stack::new(PLANKS, 96));
    }

    #[test]
    fn stowing_the_cursor_preserves_overflow_and_tool_wear() {
        let mut inv = Inventory::new();
        inv.slots[..BAG_END].fill(Stack::new(DIRT, 99));
        inv.slots[0].n = 97;
        inv.held = Stack::new(DIRT, 5);
        inv.stow_held();
        assert_eq!(inv.slots[0].n, 99);
        assert_eq!(inv.held, Stack::new(DIRT, 3));

        let pick = Stack { item: IRON_PICK, n: 1, wear: 20, enchants: 0 };
        inv.held = pick;
        inv.stow_held();
        assert_eq!(inv.held, pick);
        inv.slots[1] = Stack::default();
        inv.stow_held();
        assert!(inv.held.is_empty());
        assert_eq!(inv.slots[1], pick);
    }

    #[test]
    fn shaped_blocks_drop_their_items() {
        for (b, item) in [(SLAB_TOP + 2, SLAB + 2), (STAIR_BASE + 8 + 5, STAIRS + 1), (LADDER_BASE + 3, LADDER), (TRAP_BASE + 13, TRAPDOOR), (DOOR_BASE + 40, BIRCH_DOOR), (CHEST, CHEST)] {
            assert_eq!(drop_for(b, WOOD_PICK), item);
            assert!(valid(item) && name(item) != "?" && !desc(item).is_empty());
        }
    }

    #[test]
    fn every_recipe_item_is_named() {
        for r in RECIPES {
            for &(i, _) in r.ins.iter().chain([&r.out]) {
                assert!(valid(i) && name(i) != "?", "item {i}");
            }
        }
    }
}
