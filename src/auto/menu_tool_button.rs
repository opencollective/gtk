// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Actionable;
use Bin;
use Buildable;
use Container;
use ToolButton;
use ToolItem;
use Widget;
use ffi;
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

glib_wrapper! {
    pub struct MenuToolButton(Object<ffi::GtkMenuToolButton, ffi::GtkMenuToolButtonClass>): ToolButton, ToolItem, Bin, Container, Widget, Buildable, Actionable;

    match fn {
        get_type => || ffi::gtk_menu_tool_button_get_type(),
    }
}

impl MenuToolButton {
    pub fn new<'a, 'b, P: IsA<Widget> + 'a, Q: Into<Option<&'a P>>, R: Into<Option<&'b str>>>(icon_widget: Q, label: R) -> MenuToolButton {
        assert_initialized_main_thread!();
        let icon_widget = icon_widget.into();
        let icon_widget = icon_widget.to_glib_none();
        let label = label.into();
        let label = label.to_glib_none();
        unsafe {
            ToolItem::from_glib_none(ffi::gtk_menu_tool_button_new(icon_widget.0, label.0)).downcast_unchecked()
        }
    }

    #[cfg_attr(feature = "v3_10", deprecated)]
    pub fn new_from_stock(stock_id: &str) -> MenuToolButton {
        assert_initialized_main_thread!();
        unsafe {
            ToolItem::from_glib_none(ffi::gtk_menu_tool_button_new_from_stock(stock_id.to_glib_none().0)).downcast_unchecked()
        }
    }
}

pub trait MenuToolButtonExt {
    fn get_menu(&self) -> Option<Widget>;

    fn set_arrow_tooltip_markup(&self, markup: &str);

    fn set_arrow_tooltip_text(&self, text: &str);

    fn set_menu<P: IsA<Widget>>(&self, menu: &P);

    fn connect_show_menu<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_menu_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<MenuToolButton> + IsA<glib::object::Object>> MenuToolButtonExt for O {
    fn get_menu(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_menu_tool_button_get_menu(self.to_glib_none().0))
        }
    }

    fn set_arrow_tooltip_markup(&self, markup: &str) {
        unsafe {
            ffi::gtk_menu_tool_button_set_arrow_tooltip_markup(self.to_glib_none().0, markup.to_glib_none().0);
        }
    }

    fn set_arrow_tooltip_text(&self, text: &str) {
        unsafe {
            ffi::gtk_menu_tool_button_set_arrow_tooltip_text(self.to_glib_none().0, text.to_glib_none().0);
        }
    }

    fn set_menu<P: IsA<Widget>>(&self, menu: &P) {
        unsafe {
            ffi::gtk_menu_tool_button_set_menu(self.to_glib_none().0, menu.to_glib_none().0);
        }
    }

    fn connect_show_menu<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "show-menu",
                transmute(show_menu_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_menu_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::menu",
                transmute(notify_menu_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn show_menu_trampoline<P>(this: *mut ffi::GtkMenuToolButton, f: glib_ffi::gpointer)
where P: IsA<MenuToolButton> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&MenuToolButton::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_menu_trampoline<P>(this: *mut ffi::GtkMenuToolButton, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<MenuToolButton> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&MenuToolButton::from_glib_borrow(this).downcast_unchecked())
}
