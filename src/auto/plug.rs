// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Bin;
use Buildable;
use Container;
use Widget;
use Window;
use ffi;
use gdk;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;
use xlib;

glib_wrapper! {
    pub struct Plug(Object<ffi::GtkPlug, ffi::GtkPlugClass>): Window, Bin, Container, Widget, Buildable;

    match fn {
        get_type => || ffi::gtk_plug_get_type(),
    }
}

impl Plug {
    pub fn new(socket_id: xlib::Window) -> Plug {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_plug_new(socket_id)).downcast_unchecked()
        }
    }

    pub fn new_for_display(display: &gdk::Display, socket_id: xlib::Window) -> Plug {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_plug_new_for_display(display.to_glib_none().0, socket_id)).downcast_unchecked()
        }
    }
}

pub trait PlugExt {
    fn construct(&self, socket_id: xlib::Window);

    fn construct_for_display(&self, display: &gdk::Display, socket_id: xlib::Window);

    fn get_embedded(&self) -> bool;

    fn get_id(&self) -> xlib::Window;

    fn get_socket_window(&self) -> Option<gdk::Window>;

    fn connect_embedded<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_embedded_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_socket_window_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Plug> + IsA<glib::object::Object>> PlugExt for O {
    fn construct(&self, socket_id: xlib::Window) {
        unsafe {
            ffi::gtk_plug_construct(self.to_glib_none().0, socket_id);
        }
    }

    fn construct_for_display(&self, display: &gdk::Display, socket_id: xlib::Window) {
        unsafe {
            ffi::gtk_plug_construct_for_display(self.to_glib_none().0, display.to_glib_none().0, socket_id);
        }
    }

    fn get_embedded(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_plug_get_embedded(self.to_glib_none().0))
        }
    }

    fn get_id(&self) -> xlib::Window {
        unsafe {
            ffi::gtk_plug_get_id(self.to_glib_none().0)
        }
    }

    fn get_socket_window(&self) -> Option<gdk::Window> {
        unsafe {
            from_glib_none(ffi::gtk_plug_get_socket_window(self.to_glib_none().0))
        }
    }

    fn connect_embedded<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "embedded",
                transmute(embedded_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_embedded_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::embedded",
                transmute(notify_embedded_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_socket_window_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::socket-window",
                transmute(notify_socket_window_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn embedded_trampoline<P>(this: *mut ffi::GtkPlug, f: glib_ffi::gpointer)
where P: IsA<Plug> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Plug::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_embedded_trampoline<P>(this: *mut ffi::GtkPlug, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Plug> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Plug::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_socket_window_trampoline<P>(this: *mut ffi::GtkPlug, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Plug> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Plug::from_glib_borrow(this).downcast_unchecked())
}
