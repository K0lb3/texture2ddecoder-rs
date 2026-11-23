// Synthetic BC6H regressions checked against the independent bcdec reference.
//
// The block below is a random synthetic block, not game data. Expected eight-bit
// channels were computed from `bcdec_bc6h_half` at revision
// 80859ed3b7afb1c527a2a99d70c61457bea72d0c, converting each half to float and
// applying the same [0, 255] clamp and truncation as the public API.

use texture2ddecoder::{decode_bc6_block_signed, decode_bc6_block_unsigned};

const BLOCK: [u8; 16] = [
    0x09, 0x02, 0xe0, 0xf0, 0xd8, 0xd6, 0xce, 0x98, 0x65, 0x6d, 0x9c, 0x2b, 0x63, 0x00, 0xe9, 0xc5,
];

fn rgb(pixel: u32) -> [u8; 3] {
    // color() stores little-endian [B, G, R, A].
    [(pixel >> 16) as u8, (pixel >> 8) as u8, pixel as u8]
}

#[test]
fn unsigned_maximum_threshold_is_full_range() {
    let mut pixels = [0u32; 16];
    decode_bc6_block_unsigned(&BLOCK, &mut pixels);
    let expected = [
        [0, 203, 255],
        [0, 255, 255],
        [12, 76, 255],
        [0, 255, 255],
        [0, 255, 255],
        [0, 203, 255],
        [0, 255, 255],
        [0, 255, 255],
        [0, 255, 255],
        [0, 255, 255],
        [0, 255, 255],
        [0, 255, 255],
        [12, 76, 255],
        [1, 255, 255],
        [0, 255, 255],
        [0, 255, 255],
    ];
    for (index, pixel) in pixels.iter().enumerate() {
        assert_eq!(rgb(*pixel), expected[index], "pixel {index}");
    }
}

#[test]
fn signed_endpoints_keep_their_sign_and_magnitude() {
    let mut pixels = [0u32; 16];
    decode_bc6_block_signed(&BLOCK, &mut pixels);
    let expected = [
        [96, 0, 0],
        [7, 0, 0],
        [255, 255, 0],
        [0, 0, 0],
        [7, 0, 0],
        [96, 0, 0],
        [45, 0, 0],
        [0, 0, 0],
        [1, 0, 0],
        [1, 0, 0],
        [0, 0, 0],
        [0, 0, 0],
        [255, 255, 0],
        [255, 0, 0],
        [0, 0, 0],
        [5, 0, 0],
    ];
    for (index, pixel) in pixels.iter().enumerate() {
        assert_eq!(rgb(*pixel), expected[index], "pixel {index}");
    }
}
