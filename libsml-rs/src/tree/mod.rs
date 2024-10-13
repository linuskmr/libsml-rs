//! See <https://github.com/volkszaehler/libsml/blob/master/sml/include/sml/sml_tree.h>


mod tree_path;
mod proc_par_value;

pub use tree_path::TreePath;
pub use proc_par_value::ProcParValue;

use std::marker::PhantomData;

use crate::OctetString;

/// See <https://github.com/volkszaehler/libsml/blob/master/sml/include/sml/sml_tree.h>
pub struct Tree<'a>(pub(crate) *mut libsml_sys::sml_tree, pub(crate) PhantomData<&'a ()>);

impl<'a> Tree<'a> {
	pub fn new() -> Self {
		Tree(unsafe { libsml_sys::sml_tree_init() }, PhantomData)
	}

	pub fn parameter_name(&self) -> OctetString {
		OctetString(unsafe { (*self.0).parameter_name }, PhantomData)
	}

	pub fn set_parameter_name(&mut self, parameter_name: &'a OctetString) {
		unsafe { (*self.0).parameter_name = parameter_name.0 }
	}

	pub fn parameter_value(&mut self) -> Option<ProcParValue> {
		let parameter_value = unsafe { (*self.0).parameter_value };
		if parameter_value.is_null() {
			None
		} else {
			Some(ProcParValue(parameter_value, PhantomData))
		}
	}

	pub fn set_parameter_value(&mut self, parameter_value: Option<&'a ProcParValue>) {
		unsafe { (*self.0).parameter_value = parameter_value.map_or(std::ptr::null_mut(), |v| v.0) }
	}

	pub fn child_list(&self) -> Vec<Tree<'a>> {
		let child_list_len = unsafe { (*self.0).child_list_len } as usize;
		let mut child_list = Vec::with_capacity(child_list_len);
		for i in 0..child_list_len {
			let child = unsafe { *(*self.0).child_list.add(i) };
			child_list.push(Tree(child, PhantomData));
		}
		child_list
	}

	pub fn set_child_list(&mut self, child_list: Vec<Tree<'a>>) {
		unsafe {
			(*self.0).child_list_len = child_list.len() as i32;
			(*self.0).child_list = child_list.as_ptr() as *mut _;
		}
	}
}



