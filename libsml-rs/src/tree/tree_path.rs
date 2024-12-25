use crate::OctetString;
use std::marker::PhantomData;

/// See <https://github.com/volkszaehler/libsml/blob/master/sml/include/sml/sml_tree.h>
pub struct TreePath<'a>(
    pub(crate) *mut libsml_sys::sml_tree_path,
    PhantomData<&'a ()>,
);

impl<'a> TreePath<'a> {
    pub fn new() -> Self {
        TreePath(unsafe { libsml_sys::sml_tree_path_init() }, PhantomData)
    }

    pub fn path_entries(&self) -> Vec<OctetString<'a>> {
        let path_entries_len = unsafe { self.0.as_ref().unwrap().path_entries_len } as usize;
        let mut path_entries = Vec::with_capacity(path_entries_len);
        for i in 0..path_entries_len {
            let path_entry = unsafe { *(*self.0).path_entries.add(i) };
            path_entries.push(OctetString(path_entry, PhantomData));
        }
        path_entries
    }
}

impl<'a> Drop for TreePath<'a> {
    fn drop(&mut self) {
        unsafe { libsml_sys::sml_tree_path_free(self.0) }
    }
}
