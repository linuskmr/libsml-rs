//! See <https://github.com/volkszaehler/libsml/blob/master/sml/include/sml/sml_file.h>

use std::marker::PhantomData;

use crate::Message;

/// A SML file consist of multiple [SML messages](crate::Message).
///
/// See <https://github.com/volkszaehler/libsml/blob/master/sml/include/sml/sml_file.h>
pub struct File<'a>(
    pub(crate) *mut libsml_sys::sml_file,
    pub(crate) PhantomData<&'a ()>,
);

impl<'a> File<'a> {
    pub fn new() -> File<'a> {
        File(unsafe { libsml_sys::sml_file_init() }, PhantomData)
    }

    pub fn add_message(&mut self, message: Message<'a>) {
        unsafe { libsml_sys::sml_file_add_message(self.0, message.0) }
    }
}

impl<'a> Drop for File<'a> {
    fn drop(&mut self) {
        unsafe { libsml_sys::sml_file_free(self.0) }
    }
}
