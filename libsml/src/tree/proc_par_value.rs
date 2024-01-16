use std::marker::PhantomData;

/// See <https://github.com/volkszaehler/libsml/blob/master/sml/include/sml/sml_tree.h>
pub struct ProcParValue<'a>(pub(crate) *mut libsml_sys::sml_proc_par_value, pub(crate) PhantomData<&'a ()>);

impl<'a> ProcParValue<'a> {
	pub fn new() -> Self {
		ProcParValue(unsafe { libsml_sys::sml_proc_par_value_init() }, PhantomData)
	}
}

impl<'a> Drop for ProcParValue<'a> {
	fn drop(&mut self) {
		unsafe { libsml_sys::sml_proc_par_value_free(self.0) }
	}
}