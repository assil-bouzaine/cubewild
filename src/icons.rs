//! Icons of the items added from Minecraft's item list (`items::BOWL` onwards), drawn
//! procedurally like every other texture. Layer `textures::MORE_ICONS` + k.

const CLEAR: [u8; 4] = [0, 0, 0, 0];

fn hash(k: usize, x: i32, y: i32) -> f32 {
    let mut v = (k as u32).wrapping_mul(0x9e37_79b9) ^ (x as u32).wrapping_mul(0x85eb_ca6b) ^ (y as u32).wrapping_mul(0xc2b2_ae35).rotate_left(7);
    v ^= v >> 15;
    v = v.wrapping_mul(0x2c1b_3c6d);
    v ^= v >> 12;
    (v & 0xffff) as f32 / 65535.0
}

fn shade(c: [f32; 3], k: f32) -> [u8; 4] {
    let f = |v: f32| (v * k).clamp(0.0, 255.0) as u8;
    [f(c[0]), f(c[1]), f(c[2]), 255]
}

/// A round body lit from the top left: rim darker, a highlight up and to the left.
fn ball(x: i32, y: i32, cx: f32, cy: f32, r: f32, c: [f32; 3]) -> Option<[u8; 4]> {
    let (dx, dy) = (x as f32 + 0.5 - cx, y as f32 + 0.5 - cy);
    let d = (dx * dx + dy * dy).sqrt();
    if d > r {
        return None;
    }
    let light = 1.0 - (dx + dy) / (2.0 * r) * 0.35;
    let k = if d > r - 1.0 { 0.68 } else if (dx + r * 0.4).abs() + (dy + r * 0.4).abs() < 1.3 { 1.35 } else { light };
    Some(shade(c, k))
}

/// An apple, stalk and leaf, in `c`.
fn apple(x: i32, y: i32, c: [f32; 3], leaf: [f32; 3]) -> [u8; 4] {
    if x == 8 && (2..=4).contains(&y) {
        return shade([96.0, 64.0, 32.0], 1.0);
    }
    if (9..=11).contains(&x) && y == 3 || x == 10 && y == 2 {
        return shade(leaf, if x == 11 { 0.8 } else { 1.0 });
    }
    // two lobes make the dip at the top
    ball(x, y, 6.6, 9.6, 5.2, c).or_else(|| ball(x, y, 9.4, 9.6, 5.2, c)).unwrap_or(CLEAR)
}

/// A small heap of lumps (nuggets) in `c`.
fn nuggets(x: i32, y: i32, c: [f32; 3]) -> [u8; 4] {
    [(5.5, 7.5, 2.6), (10.5, 8.5, 2.4), (7.5, 11.5, 2.8), (11.0, 12.5, 1.8)]
        .into_iter()
        .find_map(|(cx, cy, r)| ball(x, y, cx, cy, r, c))
        .unwrap_or(CLEAR)
}

pub fn pixel(k: usize, x: i32, y: i32) -> [u8; 4] {
    let n = hash(k, x, y);
    let gold = [246.0, 196.0, 52.0];
    match k {
        // bowl: a wooden half-round with its dark inside showing over the rim
        0 => {
            let wood = [150.0, 104.0, 56.0];
            let (dx, dy) = (x as f32 + 0.5 - 8.0, y as f32 + 0.5 - 7.0);
            let inside = y == 7 && (3..=12).contains(&x);
            let body = dy >= 0.0 && dx * dx / 42.0 + dy * dy / 30.0 <= 1.0;
            if inside {
                shade([86.0, 58.0, 30.0], 0.9 + n * 0.1)
            } else if y == 6 && (2..=13).contains(&x) {
                shade(wood, 1.15)
            } else if body {
                shade(wood, if dx * dx / 42.0 + dy * dy / 30.0 > 0.7 { 0.7 } else { 0.9 + n * 0.12 - dx * 0.015 })
            } else {
                CLEAR
            }
        }
        1 => apple(x, y, [214.0, 36.0, 40.0], [70.0, 150.0, 44.0]),
        2 => apple(x, y, gold, [230.0, 210.0, 90.0]),
        3 => nuggets(x, y, [208.0, 210.0, 216.0]),
        4 => nuggets(x, y, gold),
        // golden carrot: a tapering root from the bottom left, leaves at the top right
        5 => {
            let (u, v) = (x as f32 + 0.5, y as f32 + 0.5);
            // distance along the root (0 at the tip, 1 at the crown) and across it
            let along = ((u - 2.5) + (13.5 - v)) / 18.0;
            let across = ((u - 2.5) - (13.5 - v)).abs() / 2.0f32.sqrt();
            if (0.0..=0.82).contains(&along) && across <= 0.4 + along * 2.6 {
                let ring = (along * 18.0) as i32 % 4 == 0;
                shade(gold, if across > along * 2.6 - 0.2 { 0.72 } else if ring { 0.84 } else { 1.0 + n * 0.08 })
            } else if along > 0.8 && along < 1.15 && across <= 2.5 && (x + y) % 2 == 0 {
                shade([90.0, 170.0, 60.0], 0.9 + n * 0.2)
            } else {
                CLEAR
            }
        }
        // bone meal: a heap of white powder
        6 => {
            let (dx, dy) = (x as f32 + 0.5 - 8.0, y as f32 + 0.5 - 13.0);
            let heap = dy <= 0.5 && dx * dx / 36.0 + dy * dy / 49.0 <= 1.0;
            if heap {
                shade([238.0, 236.0, 226.0], if n > 0.8 { 0.78 } else { 0.92 + n * 0.1 - dx * 0.01 })
            } else if n > 0.95 && (3..=13).contains(&x) && (5..=13).contains(&y) {
                shade([238.0, 236.0, 226.0], 0.85)
            } else {
                CLEAR
            }
        }
        // rotten flesh: a ragged grey-green lump with dark patches
        7 => {
            let (dx, dy) = (x as f32 + 0.5 - 8.0, y as f32 + 0.5 - 8.5);
            let ragged = 5.4 + (hash(7, x / 2, y / 2) - 0.5) * 2.2;
            if (dx * dx * 0.7 + dy * dy).sqrt() <= ragged {
                if n > 0.75 {
                    shade([92.0, 70.0, 48.0], 0.9)
                } else if hash(17, x / 3, y / 3) > 0.6 {
                    shade([150.0, 120.0, 90.0], 0.9 + n * 0.1)
                } else {
                    shade([118.0, 140.0, 82.0], 0.85 + n * 0.15)
                }
            } else {
                CLEAR
            }
        }
        // snowball
        8 => ball(x, y, 8.0, 8.5, 5.5, [240.0, 246.0, 252.0]).map(|c| if n > 0.85 { [c[0] - 18, c[1] - 12, c[2], 255] } else { c }).unwrap_or(CLEAR),
        // paper: a sheet with a turned corner and faint lines
        9 => match (x, y) {
            (3..=12, 2..=13) if x - 3 + 13 - y < 3 => shade([200.0, 200.0, 190.0], 1.0),
            (3..=12, 2..=13) if y % 3 == 0 && (5..=10).contains(&x) => shade([190.0, 196.0, 206.0], 1.0),
            (3..=12, 2..=13) => shade([244.0, 242.0, 230.0], 0.96 + n * 0.04),
            _ => CLEAR,
        },
        // book: a leather cover and the page edges
        10 => match (x, y) {
            (3..=12, 2..=13) if x >= 11 => shade([238.0, 234.0, 214.0], if y % 2 == 0 { 0.9 } else { 1.0 }),
            (3..=12, 2..=13) if x == 3 || y == 2 || y == 13 => shade([90.0, 50.0, 30.0], 1.0),
            (3..=12, 2..=13) if (5..=8).contains(&x) && (5..=6).contains(&y) => shade([220.0, 190.0, 80.0], 1.0),
            (3..=12, 2..=13) => shade([130.0, 74.0, 42.0], 0.92 + n * 0.1),
            _ => CLEAR,
        },
        // clay ball
        11 => ball(x, y, 8.0, 8.5, 5.0, [160.0, 166.0, 180.0]).unwrap_or(CLEAR),
        // brick: a fired block seen at an angle
        12 => match (x, y) {
            (2..=13, 6..=11) if y == 6 => shade([200.0, 110.0, 80.0], 1.0),
            (2..=13, 6..=11) if x == 13 || y == 11 => shade([120.0, 50.0, 36.0], 1.0),
            (2..=13, 6..=11) => shade([168.0, 78.0, 56.0], 0.92 + n * 0.1),
            _ => CLEAR,
        },
        // reeds: three jointed green stalks
        13 => {
            let stalk = matches!(x, 4 | 5 | 8 | 9 | 12) && y >= 1 + (x % 3);
            if !stalk {
                CLEAR
            } else if y % 5 == 0 {
                shade([150.0, 190.0, 90.0], 1.0)
            } else {
                shade([110.0, 170.0, 70.0], if x % 2 == 1 { 0.85 } else { 1.0 })
            }
        }
        _ => CLEAR,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn every_new_item_has_a_visible_icon_inside_its_layer() {
        for k in 0..(crate::items::PLAIN_END - crate::items::BOWL) as usize {
            assert!(k < crate::textures::MORE_ICON_COUNT);
            let mut shown = 0;
            for y in 0..16 {
                for x in 0..16 {
                    shown += (super::pixel(k, x, y)[3] > 0) as u32;
                }
            }
            assert!((20..240).contains(&shown), "icon {k} has {shown} visible pixels");
        }
    }
}
