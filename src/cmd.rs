pub struct Cmd;
impl Cmd {
    // Init
    pub const SW_RESET: u8 = 0x12;
    pub const DRIVER_CONTROL: u8 = 0x01;
    pub const SET_RAMXPOS: u8 = 0x44;
    pub const SET_RAMYPOS: u8 = 0x45;
    pub const BORDER_WAVEFORM_CONTROL: u8 = 0x3C;
    pub const TEMP_CONTROL: u8 = 0x18;

    // Update
    pub const SET_RAMX_COUNTER: u8 = 0x4E;
    pub const SET_RAMY_COUNTER: u8 = 0x4F;
    pub const WRITE_BW_DATA: u8 = 0x24;
    pub const WRITE_RED_DATA: u8 = 0x26;
    pub const UPDATE_DISPLAY_CTRL2: u8 = 0x22;
    pub const MASTER_ACTIVATE: u8 = 0x20;
}

pub struct Flag;
impl Flag {
    pub const INTERNAL_TEMP_SENSOR: u8 = 0x80;
    pub const BORDER_WAVEFORM_FOLLOW_LUT: u8 = 0b0100;
    pub const BORDER_WAVEFORM_LUT1: u8 = 0b0001;
    pub const DISPLAY_UPDATE_SEQUENCE: u8 = 0xF7;
}
