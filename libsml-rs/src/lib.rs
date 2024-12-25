//! High-level Rust bindings to [libsml](https://github.com/volkszaehler/libsml).

mod attention_response;
mod boolean;
mod buffer;
mod close_request;
pub mod crc16;
mod file;
mod get_list_response;
mod message;
mod message_body;
mod octet_string;
mod time;
pub mod transport;
pub mod tree;
mod value;

pub use attention_response::AttentionResponse;
pub use boolean::Boolean;
pub use buffer::Buffer;
pub use close_request::CloseRequest;
pub use file::File;
pub use get_list_response::GetListResponse;
pub use message::Message;
pub use message_body::MessageBody;
pub use octet_string::OctetString;
pub use time::{Time, TimeData};
pub use tree::TreePath;
pub use value::Value;

/// Prints arbitrarily byte string to stdout with printf
///
/// See <https://github.com/volkszaehler/libsml/blob/master/sml/include/sml/sml_shared.h>
pub fn hexdump(buf: &[char]) {
    unsafe { libsml_sys::sml_hexdump(buf.as_ptr() as *mut u8, buf.len()) }
}
