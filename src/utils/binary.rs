pub struct Binary;

impl Binary {
    const BIG_ENDIAN: u8 = 0x00;
    const LITTLE_ENDIAN: u8 = 0x01;

    pub fn read_triad(data: &[u8]) -> u32 {
        ((data[0] as u32) << 16) | ((data[1] as u32) << 8) | (data[2] as u32)
    }

    pub fn write_triad(value: u32) -> [u8; 3] {
        [
            ((value >> 16) & 0xFF) as u8,
            ((value >> 8) & 0xFF) as u8,
            (value & 0xFF) as u8,
        ]
    }

    pub fn read_ltriad(data: &[u8]) -> u32 {
        assert!(data.len() >= 3);
        (data[0] as u32) | ((data[1] as u32) << 8) | ((data[2] as u32) << 16)
    }

    pub fn write_ltriad(value: u32) -> [u8; 3] {
        [
            (value & 0xFF) as u8,
            ((value >> 8) & 0xFF) as u8,
            ((value >> 16) & 0xFF) as u8,
        ]
    }

    pub fn read_metadata() {}

    pub fn read_bool(&self, byte: u8) -> bool {
        Self::read_byte(byte, false) != 0
    }

    pub fn write_bool(b: bool) -> String {
        Self::write_byte(if b { 1 } else { 0 })
    }

    pub fn read_byte(byte: u8, signed: bool) -> i32 {
        if signed {
            let shifted = (byte as i32) << 24 >> 24;
            shifted
        } else {
            byte as i32
        }
    }

    pub fn write_byte(c: u8) -> String {
        (c as char).to_string()
    }
}
