//! See <https://github.com/volkszaehler/libsml/blob/master/sml/include/sml/sml_value.h>

/// See <https://github.com/volkszaehler/libsml/blob/master/sml/include/sml/sml_value.h>
pub struct Value(pub(crate) *mut libsml_sys::sml_value);

impl Value {
	pub fn new() -> Self {
		Value(unsafe { libsml_sys::sml_value_init() })
	}
}

impl Drop for Value {
	fn drop(&mut self) {
		unsafe { libsml_sys::sml_value_free(self.0) }
	}
}

impl From<Value> for f64 {
	fn from(value: Value) -> Self {
		unsafe { libsml_sys::sml_value_to_double(value.0) }
	}
}