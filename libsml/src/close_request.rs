/// See <https://github.com/volkszaehler/libsml/blob/master/sml/include/sml/sml_close_request.h>
pub struct CloseRequest(pub(crate) *mut libsml_sys::sml_close_request);

impl CloseRequest {
	pub fn new() -> Self {
		Self( unsafe { libsml_sys::sml_close_request_init() } )
	}

	pub fn parse(buf: crate::Buffer) -> Self {
		unsafe { Self(libsml_sys::sml_close_request_parse(buf.0)) }
	}

	pub fn write(&self, buf: crate::Buffer) {
		unsafe { libsml_sys::sml_close_request_write(self.0, buf.0) };
	}
}

impl Drop for CloseRequest {
	fn drop(&mut self) {
		unsafe { libsml_sys::sml_close_request_free(self.0) }
	}
}