//! See <https://github.com/volkszaehler/libsml/blob/master/sml/include/sml/sml_octet_string.h>

use std::marker::PhantomData;

/// See <https://github.com/volkszaehler/libsml/blob/master/sml/include/sml/sml_octet_string.h>
pub struct OctetString<'a>(pub(crate) *mut libsml_sys::octet_string, pub(crate) PhantomData<&'a ()>);

impl<'a> Drop for OctetString<'a> {
	fn drop(&mut self) {
		unsafe { libsml_sys::sml_octet_string_free(self.0) }
	}
}