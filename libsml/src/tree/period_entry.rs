use std::marker::PhantomData;

/// See <https://github.com/volkszaehler/libsml/blob/master/sml/include/sml/sml_tree.h>
pub struct PeriodEntry<'a>(pub(crate) *mut libsml_sys::sml_period_entry, PhantomData<&'a ()>);

impl<'a> PeriodEntry<'a> {
	pub fn new() -> Self {
		PeriodEntry(unsafe { libsml_sys::sml_period_entry_init() }, PhantomData)
	}
}

impl<'a> Drop for PeriodEntry<'a> {
	fn drop(&mut self) {
		unsafe { libsml_sys::sml_period_entry_free(self.0) }
	}
}