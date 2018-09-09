// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Buildable;
use Container;
use Widget;
use ffi;
use glib::object::Downcast;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct Fixed(Object<ffi::GtkFixed, ffi::GtkFixedClass>): Container, Widget, Buildable;

    match fn {
        get_type => || ffi::gtk_fixed_get_type(),
    }
}

impl Fixed {
    pub fn new() -> Fixed {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_fixed_new()).downcast_unchecked()
        }
    }
}

impl Default for Fixed {
    fn default() -> Self {
        Self::new()
    }
}

pub trait FixedExt {
    fn move_<P: IsA<Widget>>(&self, widget: &P, x: i32, y: i32);

    fn put<P: IsA<Widget>>(&self, widget: &P, x: i32, y: i32);
}

impl<O: IsA<Fixed>> FixedExt for O {
    fn move_<P: IsA<Widget>>(&self, widget: &P, x: i32, y: i32) {
        unsafe {
            ffi::gtk_fixed_move(self.to_glib_none().0, widget.to_glib_none().0, x, y);
        }
    }

    fn put<P: IsA<Widget>>(&self, widget: &P, x: i32, y: i32) {
        unsafe {
            ffi::gtk_fixed_put(self.to_glib_none().0, widget.to_glib_none().0, x, y);
        }
    }
}
