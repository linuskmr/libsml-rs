/// See <https://github.com/volkszaehler/libsml/blob/master/sml/include/sml/sml_boolean.h>
pub struct Boolean(pub(crate) *mut libsml_sys::sml_boolean);

impl Boolean {
	pub fn new(value: bool) -> Self {
		Self( unsafe { libsml_sys::sml_boolean_init(value as u8) } )
	}

	pub fn get(&self) -> bool {
		let value: u8 = unsafe { *self.0 };
		match value as u32 {
			libsml_sys::SML_BOOLEAN_TRUE => true,
			libsml_sys::SML_BOOLEAN_FALSE => false,
			_ => panic!("libsml: boolean: invalid value: {}", value),
		}
	}

	pub fn set(&mut self, value: bool) {
		let value = match value {
			true => libsml_sys::SML_BOOLEAN_TRUE,
			false => libsml_sys::SML_BOOLEAN_FALSE,
		};
		unsafe { *self.0 = value as u8 };
	}

	pub fn parse(buf: crate::Buffer) -> Self {
		Boolean(unsafe { libsml_sys::sml_boolean_parse(buf.0) })
	}
}

impl Drop for Boolean {
	fn drop(&mut self) {
		unsafe { libsml_sys::sml_boolean_free(self.0) }
	}
}