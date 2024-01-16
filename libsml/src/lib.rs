//! High-level Rust bindings to [libsml](https://github.com/volkszaehler/libsml).

mod buffer;
mod file;
mod message_body;
mod message;
mod octet_string;
pub mod transport;
mod value;
pub mod tree;
mod boolean;
mod get_list_response;
mod time;
mod attention_response;
mod close_request;


pub use buffer::Buffer;
pub use file::File;
pub use message_body::MessageBody;
pub use message::Message;
pub use octet_string::OctetString;
pub use value::Value;
pub use tree::TreePath;
pub use boolean::Boolean;
pub use get_list_response::GetListResponse;
pub use time::{Time, TimeData};
pub use attention_response::AttentionResponse;
pub use close_request::CloseRequest;


/// Prints arbitrarily byte string to stdout with printf
/// 
/// See <https://github.com/volkszaehler/libsml/blob/master/sml/include/sml/sml_shared.h>
pub fn hexdump(buf: &[char]) {
	unsafe { libsml_sys::sml_hexdump(buf.as_ptr() as *mut u8, buf.len()) }
}