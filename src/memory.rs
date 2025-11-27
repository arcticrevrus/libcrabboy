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

#[derive(Default, Copy, Clone, PartialEq)]
pub struct JoyPadIO {
    pub buttons: u8,
}
#[derive(Default, Copy, Clone, PartialEq)]
pub struct SerialIO {
    pub serial_data: u8,
    pub transfer_control: u8,
}
#[derive(Default, Copy, Clone, PartialEq)]
pub struct TimerAndDivider {
    pub divider_register: u8,
    pub timer_counter: u8,
    pub timer_modulo: u8,
    pub timer_control: u8,
}
#[derive(Default, Copy, Clone, PartialEq)]
pub struct InterruptFlags {
    pub interrupt_flag: u8,
}
#[derive(Default, Copy, Clone, PartialEq)]
pub struct AudioRegisters {
    pub master_control: u8,
    pub sound_panning: u8,
    pub master_volume_and_vin: u8,
    pub channel_1_sweep: u8,
    pub channel_1_length_and_duty_cycle: u8,
    pub channel_1_volume_and_envelope: u8,
    pub channel_1_period_low: u8,
    pub channel_1_period_high_and_control: u8,
    pub channel_2_length_and_duty_cycle: u8,
    pub channel_2_volume_and_envelope: u8,
    pub channel_2_period_low: u8,
    pub channel_2_period_high_and_control: u8,
    pub channel_3_dac_enable: u8,
    pub channel_3_length_timer: u8,
    pub channel_3_output_level: u8,
    pub channel_3_period_low: u8,
    pub channel_3_period_high_and_control: u8,
    pub channel_4_length_timer: u8,
    pub channel_4_volume_and_envelope: u8,
    pub channel_4_frequency_and_randomness: u8,
    pub channel_4_control: u8,
    pub wave_pattern_ram: [u8; 16],
}
#[allow(clippy::upper_case_acronyms)]
#[derive(Copy, Clone, PartialEq)]
pub struct LCDC {
    lcdcontrol: u8,
}
#[derive(Default, Copy, Clone, PartialEq)]
#[allow(clippy::upper_case_acronyms)]
pub struct SCY {
    pub scroll_y: u8,
}
#[derive(Default, Copy, Clone, PartialEq)]
#[allow(clippy::upper_case_acronyms)]
pub struct SCX {
    pub scroll_x: u8,
}

#[derive(Default, Copy, Clone, PartialEq)]
#[allow(clippy::upper_case_acronyms)]
pub struct DMA {
    pub oam_dma: u8,
}
#[derive(Copy, Clone, PartialEq)]
pub struct IORegisters {
    memory: [u8; 0x0080],
    pub joypad: JoyPadIO,
    pub serial: SerialIO,
    pub timer_and_divider: TimerAndDivider,
    pub interrupt_flags: InterruptFlags,
    pub audio_registers: AudioRegisters,
    pub lcdcontrol: LCDC,
    pub scy: SCY,
    pub scx: SCX,
    pub dma: DMA,
}

impl IORegisters {
    pub fn new() -> Self {
        Self {
            memory: [0; 0x0080],
            joypad: JoyPadIO::default(),
            serial: SerialIO::default(),
            timer_and_divider: TimerAndDivider::default(),
            interrupt_flags: InterruptFlags::default(),
            audio_registers: AudioRegisters::default(),
            lcdcontrol: LCDC { lcdcontrol: 0 },
            scy: SCY { scroll_y: 0 },
            scx: SCX { scroll_x: 0 },
            dma: DMA::default(),
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
        #[allow(clippy::match_overlapping_arm)]
        match address {
            0xFF00 => self.joypad.buttons,
            0xFF01 => self.serial.serial_data,
            0xFF02 => self.serial.transfer_control,
            0xFF04 => self.timer_and_divider.divider_register,
            0xFF05 => self.timer_and_divider.timer_counter,
            0xFF06 => self.timer_and_divider.timer_modulo,
            0xFF07 => self.timer_and_divider.timer_control,
            0xFF0F => self.interrupt_flags.interrupt_flag,
            0xFF10 => self.audio_registers.channel_1_sweep,
            0xFF11 => self.audio_registers.channel_1_length_and_duty_cycle,
            0xFF12 => self.audio_registers.channel_1_volume_and_envelope,
            0xFF13 => self.audio_registers.channel_1_period_low,
            0xFF14 => self.audio_registers.channel_1_period_high_and_control,
            0xFF16 => self.audio_registers.channel_2_length_and_duty_cycle,
            0xFF17 => self.audio_registers.channel_2_volume_and_envelope,
            0xFF18 => self.audio_registers.channel_2_period_low,
            0xFF19 => self.audio_registers.channel_2_period_high_and_control,
            0xFF1A => self.audio_registers.channel_3_dac_enable,
            0xFF1B => self.audio_registers.channel_3_length_timer,
            0xFF1C => self.audio_registers.channel_3_output_level,
            0xFF1D => self.audio_registers.channel_3_period_low,
            0xFF1E => self.audio_registers.channel_3_period_high_and_control,
            0xFF20 => self.audio_registers.channel_4_length_timer,
            0xFF21 => self.audio_registers.channel_4_volume_and_envelope,
            0xFF22 => self.audio_registers.channel_4_frequency_and_randomness,
            0xFF23 => self.audio_registers.channel_4_control,
            0xFF24 => self.audio_registers.master_volume_and_vin,
            0xFF25 => self.audio_registers.sound_panning,
            0xFF26 => self.audio_registers.master_control,
            0xFF30..=0xFF3F => self.audio_registers.wave_pattern_ram[(address - 0xFF30) as usize],
            0xFF40 => self.lcdcontrol.lcdcontrol,
            0xFF46 => self.dma.oam_dma,
            0xFF0..=0xFF80 => self.memory[(address - 0xFF00) as usize],
            _ => unreachable!(),
        }
    }
    fn write(&mut self, address: u16, value: u8) {
        #[allow(clippy::match_overlapping_arm)]
        let dest = match address {
            0xFF00 => &mut self.joypad.buttons,
            0xFF01 => &mut self.serial.serial_data,
            0xFF02 => &mut self.serial.transfer_control,
            0xFF04 => &mut self.timer_and_divider.divider_register,
            0xFF05 => &mut self.timer_and_divider.timer_counter,
            0xFF06 => &mut self.timer_and_divider.timer_modulo,
            0xFF07 => &mut self.timer_and_divider.timer_control,
            0xFF0F => &mut self.interrupt_flags.interrupt_flag,
            0xFF10 => &mut self.audio_registers.channel_1_sweep,
            0xFF11 => &mut self.audio_registers.channel_1_length_and_duty_cycle,
            0xFF12 => &mut self.audio_registers.channel_1_volume_and_envelope,
            0xFF13 => &mut self.audio_registers.channel_1_period_low,
            0xFF14 => &mut self.audio_registers.channel_1_period_high_and_control,
            0xFF16 => &mut self.audio_registers.channel_2_length_and_duty_cycle,
            0xFF17 => &mut self.audio_registers.channel_2_volume_and_envelope,
            0xFF18 => &mut self.audio_registers.channel_2_period_low,
            0xFF19 => &mut self.audio_registers.channel_2_period_high_and_control,
            0xFF1A => &mut self.audio_registers.channel_3_dac_enable,
            0xFF1C => &mut self.audio_registers.channel_3_output_level,
            0xFF1D => &mut self.audio_registers.channel_3_period_low,
            0xFF1E => &mut self.audio_registers.channel_3_period_high_and_control,
            0xFF20 => &mut self.audio_registers.channel_4_length_timer,
            0xFF21 => &mut self.audio_registers.channel_4_volume_and_envelope,
            0xFF22 => &mut self.audio_registers.channel_4_frequency_and_randomness,
            0xFF23 => &mut self.audio_registers.channel_4_control,
            0xFF24 => &mut self.audio_registers.master_volume_and_vin,
            0xFF25 => &mut self.audio_registers.sound_panning,
            0xFF26 => &mut self.audio_registers.master_control,
            0xFF30..=0xFF3F => {
                &mut self.audio_registers.wave_pattern_ram[(address - 0xFF30) as usize]
            }
            0xFF40 => &mut self.lcdcontrol.lcdcontrol,
            0xFF46 => &mut self.dma.oam_dma,
            0xFF00..=0xFF80 => &mut self.memory[(address - 0xFF00) as usize],
            _ => {
                unreachable!()
            }
        };
        *dest = value;
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
