//! See <https://github.com/volkszaehler/libsml/blob/master/sml/include/sml/sml_transport.h>

use std::{os::{fd::AsRawFd, self}, io};

use crate::File;

/// Reads a [SML file](File) from `fd` into a buffer.
/// 
/// Continuously read bytes from `fd` and scans for the SML transport protocol escape sequences. If a SML file is detected it will be copied into the `buf` and the number of bytes read will be returned. If the SML file exceeds the len of `buf`, `io::Error::other("buf to small")` will be returned.
pub fn read(fd: impl AsRawFd, buf: &mut [u8]) -> Result<usize, io::Error> {
	let bytes_read = unsafe { libsml_sys::sml_transport_read(fd.as_raw_fd(), buf.as_mut_ptr(), buf.len()) };
	if bytes_read as isize == -1 {
		Err(io::Error::other("buf to small"))
	} else if bytes_read == 0 {
		Err(io::Error::last_os_error())
	} else {
		Ok(bytes_read)
	}
}

/// Continuously reads via [`read()`] in an endless loop and calls the `receiver`.
pub fn listen(fd: impl AsRawFd, receiver: fn(&[u8])) {
	let receiver = move |buf: *const u8, len: usize| {
		let buf = unsafe { std::slice::from_raw_parts(buf, len) };
		receiver(buf);
	};
	unsafe {
		let receiver = &receiver as *const dyn Fn(*const u8, usize) as *const unsafe extern fn(*mut u8, usize);
		libsml_sys::sml_transport_listen(fd.as_raw_fd(), Some(*receiver));
	}
}

/// Writes a [SML file](File) to `fd`.
/// 
/// Adds the SML transport protocol escape sequences and writes the given `file` to `fd`. The file must be in the parsed format. The number of bytes written is returned.
pub fn write(fd: impl AsRawFd, file: &File) -> Result<usize, io::Error> {
	let bytes_written = unsafe { libsml_sys::sml_transport_write(fd.as_raw_fd(), file.0) };
	if bytes_written == 0 {
		Err(io::Error::last_os_error())
	} else {
		Ok(bytes_written as usize)
	}
}