/// See <https://github.com/volkszaehler/libsml/blob/master/sml/include/sml/sml_close_response.h>
pub struct CloseResponse(pub(crate) *mut libsml_sys::sml_close_response);

impl CloseResponse {
	pub fn new() -> Self {
		Self( unsafe { libsml_sys::sml_close_response_init() } )
	}

	pub fn parse(buf: crate::Buffer) -> Self {
		unsafe { Self(libsml_sys::sml_close_response_parse(buf.0)) }
	}

	pub fn write(&self, buf: crate::Buffer) {
		unsafe { libsml_sys::sml_close_response_write(self.0, buf.0) };
	}
}

impl Drop for CloseResponse {
	fn drop(&mut self) {
		unsafe { libsml_sys::sml_close_response_free(self.0) }
	}
}