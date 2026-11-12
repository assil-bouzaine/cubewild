//! Building states and container pairing. Every chest keeps its own 27 slots,
//! so joining or breaking a double chest never moves or duplicates its contents.
use crate::world::*;
use glam::IVec3;
use std::{collections::HashMap, io, path::Path};

pub const FENCE_GATE: Block = 288; // axis bit 0, open bit 1, birch bit 2
pub const SIGN: Block = 304; // four orientations
pub const SLABS: Block = 320; // four lower halves, then four upper halves
pub const STAIRS: Block = 352; // four materials, eight states each
pub fn is_fence_gate(b: Block) -> bool {
    (FENCE_GATE..FENCE_GATE + 8).contains(&b)
}
pub fn gate_open(b: Block) -> bool {
    is_fence_gate(b) && (b - FENCE_GATE) & 2 != 0
}
pub fn placed_gate(wood: Block, facing: Block) -> Block {
    // SIDE_DIR groups X directions first. The gate spans across the approach.
    FENCE_GATE + wood * 4 + ((facing / 2) ^ 1)
}
pub fn is_sign(b: Block) -> bool {
    (SIGN..SIGN + 4).contains(&b)
}
pub fn slab(mat: Block, top: bool) -> Block {
    if mat < 4 { if top { SLAB_TOP + mat } else { SLAB + mat } } else { SLABS + mat - 4 + if top { 4 } else { 0 } }
}
pub fn stairs(mat: Block, side: Block, top: bool) -> Block {
    if mat < 4 {
        STAIR_BASE + mat * 8 + side + if top { STAIR_FLIP } else { 0 }
    } else {
        STAIRS + (mat - 4) * 8 + side + if top { STAIR_FLIP } else { 0 }
    }
}

pub fn gate_boxes(b: Block, drawing: bool) -> Boxes {
    let mut v = [[0.0; 6]; 8];
    if gate_open(b) && !drawing {
        return (v, 0);
    }
    v[0] = [0.0, 0.0, 0.375, 0.1875, 1.0, 0.625];
    v[1] = [0.8125, 0.0, 0.375, 1.0, 1.0, 0.625];
    v[2] = if gate_open(b) {
        [0.0, 0.375, 0.375, 0.1875, 0.9375, 1.0]
    } else {
        [0.1875, 0.375, 0.4375, 0.8125, if drawing { 0.9375 } else { 1.5 }, 0.5625]
    };
    if (b - FENCE_GATE) & 1 != 0 {
        for bx in &mut v[..3] {
            bx.swap(0, 2);
            bx.swap(3, 5);
        }
    }
    (v, 3)
}

pub fn sign_boxes(b: Block) -> Boxes {
    let mut v = [[0.0; 6]; 8];
    v[0] = [0.4375, 0.0, 0.4375, 0.5625, 0.5, 0.5625];
    v[1] = [0.0625, 0.5, 0.4375, 0.9375, 1.0, 0.5625];
    if (b - SIGN) / 2 == 0 {
        for bx in &mut v[..2] {
            bx.swap(0, 2);
            bx.swap(3, 5);
        }
    }
    (v, 2)
}

// ---------------------------------------------------------------- chests
//
// As in Minecraft, a chest faces the player who placed it and joins a single chest beside
// it that faces the same way, unless the player sneaks. The pairing is stored in the
// state, so a row of chests never becomes ambiguous. A half whose partner is gone or
// mismatched (an explosion, an old save) simply stands as a single chest.

/// bits 0-1 the side the front (latch) faces (see `SIDE_DIR`), bits 2-3 how it joins:
/// 0 single, 1 to the chest one step along +axis, 2 to the one along -axis. The axis is
/// the chest's width: X for chests facing ±Z, Z for chests facing ±X.
pub const CHESTS: Block = 384;
/// Old chests (and cave/village loot chests) have no facing: they stand facing +Z.
const OLD_CHEST_FACING: u8 = 3;
pub fn is_facing_chest(b: Block) -> bool {
    (CHESTS..CHESTS + 12).contains(&b)
}
/// Any chest block, including old and loot chests.
pub fn is_chest(b: Block) -> bool {
    b == CHEST || b == LOOT_CHEST || is_facing_chest(b)
}
pub fn chest(facing: u8, join: u8) -> Block {
    CHESTS + facing as Block + join as Block * 4
}
pub fn chest_facing(b: Block) -> u8 {
    if is_facing_chest(b) { ((b - CHESTS) & 3) as u8 } else { OLD_CHEST_FACING }
}
pub fn chest_join(b: Block) -> u8 {
    if is_facing_chest(b) { ((b - CHESTS) >> 2) as u8 } else { 0 }
}
/// The step from a chest half to the other half its state names.
pub fn join_step(facing: u8, join: u8) -> Option<(i32, i32)> {
    let s = match join {
        1 => 1,
        2 => -1,
        _ => return None,
    };
    Some(if facing < 2 { (0, s) } else { (s, 0) })
}
/// The chest as it stands: a half whose partner does not answer it shows as a single
/// chest, and old chests show as single chests facing +Z. `at` reads its level by offset.
pub fn chest_shown(b: Block, at: impl Fn(i32, i32) -> Block) -> Block {
    let (f, j) = (chest_facing(b), chest_join(b));
    if let Some((dx, dz)) = join_step(f, j) {
        let o = at(dx, dz);
        if is_facing_chest(o) && chest_facing(o) == f && chest_join(o) == 3 - j {
            return b;
        }
    }
    chest(f, 0)
}
/// The boxes of a chest: one body (two halves of a double chest meet flush, so they read
/// as one wide chest), and when drawn, the latch on its front. A double chest has one
/// latch in the middle, drawn by the half on the low side.
pub fn chest_boxes(b: Block, at: impl Fn(i32, i32) -> Block, drawing: bool) -> Boxes {
    const P: f32 = 1.0 / 16.0;
    let b = chest_shown(b, at);
    let (f, j) = (chest_facing(b), chest_join(b));
    // width along `a`, depth along `d`
    let make = |a0: f32, a1: f32, d0: f32, d1: f32, y0: f32, y1: f32| if f >= 2 { [a0, y0, d0, a1, y1, d1] } else { [d0, y0, a0, d1, y1, a1] };
    let mut v = [[0.0f32; 6]; 8];
    let a0 = if j == 2 { 0.0 } else { P };
    let a1 = if j == 1 { 1.0 } else { 15.0 * P };
    v[0] = make(a0, a1, P, 15.0 * P, 0.0, 14.0 * P);
    if !drawing || j == 2 {
        return (v, 1);
    }
    let (l0, l1) = if j == 1 { (15.0 * P, 17.0 * P) } else { (7.0 * P, 9.0 * P) };
    let (d0, d1) = if f & 1 == 1 { (15.0 * P, 1.0) } else { (0.0, P) };
    v[1] = make(l0, l1, d0, d1, 7.0 * P, 11.0 * P);
    (v, 2)
}
/// The state for a chest placed at `p` facing `facing`. Unless `alone` (sneaking), it
/// joins the single chest it was placed against (taking its facing), or else a single
/// chest beside it facing the same way. Returns the state and the partner to rewrite.
pub fn placed_chest(w: &World, p: IVec3, facing: u8, clicked: Option<IVec3>, alone: bool) -> (Block, Option<(IVec3, Block)>) {
    let single = |q: IVec3| {
        let b = w.get_block(q.x, q.y, q.z);
        (b == CHEST || is_facing_chest(b)) && chest_join(chest_shown(b, |dx, dz| w.get_block(q.x + dx, q.y, q.z + dz))) == 0
    };
    if alone {
        return (chest(facing, 0), None);
    }
    let try_join = |f: u8, q: IVec3| -> Option<(Block, Option<(IVec3, Block)>)> {
        let step = q - p;
        if step.y != 0 || !single(q) || chest_facing(w.get_block(q.x, q.y, q.z)) != f {
            return None;
        }
        // the partner has to lie along the chest's width
        let j = [1u8, 2].into_iter().find(|&j| join_step(f, j) == Some((step.x, step.z)))?;
        Some((chest(f, j), Some((q, chest(f, 3 - j)))))
    };
    if let Some(q) = clicked {
        if q.y == p.y {
            let f = chest_facing(w.get_block(q.x, q.y, q.z));
            if let Some(r) = try_join(f, q) {
                return r;
            }
        }
    }
    for j in [1u8, 2] {
        let (dx, dz) = join_step(facing, j).unwrap();
        if let Some(r) = try_join(facing, p + IVec3::new(dx, 0, dz)) {
            return r;
        }
    }
    (chest(facing, 0), None)
}
/// Gives an old facingless chest a state: two old chests that only had each other
/// become one double chest (as the old rules paired them), any other one a single chest.
pub fn upgrade_old_chest(w: &mut World, p: IVec3) {
    if w.get_block(p.x, p.y, p.z) != CHEST {
        return;
    }
    let old = |w: &World, p: IVec3| -> Vec<IVec3> {
        SIDE_DIR.iter().map(|&(x, z)| p + IVec3::new(x, 0, z)).filter(|q| w.get_block(q.x, q.y, q.z) == CHEST).collect()
    };
    let n = old(w, p);
    if n.len() == 1 && old(w, n[0]).len() == 1 {
        let q = n[0];
        let step = q - p;
        // the old double chest stretched along its pair; it faces off that line
        let f = if step.x != 0 { OLD_CHEST_FACING } else { 1 };
        let j = [1u8, 2].into_iter().find(|&j| join_step(f, j) == Some((step.x, step.z))).unwrap();
        w.set_block(p.x, p.y, p.z, chest(f, j));
        w.set_block(q.x, q.y, q.z, chest(f, 3 - j));
    } else {
        w.set_block(p.x, p.y, p.z, chest(OLD_CHEST_FACING, 0));
    }
}
pub fn chest_pair(w: &World, p: IVec3) -> Option<IVec3> {
    let b = w.get_block(p.x, p.y, p.z);
    if !is_facing_chest(b) {
        return None;
    }
    let s = chest_shown(b, |dx, dz| w.get_block(p.x + dx, p.y, p.z + dz));
    join_step(chest_facing(s), chest_join(s)).map(|(dx, dz)| p + IVec3::new(dx, 0, dz))
}

// ---------------------------------------------------------------- beds
//
// A bed is two blocks long and 9/16 of a block high, like Minecraft's: the foot where the
// player aims and the head one step further on, along the way they look.

/// bits 0-1 the side the head lies on from the foot (see `SIDE_DIR`), bit 2 the head half
pub const BEDS: Block = 400;
pub const BED_HEAD: Block = 4;
pub const BED_TOP: f32 = 9.0 / 16.0;
/// Old one-block beds stay where they are, drawn as a short bed with its head to +Z.
pub fn is_bed(b: Block) -> bool {
    b == BED || (BEDS..BEDS + 8).contains(&b)
}
pub fn bed(dir: u8, head: bool) -> Block {
    BEDS + dir as Block + if head { BED_HEAD } else { 0 }
}
pub fn bed_dir(b: Block) -> u8 {
    if b == BED { 3 } else { ((b - BEDS) & 3) as u8 }
}
pub fn bed_head(b: Block) -> bool {
    b == BED || (b - BEDS) & BED_HEAD != 0
}
/// The step to the bed's other half (an old one-block bed has none).
pub fn bed_other(b: Block) -> Option<(i32, i32)> {
    if !is_bed(b) || b == BED {
        return None;
    }
    let (dx, dz) = SIDE_DIR[bed_dir(b) as usize];
    Some(if bed_head(b) { (-dx, -dz) } else { (dx, dz) })
}
/// A mattress on short legs at its outer end (both ends for an old one-block bed).
pub fn bed_boxes(b: Block, drawing: bool) -> Boxes {
    const P: f32 = 1.0 / 16.0;
    let mut v = [[0.0f32; 6]; 8];
    v[0] = [0.0, if drawing { 3.0 * P } else { 0.0 }, 0.0, 1.0, BED_TOP, 1.0];
    if !drawing {
        return (v, 1);
    }
    let end = if bed_head(b) { bed_dir(b) } else { bed_dir(b) ^ 1 };
    let mut n = 1;
    for (cx, cz) in [(0, 0), (1, 0), (0, 1), (1, 1)] {
        let on_end = match end {
            0 => cx == 0,
            1 => cx == 1,
            2 => cz == 0,
            _ => cz == 1,
        };
        if on_end || b == BED {
            let (x0, z0) = (if cx == 0 { 0.0 } else { 13.0 * P }, if cz == 0 { 0.0 } else { 13.0 * P });
            v[n] = [x0, 0.0, z0, x0 + 3.0 * P, 3.0 * P, z0 + 3.0 * P];
            n += 1;
        }
    }
    (v, n)
}

pub fn chest_halves(w: &World, p: IVec3) -> Vec<IVec3> {
    let mut positions = vec![p];
    if let Some(q) = chest_pair(w, p) {
        positions.push(q);
        positions.sort_by_key(|p| (p.x, p.y, p.z));
    }
    positions
}

pub type Signs = HashMap<IVec3, String>;
pub fn encode_signs(signs: &Signs) -> String {
    signs.iter().map(|(p, t)| format!("{} {} {} {}\n", p.x, p.y, p.z, t)).collect()
}
pub fn load_signs(path: &Path) -> io::Result<Signs> {
    let text = match std::fs::read_to_string(path) {
        Ok(t) => t,
        Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(Signs::new()),
        Err(e) => return Err(e),
    };
    let mut signs = Signs::new();
    for line in text.lines() {
        let mut parts = line.splitn(4, ' ');
        let parsed = (|| {
            Some((IVec3::new(parts.next()?.parse().ok()?, parts.next()?.parse().ok()?, parts.next()?.parse().ok()?), parts.next().unwrap_or("")))
        })();
        let Some((p, t)) = parsed else {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "damaged sign data"));
        };
        if t.len() > 60 || !t.is_ascii() || t.bytes().any(|b| b < 32) {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "damaged sign text"));
        }
        signs.insert(p, t.to_string());
    }
    Ok(signs)
}

impl super::Game {
    pub(super) fn chest_stack(&self, i: usize) -> Option<crate::items::Stack> {
        let halves = chest_halves(&self.world, self.chest_at);
        self.chests.get(halves.get(i / crate::items::CHEST_SLOTS)?)?.get(i % crate::items::CHEST_SLOTS).copied()
    }
    pub(super) fn set_chest_stack(&mut self, i: usize, st: crate::items::Stack) {
        let halves = chest_halves(&self.world, self.chest_at);
        if let Some(p) = halves.get(i / crate::items::CHEST_SLOTS) {
            self.chests.entry(*p).or_insert([crate::items::Stack::default(); crate::items::CHEST_SLOTS])[i % crate::items::CHEST_SLOTS] = st;
        }
    }
    pub(super) fn stash_chest(&mut self, mut st: crate::items::Stack) -> crate::items::Stack {
        for p in chest_halves(&self.world, self.chest_at) {
            st = crate::items::stash_in(self.chests.entry(p).or_insert([crate::items::Stack::default(); crate::items::CHEST_SLOTS]), st);
        }
        st
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn floor() -> World {
        World::flat(|_, _, _| None)
    }
    fn place(w: &mut World, p: IVec3, facing: u8, clicked: Option<IVec3>, alone: bool) -> Block {
        let (b, partner) = placed_chest(w, p, facing, clicked, alone);
        w.set_block(p.x, p.y, p.z, b);
        if let Some((o, ob)) = partner {
            w.set_block(o.x, o.y, o.z, ob);
        }
        b
    }
    #[test]
    fn chests_join_side_by_side_like_minecraft_and_a_third_stays_single() {
        let mut w = floor();
        let (a, b, c) = (IVec3::new(0, 10, 0), IVec3::new(1, 10, 0), IVec3::new(2, 10, 0));
        // facing -Z (side 2): the width runs along x
        place(&mut w, a, 2, None, false);
        place(&mut w, b, 2, None, false);
        assert_eq!(chest_pair(&w, a), Some(b));
        assert_eq!(chest_pair(&w, b), Some(a));
        assert_eq!(chest_halves(&w, a), chest_halves(&w, b));
        // a third chest in the row is allowed and stands alone
        assert_eq!(chest_join(place(&mut w, c, 2, None, false)), 0);
        assert_eq!(chest_pair(&w, b), Some(a));
        assert_eq!(chest_halves(&w, c), vec![c]);
        // a chest facing another way, or placed sneaking, does not join
        let d = IVec3::new(0, 10, 5);
        place(&mut w, d, 2, None, false);
        assert_eq!(chest_join(place(&mut w, IVec3::new(1, 10, 5), 0, None, false)), 0);
        assert_eq!(chest_join(place(&mut w, IVec3::new(-1, 10, 5), 2, None, true)), 0);
        // chests in front of each other never join
        assert_eq!(chest_join(place(&mut w, IVec3::new(0, 10, 6), 2, None, false)), 0);
        // clicking the side of a single chest joins it, taking its facing
        let e = IVec3::new(5, 10, 9);
        place(&mut w, e, 1, None, false);
        let f = place(&mut w, e + IVec3::Z, 3, Some(e), false);
        assert_eq!((chest_facing(f), chest_pair(&w, e + IVec3::Z)), (1, Some(e)));
        // a broken half leaves the other standing as a single chest
        w.set_block(a.x, a.y, a.z, AIR);
        assert_eq!(chest_pair(&w, b), None);
        assert_eq!(chest_halves(&w, b), vec![b]);
        let shown = chest_shown(w.get_block(1, 10, 0), |dx, dz| w.get_block(1 + dx, 10, dz));
        assert_eq!(chest_join(shown), 0);
        // and a new chest there can join it again
        place(&mut w, a, 2, None, false);
        assert_eq!(chest_pair(&w, a), Some(b));
    }
    #[test]
    fn old_chests_keep_their_pairs_when_upgraded() {
        let mut w = World::flat(|x, y, z| (y == 10 && ((z == 0 && (x == 0 || x == 1)) || (x == 5 && z == 5))).then_some(CHEST));
        upgrade_old_chest(&mut w, IVec3::new(1, 10, 0));
        assert_eq!(chest_pair(&w, IVec3::new(0, 10, 0)), Some(IVec3::new(1, 10, 0)));
        upgrade_old_chest(&mut w, IVec3::new(5, 10, 5));
        assert_eq!(w.get_block(5, 10, 5), chest(OLD_CHEST_FACING, 0));
        // old pairs along z face +X
        let mut w = World::flat(|x, y, z| (y == 10 && x == 0 && (z == 0 || z == 1)).then_some(CHEST));
        upgrade_old_chest(&mut w, IVec3::new(0, 10, 0));
        assert_eq!(chest_facing(w.get_block(0, 10, 1)), 1);
        assert_eq!(chest_pair(&w, IVec3::new(0, 10, 1)), Some(IVec3::new(0, 10, 0)));
    }
    #[test]
    fn double_chests_meet_flush_with_one_latch_in_the_middle() {
        let (lo, hi) = (chest(2, 1), chest(2, 2));
        let (lo_at, hi_at) = (|dx: i32, _| if dx == 1 { chest(2, 2) } else { AIR }, |dx: i32, _| if dx == -1 { chest(2, 1) } else { AIR });
        let (a, an) = chest_boxes(lo, lo_at, true);
        let (b, bn) = chest_boxes(hi, hi_at, true);
        assert_eq!((an, bn), (2, 1));
        assert_eq!((a[0][3], b[0][0]), (1.0, 0.0));
        // the latch straddles the seam, on the front (-Z)
        assert_eq!((a[1][0], a[1][3], a[1][2]), (15.0 / 16.0, 17.0 / 16.0, 0.0));
        // collision is the same body without the latch
        assert_eq!(chest_boxes(lo, lo_at, false).1, 1);
        // a half with no partner is drawn as a single chest, latch in the middle
        let (s, sn) = chest_boxes(lo, |_, _| AIR, true);
        assert_eq!((sn, s[0][3], s[1][0]), (2, 15.0 / 16.0, 7.0 / 16.0));
    }
    #[test]
    fn beds_are_two_blocks_long_and_nine_sixteenths_high() {
        for dir in 0..4u8 {
            let (foot, head) = (bed(dir, false), bed(dir, true));
            let (fx, fz) = bed_other(foot).unwrap();
            let (hx, hz) = bed_other(head).unwrap();
            assert_eq!((fx, fz), SIDE_DIR[dir as usize]);
            assert_eq!((fx + hx, fz + hz), (0, 0));
            assert!(crate::registry::valid(foot) && crate::registry::valid(head) && is_shaped(foot) && is_solid(head));
            let (v, n) = bed_boxes(foot, false);
            assert_eq!((n, v[0][4]), (1, BED_TOP));
            // a mattress and two legs at its outer end
            assert_eq!(bed_boxes(head, true).1, 3);
            assert_eq!(crate::items::item_of(head), BED);
        }
        assert_eq!(bed_other(BED), None);
        assert_eq!(bed_boxes(BED, true).1, 5);
        for f in 0..4 {
            for j in 0..3 {
                assert!(crate::registry::valid(chest(f, j)) && is_chest(chest(f, j)));
                assert_eq!(crate::items::item_of(chest(f, j)), CHEST);
            }
        }
    }
    #[test]
    fn open_gates_allow_passage_and_signs_round_trip() {
        assert!(gate_boxes(FENCE_GATE, false).1 > 0);
        assert_eq!(gate_boxes(FENCE_GATE + 2, false).1, 0);
        let root = crate::save::tests::TestDir::new("signs");
        let path = root.0.join("signs.txt");
        let signs = Signs::from([(IVec3::new(-2, 150, 3), "Home and farm".into())]);
        std::fs::write(&path, encode_signs(&signs)).unwrap();
        assert_eq!(load_signs(&path).unwrap(), signs);
    }

    #[test]
    fn gates_and_sign_boards_face_the_placement_approach() {
        for facing in 0..4 {
            let axis = if facing < 2 { 0 } else { 2 };
            for wood in 0..2 {
                let (posts, _) = gate_boxes(placed_gate(wood, facing), false);
                assert_eq!((posts[0][axis], posts[0][axis + 3]), (0.375, 0.625));
            }
            let (boards, _) = sign_boxes(SIGN + facing);
            assert_eq!(boards[1][axis + 3] - boards[1][axis], 0.125);
        }
    }
}

use super::*;

impl Game {
    pub(super) fn fits(&self, q: IVec3, b: Block) -> bool {
        if !is_solid(b) {
            return true;
        }
        let (boxes, n) = self.world.shape_at(b, q.x, q.y, q.z);
        let (lo, hi) =
            (self.player.pos - Vec3::new(PLAYER_HW, 0.0, PLAYER_HW), self.player.pos + Vec3::new(PLAYER_HW, self.player.height(), PLAYER_HW));
        boxes[..n].iter().all(|bx| {
            let (min, max) = (Vec3::new(bx[0], bx[1], bx[2]), Vec3::new(bx[3], bx[4], bx[5]));
            !(hi.cmpgt(min).all() && lo.cmplt(max).all()) && !self.mobs.blocks_box(min, max)
        })
    }

    pub(super) fn hiss(&mut self, at: Vec3) {
        self.audio.play_var(Snd::Fizz, at, 1.0);
        for _ in 0..10 {
            self.particles.smoke(&mut self.rng, at, false);
        }
    }

    pub(super) fn light_gate(&mut self, p: IVec3, n: IVec3) {
        let at = (p + n).as_vec3() + 0.5;
        self.particles.sparks(&mut self.rng, at, 8);
        self.audio.play_var(Snd::Dig(audio::Surface::Stone), at, 0.6);
        self.wear_held();
        if self.world.get_block(p.x, p.y, p.z) != CINDERSTONE {
            return;
        }
        let q = p + n;
        let found = [0u8, 1].into_iter().find_map(|axis| frame_opening(&self.world, q, axis).map(|o| (axis, o)));
        let Some((axis, (lo, hi))) = found else {
            self.say("An Ember Gate needs a closed cinderstone frame (an opening 2 wide and 3 high, up to 10)".into());
            return;
        };
        let mut edits = Vec::new();
        for y in lo.y..=hi.y {
            for z in lo.z..=hi.z {
                for x in lo.x..=hi.x {
                    edits.push((IVec3::new(x, y, z), GATE + axis as Block));
                }
            }
        }
        self.world.set_many(&edits);
        self.gates.push(Gate { deep: self.world.is_deep(), at: lo, axis });
        let mid = (lo.as_vec3() + hi.as_vec3()) * 0.5 + 0.5;
        self.audio.play_at(Snd::GateOpen, mid, 1.2, 1.0);
        for _ in 0..30 {
            let o = Vec3::new(self.rng.range(-1.0, 1.0), self.rng.range(-1.5, 1.5), self.rng.range(-1.0, 1.0));
            self.particles.ember(&mut self.rng, mid + o);
        }
        self.say("The Ember Gate opens. Stand in it to cross".into());
    }

    pub(super) fn check_gates(&mut self, cells: &[IVec3]) {
        let mut out: Vec<IVec3> = Vec::new();
        for &c in cells {
            for d in [IVec3::X, IVec3::NEG_X, IVec3::Y, IVec3::NEG_Y, IVec3::Z, IVec3::NEG_Z, IVec3::ZERO] {
                let s = c + d;
                if !is_gate(self.world.get_block(s.x, s.y, s.z)) || out.contains(&s) {
                    continue;
                }
                // every veil cell joined to this one goes
                let mut todo = vec![s];
                out.push(s);
                while let Some(g) = todo.pop() {
                    for d in [IVec3::X, IVec3::NEG_X, IVec3::Y, IVec3::NEG_Y, IVec3::Z, IVec3::NEG_Z] {
                        let n = g + d;
                        if out.len() < 200 && is_gate(self.world.get_block(n.x, n.y, n.z)) && !out.contains(&n) {
                            out.push(n);
                            todo.push(n);
                        }
                    }
                }
            }
        }
        if out.is_empty() {
            return;
        }
        let deep = self.world.is_deep();
        self.gates.retain(|g| g.deep != deep || !out.contains(&g.at));
        let edits: Vec<(IVec3, Block)> = out.iter().map(|&c| (c, AIR)).collect();
        self.world.set_many(&edits);
        let mid = out[0].as_vec3() + 0.5;
        self.audio.play_at(Snd::Fizz, mid, 1.0, 0.6);
    }

    pub(super) fn place(&mut self) {
        if self.combat.bow.is_some() {
            return;
        }
        self.place_cd = 0.2;
        let item = self.inv.hot(self.sel);
        if item != AIR {
            self.swing = SWING;
        }
        let target = self.target();
        // use the held item on a creature in reach: feed, shear, dye, tame, trade
        if let Some((i, t)) = self.mobs.pick(self.player.eye(), self.player.look_dir(), 4.0) {
            if target.map_or(true, |b| t < b.2) && self.use_on_mob(i, item) {
                self.place_cd = 0.3;
                return;
            }
        }
        // get into a boat in reach (unless a block is nearer)
        if self.riding.is_none() {
            if let Some((i, t)) = self.boats.pick(self.player.eye(), self.player.look_dir(), 4.0) {
                if target.map_or(true, |b| t < b.2) {
                    self.riding = Some(i);
                    self.player.vel = Vec3::ZERO;
                    self.player.swimming = false;
                    self.place_cd = 0.4;
                    self.say("Shift to get out".into());
                    return;
                }
            }
        }
        if item == items::BOAT {
            // on water if the crosshair finds some, otherwise on top of the block aimed at
            let yaw = self.player.yaw;
            let at = if let Some(w) = self.water_target(6.0) {
                // (the top of the water: climb out of any water above the cell aimed at)
                let mut y = w.y;
                while self.world.get_block(w.x, y + 1, w.z) == WATER {
                    y += 1;
                }
                Some(Vec3::new(w.x as f32 + 0.5, y as f32 + 0.9, w.z as f32 + 0.5))
            } else {
                target.filter(|t| t.1.y == 1).map(|(p, _, _)| p.as_vec3() + Vec3::new(0.5, 1.0, 0.5))
            };
            match at {
                Some(at) if !collides(&self.world, at + Vec3::Y * 0.05, boat::HW, boat::H) => {
                    self.boats.place(at, yaw);
                    self.spend();
                    self.audio.play_var(Snd::Splash, at, 0.4);
                }
                _ => self.say("A boat goes on water, or on top of a block".into()),
            }
            return;
        }
        // sneak to place a block against a door, chest or trapdoor instead of using it
        let sneak = self.settings.bindings.pressed(6, &self.keys);
        // use the block you are looking at
        if let Some((p, _, _)) = target {
            match self.world.get_block(p.x, p.y, p.z) {
                b if building::is_chest(b) && !sneak => {
                    self.chest_at = p;
                    if b == LOOT_CHEST {
                        // found in a cave and opened for the first time: from now on an
                        // ordinary chest holding what was left in it
                        self.chests.insert(p, loot::roll_chest(self.world.seed, p, self.world.is_deep()));
                        self.world.set_block(p.x, p.y, p.z, building::chest(building::chest_facing(b), 0));
                        self.say("You found a chest".into());
                    } else if b == CHEST {
                        // a chest from an older world takes a facing (and its old partner)
                        building::upgrade_old_chest(&mut self.world, p);
                    }
                    self.chests.entry(p).or_insert([Stack::default(); items::CHEST_SLOTS]);
                    let halves = building::chest_halves(&self.world, p);
                    if halves.iter().any(|p| is_opaque(self.world.get_block(p.x, p.y + 1, p.z))) {
                        self.say("The chest needs room above its lid".into());
                        return;
                    }
                    self.chest_at = halves[0];
                    self.audio.play_var(Snd::Chest, p.as_vec3() + 0.5, 0.8);
                    self.goto(Screen::Chest);
                    return;
                }
                b if building::is_sign(b) && !sneak => {
                    self.chest_at = p;
                    self.sign_edit = self.signs.get(&p).cloned().unwrap_or_default();
                    self.goto(Screen::Sign);
                    return;
                }
                b if building::is_fence_gate(b) && !sneak => {
                    self.place_cd = 0.25;
                    let turned = b ^ 2;
                    if self.fits(p, turned) {
                        self.world.set_block(p.x, p.y, p.z, turned);
                        self.audio.play_var(Snd::Door, p.as_vec3() + 0.5, 0.7);
                    }
                    return;
                }
                b if is_trapdoor(b) && !sneak => {
                    self.place_cd = 0.25;
                    let turned = TRAP_BASE + ((b - TRAP_BASE) ^ TRAP_OPEN);
                    if !self.fits(p, turned) {
                        self.say("Something is in the way of the trapdoor".into());
                    } else if self.world.set_block(p.x, p.y, p.z, turned) {
                        self.audio.play_var(Snd::Door, p.as_vec3() + 0.5, 0.7);
                    }
                    return;
                }
                b if building::is_bed(b) && self.world.is_deep() => {
                    // no sleeping down here
                    self.place_cd = 0.6;
                    self.world.set_block(p.x, p.y, p.z, AIR);
                    if let Some((dx, dz)) = building::bed_other(b) {
                        if building::is_bed(self.world.get_block(p.x + dx, p.y, p.z + dz)) {
                            self.world.set_block(p.x + dx, p.y, p.z + dz, AIR);
                        }
                    }
                    self.explode(p.as_vec3() + 0.5, 3.0);
                    self.say("Beds don't work in the Emberdeep".into());
                    return;
                }
                b if building::is_bed(b) => {
                    self.place_cd = 0.6;
                    self.spawn = p.as_vec3() + Vec3::new(0.5, 1.01, 0.5);
                    if self.sun().1 < 0.35 {
                        self.time = 0.015;
                        self.trade_uses.clear();
                        self.say("You slept until morning. Spawn point set".into());
                    } else {
                        self.say("Spawn point set. You can only sleep at night".into());
                    }
                    return;
                }
                b @ (ANVIL | ENCHANTING_TABLE | WORKBENCH) if !sneak => {
                    self.equipment_open = b != WORKBENCH;
                    self.goto(Screen::Inventory);
                    return;
                }
                // raw food goes on a campfire's free place to cook
                b if catalogue::is_campfire(b) && !sneak && furnace::accepts(furnace::CAMPFIRE, item) => {
                    self.place_cd = 0.25;
                    let fire = self.furnaces.entry(p).or_default();
                    fire.kind = furnace::CAMPFIRE;
                    if fire.put_on(self.inv.slots[self.sel]) {
                        self.spend();
                        self.audio.play_var(Snd::Crackle, p.as_vec3() + 0.5, 0.7);
                    } else {
                        self.say("The campfire is full".into());
                    }
                    return;
                }
                b if furnace::kind_of(b).is_some_and(|k| k != furnace::CAMPFIRE) && !sneak => {
                    self.chest_at = p;
                    self.furnaces.entry(p).or_default().kind = furnace::kind_of(b).unwrap();
                    self.goto(Screen::Furnace);
                    return;
                }
                b if is_door(b) && !sneak => {
                    self.toggle_door(p, b);
                    return;
                }
                _ => {}
            }
        }
        if let Some(crop) = farming::plant(item) {
            if let Some((p, n, _)) = target {
                if n.y == 1 && farming::moisture(self.world.get_block(p.x, p.y, p.z)).is_some() && self.world.get_block(p.x, p.y + 1, p.z) == AIR {
                    if self.world.set_block(p.x, p.y + 1, p.z, crop) {
                        self.spend();
                    }
                    return;
                }
            }
        }
        let bite = items::food(item);
        if bite > 0.0 {
            // a golden apple can be eaten at any time, as in Minecraft
            if self.food < MAX_FOOD - 0.5 || item == items::GOLDEN_APPLE {
                self.spend();
                self.food = (self.food + bite).min(MAX_FOOD);
                self.place_cd = 0.4;
                let pitch = 0.9 + self.rng.f() * 0.2;
                self.audio.play(Snd::Eat, 0.7, pitch);
                match item {
                    // (Minecraft's few seconds of regeneration, given at once)
                    items::GOLDEN_APPLE => {
                        self.health = (self.health + 4.0).min(MAX_HEALTH);
                        self.say("Ate a Golden Apple. You feel restored".into());
                    }
                    // (Minecraft's Hunger effect, as the hunger it would cost)
                    items::ROTTEN_FLESH if self.rng.f() < 0.8 => {
                        self.food = (self.food - 1.0).max(0.0);
                        self.say("The flesh was rotten. You feel queasy".into());
                    }
                    _ => self.say(format!("Ate {}", items::name(item))),
                }
                if item == items::STEW {
                    self.give(items::BOWL, 1);
                }
            }
            return;
        }
        match items::gear(item) {
            Some((items::Kind::Hoe, _)) => {
                // till the ground
                if let Some((p, _, _)) = target {
                    let above = self.world.get_block(p.x, p.y + 1, p.z);
                    let soil = {
                        let b = self.world.get_block(p.x, p.y, p.z);
                        b == DIRT || is_grassy(b)
                    };
                    if soil && (above == AIR || is_tuft(above)) {
                        if above != AIR {
                            self.world.set_block(p.x, p.y + 1, p.z, AIR);
                        }
                        if self.world.set_block(p.x, p.y, p.z, FARMLAND) {
                            self.wear_held();
                        }
                    }
                }
                return;
            }
            _ => {}
        }
        match item {
            items::SEEDS => {
                if let Some((p, n, _)) = target {
                    if n.y == 1 && self.world.get_block(p.x, p.y, p.z) == FARMLAND && self.world.get_block(p.x, p.y + 1, p.z) == AIR {
                        if self.world.set_block(p.x, p.y + 1, p.z, WHEAT_0) {
                            self.spend();
                        }
                    } else {
                        self.say("Seeds need farmland: till grass or dirt with a hoe".into());
                    }
                }
                return;
            }
            items::BUCKET => {
                // scoops up the still water or lava it is pointed at (not running water)
                let hit = raycast_with(&self.world, self.player.eye(), self.player.look_dir(), 6.0, |b| b == WATER || b == LAVA || is_solid(b));
                if let Some((w, _, _)) = hit {
                    let b = self.world.get_block(w.x, w.y, w.z);
                    let full = if b == WATER {
                        items::WATER_BUCKET
                    } else if b == LAVA {
                        items::LAVA_BUCKET
                    } else {
                        return;
                    };
                    if self.world.set_block(w.x, w.y, w.z, AIR) {
                        self.audio.play_var(if b == WATER { Snd::Swim } else { Snd::Fizz }, w.as_vec3() + 0.5, 0.9);
                        self.inv.slots[self.sel] = Stack::new(full, 1);
                    }
                }
                return;
            }
            items::LAVA_BUCKET => {
                if let Some((p, n, _)) = target {
                    let q = p + n;
                    let here = q.as_vec3() + 0.5;
                    let was = self.world.get_block(q.x, q.y, q.z);
                    if is_water(was) {
                        // poured into water it hardens at once
                        if self.world.set_block(q.x, q.y, q.z, if was == WATER { CINDERSTONE } else { COBBLE }) {
                            self.inv.slots[self.sel] = Stack::new(items::BUCKET, 1);
                            self.hiss(here);
                        }
                    } else if is_replaceable(was) && self.world.set_block(q.x, q.y, q.z, LAVA) {
                        self.audio.play_var(Snd::Fizz, here, 0.6);
                        self.inv.slots[self.sel] = Stack::new(items::BUCKET, 1);
                    }
                }
                return;
            }
            items::WATER_BUCKET => {
                if let Some((p, n, _)) = target {
                    let q = p + n;
                    let here = q.as_vec3() + 0.5;
                    if self.world.is_deep() {
                        // the heat down here boils it away at once
                        self.inv.slots[self.sel] = Stack::new(items::BUCKET, 1);
                        self.audio.play_var(Snd::Fizz, here, 1.0);
                        for _ in 0..12 {
                            self.particles.smoke(&mut self.rng, here, false);
                        }
                        self.say("The water boils away".into());
                        return;
                    }
                    let was = self.world.get_block(q.x, q.y, q.z);
                    if was == LAVA {
                        // water poured on lava: it hardens into cinderstone
                        if self.world.set_block(q.x, q.y, q.z, CINDERSTONE) {
                            self.inv.slots[self.sel] = Stack::new(items::BUCKET, 1);
                            self.hiss(here);
                        }
                    } else if is_replaceable(was) && self.world.set_block(q.x, q.y, q.z, WATER) {
                        self.audio.play_var(Snd::Splash, here, 0.5);
                        self.inv.slots[self.sel] = Stack::new(items::BUCKET, 1);
                        // and the lava it touches hardens
                        let mut hard = Vec::new();
                        for d in [IVec3::X, IVec3::NEG_X, IVec3::Z, IVec3::NEG_Z, IVec3::NEG_Y, IVec3::Y] {
                            let c = q + d;
                            if self.world.get_block(c.x, c.y, c.z) == LAVA {
                                hard.push((c, CINDERSTONE));
                            }
                        }
                        if !hard.is_empty() {
                            self.world.set_many(&hard);
                            self.hiss(here);
                        }
                    }
                }
                return;
            }
            items::STRIKER => {
                if let Some((p, n, _)) = target {
                    self.light_gate(p, n);
                }
                return;
            }
            items::SHIELD => return,
            items::SNOWBALL => {
                self.throw_snowball();
                return;
            }
            items::BONE_MEAL => {
                if let Some((p, _, _)) = target {
                    if self.bone_meal(p) {
                        self.spend();
                    }
                }
                return;
            }
            items::BOW => {
                if self.combat.bow.is_none() {
                    self.combat.bow = Some((self.sel, 0.0));
                }
                return;
            }

            _ => {}
        }
        if matches!(item, items::OAK_DOOR | items::BIRCH_DOOR) {
            self.place_door(target, item);
            return;
        }
        let Some((p, n, t)) = target else { return };
        let q = p + n;
        let clicked = self.world.get_block(p.x, p.y, p.z);
        let d = self.player.look_dir();
        // the side of the cell you look towards, and the side the clicked wall is on
        let facing: Block = if d.x.abs() > d.z.abs() { (d.x > 0.0) as Block } else { 2 + (d.z > 0.0) as Block };
        let wall: Block = if n.x == 1 {
            0
        } else if n.x == -1 {
            1
        } else if n.z == 1 {
            2
        } else {
            3
        };
        // pointing at the upper half of a side, or at an underside, places the high variant
        let high = n.y == -1 || (n.y == 0 && (self.player.eye() + d * t).y - p.y as f32 > 0.5);
        let (at, block): (IVec3, Block) = match item {
            items::OAK_GATE | items::BIRCH_GATE => (q, building::placed_gate(item - items::OAK_GATE, facing)),
            items::SIGN_ITEM => (q, building::SIGN + facing),
            items::LADDER => {
                if n.y != 0 || !is_opaque(clicked) {
                    self.say("A ladder hangs on the side of a solid block".into());
                    return;
                }
                (q, LADDER_BASE + wall)
            }
            TORCH => {
                // on top of a block it stands, on a side it hangs from the wall
                let ok = is_solid(clicked) && !is_door(clicked) && !is_ladder(clicked) && !is_trapdoor(clicked) && !is_fence(clicked)
                    || (is_fence(clicked) && n.y == 1);
                if n.y == -1 || !ok || (n.y == 0 && !is_opaque(clicked)) {
                    self.say("A torch stands on a block or hangs on its side".into());
                    return;
                }
                (q, if n.y == 1 { TORCH } else { WALL_TORCH + wall })
            }
            items::TRAPDOOR => (q, TRAP_BASE + if n.y == 0 { wall } else { facing } + if high { TRAP_TOP } else { 0 }),
            i if items::stair_material(i).is_some() => (q, building::stairs(items::stair_material(i).unwrap(), facing, high)),
            i if items::slab_material(i).is_some() => {
                // two halves of a kind make a full block again
                let mat = items::slab_material(i).unwrap();
                let full = SHAPE_FULL[mat as usize];
                let same = |b: Block| is_slab(b) && slab_mat(b) == mat;
                if same(clicked) && n.y == if slab_top(clicked) { -1 } else { 1 } {
                    (p, full)
                } else if same(self.world.get_block(q.x, q.y, q.z)) {
                    (q, full)
                } else {
                    (q, building::slab(mat, high))
                }
            }
            // a lantern hangs when placed against the underside of a block
            i if i == items::cat(catalogue::LANTERN) && n.y == -1 => {
                if !is_solid(clicked) || is_shaped(clicked) {
                    self.say("A lantern hangs from the underside of a solid block".into());
                    return;
                }
                (q, catalogue::block(catalogue::HANGING_LANTERN))
            }
            items::REED_ITEM => {
                // (Minecraft's sugar cane: on sand, dirt or grass with water beside the
                // ground, or on top of more reeds)
                let ground = clicked == SAND || clicked == RED_SAND || clicked == DIRT || is_grassy(clicked);
                let wet = SIDE_DIR.iter().any(|&(dx, dz)| is_water(self.world.get_block(p.x + dx, p.y, p.z + dz)));
                if n.y != 1 || !(clicked == REEDS || (ground && wet)) {
                    self.say("Reeds grow on sand, dirt or grass beside water".into());
                    return;
                }
                (q, REEDS)
            }
            CHEST => {
                let clicked_at = (n.y == 0 && building::is_chest(clicked)).then_some(p);
                let (state, partner) = building::placed_chest(&self.world, q, (facing ^ 1) as u8, clicked_at, sneak);
                let cur = self.world.get_block(q.x, q.y, q.z);
                if !is_replaceable(cur) || !self.fits(q, state) {
                    return;
                }
                if self.world.set_block(q.x, q.y, q.z, state) {
                    if let Some((o, b)) = partner {
                        self.world.set_block(o.x, o.y, o.z, b);
                    }
                    self.spend();
                    self.weather.dirty();
                    self.audio.play_var(Snd::Place(audio::surface(state)), q.as_vec3() + 0.5, 0.8);
                }
                return;
            }
            BED => {
                self.place_bed(q, facing as u8);
                return;
            }
            i if items::is_sapling_item(i) => {
                let soil = clicked == DIRT || clicked == FARMLAND || is_grassy(clicked);
                if n.y != 1 || !soil {
                    self.say("A sapling needs grass or dirt to grow in".into());
                    return;
                }
                (q, items::block_of(i))
            }
            i if items::is_block(i) => (q, items::block_of(i)),
            _ => return,
        };
        let cur = self.world.get_block(at.x, at.y, at.z);
        // (a slab is only ever replaced by the full block its second half makes)
        let joins = is_slab(cur) && items::slab_material(item).is_some() && block == SHAPE_FULL[slab_mat(cur) as usize];
        if !is_replaceable(cur) && !joins {
            return;
        }
        if !self.fits(at, block) {
            return;
        }
        if self.world.set_block(at.x, at.y, at.z, block) {
            self.spend();
            if building::is_sign(block) {
                self.chest_at = at;
                self.sign_edit = String::new();
                self.goto(Screen::Sign);
            }
            self.weather.dirty();
            self.audio.play_var(Snd::Place(audio::surface(block)), at.as_vec3() + 0.5, 0.8);
        }
    }

    /// The foot goes where the player aims, the head one step further the way they look.
    pub(super) fn place_bed(&mut self, foot: IVec3, dir: u8) {
        let (dx, dz) = SIDE_DIR[dir as usize];
        let head = foot + IVec3::new(dx, 0, dz);
        let w = &self.world;
        let room = |c: IVec3| is_replaceable(w.get_block(c.x, c.y, c.z)) && is_solid(w.get_block(c.x, c.y - 1, c.z));
        if !w.is_loaded(head.x, head.z) || !room(foot) || !room(head) {
            self.say("A bed needs two blocks of firm floor".into());
            return;
        }
        let (fb, hb) = (building::bed(dir, false), building::bed(dir, true));
        if !self.fits(foot, fb) || !self.fits(head, hb) {
            return;
        }
        if self.world.set_block(foot.x, foot.y, foot.z, fb) {
            self.world.set_block(head.x, head.y, head.z, hb);
            self.spend();
            self.audio.play_var(Snd::Place(audio::surface(fb)), foot.as_vec3() + 0.5, 0.8);
        }
    }

    /// Bone meal on a crop ripens it a stage or two, on a sapling may grow its tree at
    /// once, and on grass sprouts tufts and flowers around. True if it was used.
    pub(super) fn bone_meal(&mut self, p: IVec3) -> bool {
        let b = self.world.get_block(p.x, p.y, p.z);
        let used = if let Some((_, stage)) = farming::crop(b) {
            if stage >= 3 {
                return false;
            }
            let grow = (1 + (self.rng.next() % 2) as Block).min(3 - stage as Block);
            self.world.set_block(p.x, p.y, p.z, b + grow)
        } else if is_sapling(b) {
            // (Minecraft: a 45% chance to advance; its saplings need two advances)
            if self.rng.f() < 0.45 {
                if let Some(tree) = settle::grow(&self.world, p, b, &mut self.rng) {
                    self.world.set_many(&tree);
                }
            }
            true
        } else if is_grassy(b) && self.world.get_block(p.x, p.y + 1, p.z) == AIR {
            for _ in 0..12 {
                let c = p + IVec3::new((self.rng.next() % 7) as i32 - 3, 0, (self.rng.next() % 7) as i32 - 3);
                if is_grassy(self.world.get_block(c.x, c.y, c.z)) && self.world.get_block(c.x, c.y + 1, c.z) == AIR {
                    let plant = match self.rng.next() % 8 {
                        0 => FLOWER_RED,
                        1 => FLOWER_YELLOW,
                        _ => TALL_GRASS,
                    };
                    self.world.set_block(c.x, c.y + 1, c.z, plant);
                }
            }
            true
        } else {
            false
        };
        if used {
            self.place_cd = 0.25;
            self.particles.dust(&mut self.rng, p.as_vec3() + Vec3::new(0.5, 1.1, 0.5), [120, 220, 90], 8, 0.4);
        }
        used
    }

    /// Throws the held snowball: it knocks back what it hits, but does no harm.
    pub(super) fn throw_snowball(&mut self) {
        let dir = self.player.look_dir();
        self.arrows.push(super::Arrow {
            pos: self.player.eye() + dir * 0.4,
            vel: dir * 26.0 + self.player.vel * 0.5,
            life: 5.0,
            hostile: false,
            fire: false,
            snow: true,
            damage: 0,
        });
        self.spend();
        self.swing = super::SWING;
        self.place_cd = 0.25;
    }

    pub(super) fn door_blocked(&self, p: IVec3, b: Block) -> bool {
        let q = shape_boxes(b, |_, _| AIR).0[0];
        let min = p.as_vec3() + Vec3::new(q[0], 0.0, q[2]);
        let max = p.as_vec3() + Vec3::new(q[3], 2.0, q[5]);
        let (lo, hi) =
            (self.player.pos - Vec3::new(PLAYER_HW, 0.0, PLAYER_HW), self.player.pos + Vec3::new(PLAYER_HW, self.player.height(), PLAYER_HW));
        (hi.cmpgt(min).all() && lo.cmplt(max).all()) || self.mobs.blocks_box(min, max)
    }

    pub(super) fn toggle_door(&mut self, p: IVec3, b: Block) {
        self.place_cd = 0.25;
        let y = if door_upper(b) { p.y - 1 } else { p.y };
        let (lower, upper) = (self.world.get_block(p.x, y, p.z), self.world.get_block(p.x, y + 1, p.z));
        if !is_door(lower) || !is_door(upper) {
            return;
        }
        let (lower, upper) = (lower ^ DOOR_OPEN, upper ^ DOOR_OPEN);
        if self.door_blocked(IVec3::new(p.x, y, p.z), lower) {
            self.say("Something is in the way of the door".into());
            return;
        }
        if self.world.set_pair(p.x, y, p.z, lower, upper) {
            self.audio.play_var(Snd::Door, p.as_vec3() + 0.5, 0.75);
        }
    }

    pub(super) fn place_door(&mut self, target: Option<(IVec3, IVec3, f32)>, item: Item) {
        let Some((p, n, _)) = target else { return };
        let q = p + n;
        let at = |w: &World, dx: i32, dy: i32, dz: i32| w.get_block(q.x + dx, q.y + dy, q.z + dz);
        let full = |b: Block| is_solid(b) && !is_shaped(b);
        let w = &self.world;
        if !is_replaceable(at(w, 0, 0, 0)) || !is_replaceable(at(w, 0, 1, 0)) || !full(at(w, 0, -1, 0)) {
            self.say("A door needs firm ground and two blocks of room".into());
            return;
        }
        let d = self.player.look_dir();
        let side = if d.x.abs() > d.z.abs() { (d.x < 0.0) as u8 } else { 2 + (d.z < 0.0) as u8 };
        // the hinge goes on the left, unless that would leave it hanging in the open
        // or there is a door there already (so two doors meet in the middle)
        let beside = |s: u8| at(w, SIDE_DIR[s as usize].0, 0, SIDE_DIR[s as usize].1);
        let (left, right) = (beside(SIDE_LEFT[side as usize]), beside(SIDE_RIGHT[side as usize]));
        let hinge_right = !is_door(right) && (is_door(left) || (full(right) && !full(left)));
        let lower = DOOR_BASE + side as Block + if hinge_right { DOOR_HINGE_RIGHT } else { 0 } + (item - items::OAK_DOOR) * 32;
        if self.door_blocked(q, lower) {
            return;
        }
        if self.world.set_pair(q.x, q.y, q.z, lower, lower + DOOR_UPPER) {
            self.spend();
        }
    }
}
