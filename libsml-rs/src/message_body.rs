//! See <https://github.com/volkszaehler/libsml/blob/master/sml/include/sml/sml_message.h>

use std::marker::PhantomData;

use crate::Buffer;

/// See <https://github.com/volkszaehler/libsml/blob/master/sml/include/sml/sml_message.h>
pub struct MessageBody<'a>(pub(crate) *mut libsml_sys::sml_message_body, pub(crate) PhantomData<&'a ()>);

impl<'a> MessageBody<'a> {
    pub fn new(tag: u32, data: &'a mut [u8]) -> Self {
        MessageBody(unsafe { libsml_sys::sml_message_body_init(tag, data.as_mut_ptr() as *mut std::ffi::c_void) }, PhantomData)
    }
}

impl<'a> From<Buffer<'a>> for MessageBody<'a> {
    fn from(buffer: Buffer<'a>) -> Self {
        MessageBody(unsafe { libsml_sys::sml_message_body_parse(buffer.0) }, PhantomData)
    }
}

impl<'a> Drop for MessageBody<'a> {
	fn drop(&mut self) {
		unsafe { libsml_sys::sml_message_body_free(self.0) }
	}
}