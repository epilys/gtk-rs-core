// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{ActionGroup, ActionMap};
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GSimpleActionGroup")]
    pub struct SimpleActionGroup(Object<ffi::GSimpleActionGroup, ffi::GSimpleActionGroupClass>) @implements ActionGroup, ActionMap;

    match fn {
        type_ => || ffi::g_simple_action_group_get_type(),
    }
}

impl SimpleActionGroup {
    pub const NONE: Option<&'static SimpleActionGroup> = None;

    #[doc(alias = "g_simple_action_group_new")]
    pub fn new() -> SimpleActionGroup {
        unsafe { from_glib_full(ffi::g_simple_action_group_new()) }
    }
}

impl Default for SimpleActionGroup {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for SimpleActionGroup {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("SimpleActionGroup")
    }
}
