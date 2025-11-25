use crate::memory::{self, MemoryMap, SCX, SCY};
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Color {
    C0,
    C1,
    C2,
    C3,
}
impl Color {
    pub fn as_bits(self) -> u8 {
        match self {
            Color::C0 => 0x00,
            Color::C1 => 0x01,
            Color::C2 => 0x02,
            Color::C3 => 0x03,
        }
    }
}

pub struct BGWindow {
    enabled: bool,
    scrollx: u8,
    scrolly: u8,
}
impl BGWindow {
    pub fn update(&mut self, scx: SCX, scy: SCY) {
        self.scrollx = scx.scroll_x;
        self.scrolly = scy.scroll_y;
    }
    pub fn get(self) -> (u8, u8) {
        let bottom = self.scrolly.wrapping_add(143);
        let right = self.scrollx.wrapping_add(159);
        (bottom, right)
    }
}

pub struct BackGround {
    pixels: [Color; 65536],
    enabled: bool,
    window: BGWindow,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct TileLine {
    line: [Color; 8],
}
impl TileLine {
    fn bits_bool(byte: u8) -> impl Iterator<Item = bool> {
        (0..8).rev().map(move |i| ((byte >> i) & 1) != 0)
    }
    pub fn new(byte_one: u8, byte_two: u8) -> Self {
        let mut line: [Color; 8] = [Color::C0; 8];
        for (i, (bit_a, bit_b)) in Self::bits_bool(byte_one)
            .zip(Self::bits_bool(byte_two))
            .enumerate()
        {
            line[i] = match (bit_b, bit_a) {
                (false, false) => Color::C0,
                (false, true) => Color::C1,
                (true, false) => Color::C2,
                (true, true) => Color::C3,
            };
        }
        Self { line }
    }
    pub fn as_bytes(&self) -> u16 {
        let mut hi: u8 = 0;
        let mut lo: u8 = 0;
        for (i, color) in self.line.iter().enumerate() {
            let (bit1, bit0) = match color {
                Color::C0 => (0, 0),
                Color::C1 => (0, 1),
                Color::C2 => (1, 0),
                Color::C3 => (1, 1),
            };
            let shift = 7 - i;
            lo |= bit0 << shift;
            hi |= bit1 << shift;
        }
        ((hi as u16) << 8) | lo as u16
    }
}

#[derive(PartialEq, Eq)]
pub struct Tile {
    lines: [TileLine; 8],
}
impl Tile {
    pub fn new(bytes: [u8; 16]) -> Self {
        let mut lines: [TileLine; 8] = [TileLine::new(0, 0); 8];
        for i in 0..8 {
            let byte_one = bytes[i * 2];
            let byte_two = bytes[i * 2 + 1];
            lines[i] = TileLine::new(byte_one, byte_two);
        }
        Self { lines }
    }
}

enum ObjectSize {
    EightXEight,
    EightXSixteen,
}
struct Object {
    size: ObjectSize,
    tile1: Tile,
    tile2: Option<Tile>,
}

#[derive(Copy, Clone)]
pub struct ScanLine {
    pub pixels: [Color; 160],
}
impl ScanLine {
    pub fn new() -> Self {
        Self {
            pixels: [Color::C0; 160],
        }
    }
}

impl Default for ScanLine {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone)]
pub struct Display {
    pub lines: [ScanLine; 144],
}

impl Display {
    pub fn new() -> Self {
        Self {
            lines: [ScanLine::new(); 144],
        }
    }
    #[inline(always)]
    pub fn set(&mut self, color: Color, x: usize, y: usize) {
        debug_assert!(x <= 160 && y <= 144);
        self.lines[y].pixels[x] = color;
    }
    pub fn update(&mut self, memory: &memory::MemoryMap) {}
    fn load_tiles_from_vram(memory: &memory::MemoryMap) {}
    pub fn test_pattern(&mut self) {
        let mut i = 0;
        for y in 0..144 {
            for x in 0..160 {
                self.set(
                    match i {
                        0 => {
                            i += 1;
                            Color::C0
                        }
                        1 => {
                            i += 1;
                            Color::C1
                        }
                        2 => {
                            i += 1;
                            Color::C2
                        }
                        3 => {
                            i = 0;
                            Color::C3
                        }
                        _ => panic!("Invalid color index"),
                    },
                    x,
                    y,
                );
            }
        }
    }
}

impl Default for Display {
    fn default() -> Self {
        Self::new()
    }
}
