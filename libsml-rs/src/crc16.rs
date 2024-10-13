pub type Crc16 = u16;

/// CRC16 FSC implementation based on DIN 62056-46
pub fn calculate(data: &mut [u8]) -> Crc16 {
	unsafe { libsml_sys::sml_crc16_calculate(data.as_mut_ptr(), data.len() as i32) }
}

/// CRC-16/CCITT(Kermit) implementation poly=0x1021 init=0x0000 refin=true refout=true xorout=0x0000
pub fn calculate_kerkmit(data: &mut [u8]) -> Crc16 {
	unsafe { libsml_sys::sml_crc16kermit_calculate(data.as_mut_ptr(), data.len() as i32) }
}