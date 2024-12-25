//! See <https://github.com/volkszaehler/libsml/blob/master/sml/include/sml/sml_message.h>

use crate::{MessageBody, OctetString};
use std::marker::PhantomData;

/// See <https://github.com/volkszaehler/libsml/blob/master/sml/include/sml/sml_message.h>
pub struct Message<'a>(
    pub(crate) *mut libsml_sys::sml_message,
    pub(crate) PhantomData<&'a ()>,
);

impl<'a> Message<'a> {
    pub fn new() -> Message<'a> {
        Message(unsafe { libsml_sys::sml_message_init() }, PhantomData)
    }

    pub fn transaction_id(&self) -> OctetString<'a> {
        unsafe { OctetString(self.0.as_ref().unwrap().transaction_id, PhantomData) }
    }

    pub fn set_transaction_id(&mut self, transaction_id: &mut OctetString<'a>) {
        unsafe { self.0.as_mut().unwrap().transaction_id = transaction_id.0 }
    }

    pub fn group_id(&self) -> &'a u8 {
        unsafe { self.0.as_ref().unwrap().group_id.as_ref().unwrap() }
    }

    pub fn set_group_id(&mut self, group_id: &'a mut u8) {
        unsafe { self.0.as_mut().unwrap().group_id = group_id }
    }

    pub fn abort_on_error(&self) -> &'a u8 {
        unsafe { self.0.as_ref().unwrap().abort_on_error.as_ref().unwrap() }
    }

    pub fn set_abort_on_error(&mut self, abort_on_error: &'a mut u8) {
        unsafe { self.0.as_mut().unwrap().abort_on_error = abort_on_error }
    }

    pub fn crc(&self) -> &'a [u16; 3] {
        // unsafe { crc16::calculate(); }
        todo!()
    }

    pub fn set_crc(&mut self, crc: &'a mut [u16; 3]) {
        unsafe { self.0.as_mut().unwrap().crc = crc.as_mut_ptr() }
    }

    pub fn message_body(&self) -> MessageBody<'a> {
        unsafe { MessageBody(self.0.as_ref().unwrap().message_body, PhantomData) }
    }

    pub fn set_message_body(&mut self, message_body: MessageBody<'a>) {
        unsafe { self.0.as_mut().unwrap().message_body = message_body.0 }
    }
}

impl<'a> Drop for Message<'a> {
    fn drop(&mut self) {
        unsafe { libsml_sys::sml_message_free(self.0) }
    }
}
