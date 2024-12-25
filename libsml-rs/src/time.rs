//! See <https://github.com/volkszaehler/libsml/blob/master/sml/include/sml/sml_time.h>

use std::marker::PhantomData;

/// See <https://github.com/volkszaehler/libsml/blob/master/sml/include/sml/sml_time.h>
pub struct Time<'a>(
    pub(crate) *mut libsml_sys::sml_time,
    pub(crate) PhantomData<&'a ()>,
);

pub enum TimeData {
    SecIndex(u32),
    Timestamp(u32),
}

impl<'a> Time<'a> {
    pub fn new() -> Self {
        Time(unsafe { libsml_sys::sml_time_init() }, PhantomData)
    }

    pub fn get(&self) -> TimeData {
        let tag: u8 = unsafe { *(*self.0).tag };
        match tag {
            0 => {
                let sec_index: u32 = unsafe { *(*self.0).data.sec_index };
                TimeData::SecIndex(sec_index)
            }
            1 => {
                let timestamp: u32 = unsafe { *(*self.0).data.timestamp };
                TimeData::Timestamp(timestamp)
            }
            _ => panic!("Invalid time tag"),
        }
    }
}
