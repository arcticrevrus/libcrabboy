use crate::graphics::{self, Tile};

pub trait Memory {
    fn read(&self, address: u16) -> u8;
    fn write(&mut self, address: u16, value: u8);
}

macro_rules! memory_region {
    (
        $name:ident,    // Struct name
        $size:expr,     // Size of the memory array
        $offset:expr    // Address offset
    ) => {
        #[derive(Copy, Clone, PartialEq)]
        struct $name {
            memory: [u8; $size],
        }

        impl $name {
            pub fn new() -> Self {
                Self { memory: [0; $size] }
            }
        }

        impl Memory for $name {
            fn read(&self, address: u16) -> u8 {
                self.memory[(address - $offset) as usize]
            }

            fn write(&mut self, address: u16, value: u8) {
                self.memory[(address - $offset) as usize] = value;
            }
        }
    };
}

memory_region!(Rom0, 0x4000, 0x0000);
memory_region!(RomX, 0x4000, 0x4000);
#[derive(Copy, Clone, PartialEq)]
pub struct TileData {
    block0: [u8; 0x0800],
    block1: [u8; 0x0800],
    block2: [u8; 0x0800],
}
impl TileData {
    pub fn get_tile(self, lcdc: LCDC, id: u8) -> graphics::Tile {
        let lcdc4: bool = lcdc.lcdcontrol & 0b0001_0000 == 0b0001_0000;
        let start = id as usize;
        let end = 16usize;
        let bytes: [u8; 16] = match id {
            0..=127 => match lcdc4 {
                true => self.block0,
                false => self.block1,
            },
            128..=255 => self.block1,
        }[start..=end]
            .try_into()
            .expect("Error getting tiledata");
        graphics::Tile::new(bytes)
    }
}
#[derive(Copy, Clone, PartialEq)]
pub struct TileMap {
    pub map: [u8; 1024],
}
#[derive(Copy, Clone, PartialEq)]
pub struct VRam {
    pub tiledata: TileData,
    pub tilemap0: TileMap,
    pub tilemap1: TileMap,
}

impl Default for VRam {
    fn default() -> Self {
        Self::new()
    }
}

impl VRam {
    pub fn new() -> Self {
        Self {
            tiledata: TileData {
                block0: [0; 0x0800],
                block1: [0; 0x0800],
                block2: [0; 0x0800],
            },
            tilemap0: TileMap { map: [0; 0x0400] },
            tilemap1: TileMap { map: [0; 0x0400] },
        }
    }
}
impl Memory for VRam {
    fn read(&self, address: u16) -> u8 {
        match address {
            0x8000..=0x87FF => self.tiledata.block0[(address - 0x8000) as usize],
            0x8800..=0x8FFF => self.tiledata.block1[(address - 0x8800) as usize],
            0x9000..=0x97FF => self.tiledata.block2[(address - 0x9000) as usize],
            0x9800..=0x9BFF => self.tilemap0.map[(address - 0x9800) as usize],
            0x9C00..=0x9FFF => self.tilemap1.map[(address - 0x9C00) as usize],
            _ => unreachable!(),
        }
    }
    fn write(&mut self, address: u16, value: u8) {
        match address {
            0x8000..=0x87FF => self.tiledata.block0[(address - 0x8000) as usize] = value,
            0x8800..=0x8FFF => self.tiledata.block1[(address - 0x8800) as usize] = value,
            0x9000..=0x97FF => self.tiledata.block2[(address - 0x9000) as usize] = value,
            0x9800..=0x9BFF => self.tilemap0.map[(address - 0x9800) as usize] = value,
            0x9C00..=0x9FFF => self.tilemap1.map[(address - 0x9C00) as usize] = value,
            _ => unreachable!(),
        }
    }
}
memory_region!(SRam, 0x2000, 0xA000);
memory_region!(WRam0, 0x1000, 0xC000);
memory_region!(WRamX, 0x1000, 0xD000);
memory_region!(Echo, 0x1E00, 0xE000);

#[derive(Copy, Clone, PartialEq)]
pub struct Oam {
    memory: [u8; 0x00A0],
}
impl Oam {
    pub fn new() -> Self {
        Self { memory: [0; 0x0A0] }
    }
}
impl Memory for Oam {
    fn read(&self, address: u16) -> u8 {
        self.memory[(address - 0xFE00) as usize]
    }
    fn write(&mut self, address: u16, value: u8) {
        self.memory[(address - 0xFE00) as usize] = value;
    }
}

memory_region!(UnusedMemory, 0x0060, 0xFEA0);
#[allow(clippy::upper_case_acronyms)]
#[derive(Copy, Clone, PartialEq)]
pub struct LCDC {
    lcdcontrol: u8,
}
#[derive(Copy, Clone, PartialEq)]
#[allow(clippy::upper_case_acronyms)]
pub struct SCY {
    pub scroll_y: u8,
}
#[derive(Copy, Clone, PartialEq)]
#[allow(clippy::upper_case_acronyms)]
pub struct SCX {
    pub scroll_x: u8,
}
#[derive(Copy, Clone, PartialEq)]
pub struct IORegisters {
    memory: [u8; 0x0080],
    pub lcdcontrol: LCDC,
    pub scy: SCY,
    pub scx: SCX,
}

impl IORegisters {
    pub fn new() -> Self {
        Self {
            memory: [0; 0x0080],
            lcdcontrol: LCDC { lcdcontrol: 0 },
            scy: SCY { scroll_y: 0 },
            scx: SCX { scroll_x: 0 },
        }
    }
}

impl Default for IORegisters {
    fn default() -> Self {
        Self::new()
    }
}
impl Memory for IORegisters {
    fn read(&self, address: u16) -> u8 {
        match address {
            0xFF40 => self.lcdcontrol.lcdcontrol,
            0xFF00..=0xFF3F | 0xFF41..=0xFF80 => self.memory[(address - 0xFF00) as usize],
            _ => unreachable!(),
        }
    }
    fn write(&mut self, address: u16, value: u8) {
        match address {
            0xFF40 => self.lcdcontrol.lcdcontrol = value,
            0xFF00..=0xFF3F | 0xFF41..=0xFF80 => self.memory[(address - 0xFF00) as usize] = value,
            _ => {
                unreachable!()
            }
        };
    }
}
memory_region!(HRam, 0x007F, 0xFF80);
memory_region!(IERegister, 0x0001, 0xFFFF);

#[derive(Copy, Clone, PartialEq)]
pub struct MemoryMap {
    rom0: Rom0,
    romx: RomX,
    pub vram: VRam,
    sram: SRam,
    wram0: WRam0,
    wramx: WRamX,
    echo: Echo,
    aom: Oam,
    unused: UnusedMemory,
    pub io_registers: IORegisters,
    hram: HRam,
    ie_register: IERegister,
}

impl Memory for MemoryMap {
    fn read(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x3FFF => self.rom0.read(address),
            0x4000..=0x7FFF => self.romx.read(address),
            0x8000..=0x9FFF => self.vram.read(address),
            0xA000..=0xBFFF => self.sram.read(address),
            0xC000..=0xCFFF => self.wram0.read(address),
            0xD000..=0xDFFF => self.wramx.read(address),
            0xE000..=0xFDFF => self.echo.read(address),
            0xFE00..=0xFE9F => self.aom.read(address),
            0xFEA0..=0xFEFF => self.unused.read(address),
            0xFF00..=0xFF7F => self.io_registers.read(address),
            0xFF80..=0xFFFE => self.hram.read(address),
            0xFFFF => self.ie_register.read(address),
        }
    }
    fn write(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x3FFF => self.rom0.write(address, value),
            0x4000..=0x7FFF => self.romx.write(address, value),
            0x8000..=0x9FFF => self.vram.write(address, value),
            0xA000..=0xBFFF => self.sram.write(address, value),
            0xC000..=0xCFFF => self.wram0.write(address, value),
            0xD000..=0xDFFF => self.wramx.write(address, value),
            0xE000..=0xFDFF => self.echo.write(address, value),
            0xFE00..=0xFE9F => self.aom.write(address, value),
            0xFEA0..=0xFEFF => self.unused.write(address, value),
            0xFF00..=0xFF7F => self.io_registers.write(address, value),
            0xFF80..=0xFFFE => self.hram.write(address, value),
            0xFFFF => self.ie_register.write(address, value),
        }
    }
}

#[allow(dead_code)]
impl MemoryMap {
    pub fn new() -> Self {
        Self {
            rom0: Rom0::new(),
            romx: RomX::new(),
            vram: VRam::new(),
            sram: SRam::new(),
            wram0: WRam0::new(),
            wramx: WRamX::new(),
            echo: Echo::new(),
            aom: Oam::new(),
            unused: UnusedMemory::new(),
            io_registers: IORegisters::new(),
            hram: HRam::new(),
            ie_register: IERegister::new(),
        }
    }
    pub fn load_tiles(self) -> Vec<Tile> {
        let mut i = 0_usize;
        let mut tiles: Vec<Tile> = Vec::new();
        while i <= 255 {
            let tile = self
                .vram
                .tiledata
                .get_tile(self.io_registers.lcdcontrol, i as u8);
            if !(tile == Tile::new([0; 16])) {
                tiles.push(tile)
            }
            i += 1;
        }
        tiles
    }
}

impl Default for MemoryMap {
    fn default() -> Self {
        Self::new()
    }
}
