//! See <https://github.com/volkszaehler/libsml/blob/master/sml/include/sml/sml_shared.h>

use std::marker::PhantomData;

/// See <https://github.com/volkszaehler/libsml/blob/master/sml/include/sml/sml_shared.h>
pub struct Buffer<'a>(
    pub(crate) *mut libsml_sys::sml_buffer,
    pub(crate) PhantomData<&'a ()>,
);

impl<'a> Buffer<'a> {
    pub fn new(length: usize) -> Self {
        Buffer(unsafe { libsml_sys::sml_buffer_init(length) }, PhantomData)
    }

    /// Returns the length of the following data structure. Sets the cursor position to the value field.
    pub fn get_next_length(&self) -> i32 {
        unsafe { libsml_sys::sml_buf_get_next_length(self.0) }
    }

    pub fn set_type_and_length(buf: crate::Buffer, type_: u32, len: u32) {
        unsafe { libsml_sys::sml_buf_set_type_and_length(buf.0, type_, len) }
    }

    /// Checks if a error is occurred.
    pub fn has_errors(&self) -> i32 {
        unsafe { libsml_sys::sml_buf_has_errors(self.0) }
    }

    pub fn get_next_type(&self) -> i32 {
        unsafe { libsml_sys::sml_buf_get_next_type(self.0) }
    }

    /// Returns the current byte.
    pub fn get_current_byte(&self) -> u8 {
        unsafe { libsml_sys::sml_buf_get_current_byte(self.0) }
    }

    /// Returns a pointer to the current buffer position.
    pub fn get_current_buf(&self) -> *const u8 {
        unsafe { libsml_sys::sml_buf_get_current_buf(self.0) }
    }

    pub fn optional_write(&self) {
        unsafe { libsml_sys::sml_buf_optional_write(self.0) }
    }

    /// Sets the number of bytes read (moves the cursor forward)
    pub fn update_bytes_read(&self, bytes_read: i32) {
        unsafe { libsml_sys::sml_buf_update_bytes_read(self.0, bytes_read) }
    }

    /// Checks if the next field is a skipped optional field, updates the buffer accordingly
    pub fn optional_is_skipped(buf: crate::Buffer) -> bool {
        unsafe {
            libsml_sys::sml_buf_optional_is_skipped(buf.0) as u32
                == libsml_sys::SML_OPTIONAL_SKIPPED
        }
    }
}

impl<'a> Drop for Buffer<'a> {
    fn drop(&mut self) {
        unsafe { libsml_sys::sml_buffer_free(self.0) }
    }
}
