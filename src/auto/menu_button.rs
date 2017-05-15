// This file was generated by gir (8343e00) from gir-files (71d73f0)
// DO NOT EDIT

use Actionable;
#[cfg(feature = "v3_6")]
use ArrowType;
use Bin;
use Button;
use Container;
#[cfg(feature = "v3_6")]
use Menu;
#[cfg(feature = "v3_12")]
use Popover;
use ToggleButton;
use Widget;
use ffi;
#[cfg(feature = "v3_6")]
use gio;
use glib::object::Downcast;
use glib::object::IsA;
use glib::translate::*;

glib_wrapper! {
    pub struct MenuButton(Object<ffi::GtkMenuButton>): ToggleButton, Button, Bin, Container, Widget, Actionable;

    match fn {
        get_type => || ffi::gtk_menu_button_get_type(),
    }
}

impl MenuButton {
    #[cfg(feature = "v3_6")]
    pub fn new() -> MenuButton {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_menu_button_new()).downcast_unchecked()
        }
    }
}

pub trait MenuButtonExt {
    #[cfg(feature = "v3_6")]
    fn get_align_widget(&self) -> Option<Widget>;

    #[cfg(feature = "v3_6")]
    fn get_direction(&self) -> ArrowType;

    #[cfg(feature = "v3_6")]
    fn get_menu_model(&self) -> Option<gio::MenuModel>;

    #[cfg(feature = "v3_12")]
    fn get_popover(&self) -> Option<Popover>;

    #[cfg(feature = "v3_6")]
    fn get_popup(&self) -> Option<Menu>;

    #[cfg(feature = "v3_12")]
    fn get_use_popover(&self) -> bool;

    #[cfg(feature = "v3_6")]
    fn set_align_widget<'a, P: IsA<Widget> + 'a, Q: Into<Option<&'a P>>>(&self, align_widget: Q);

    #[cfg(feature = "v3_6")]
    fn set_direction(&self, direction: ArrowType);

    #[cfg(feature = "v3_6")]
    fn set_menu_model<'a, P: IsA<gio::MenuModel> + 'a, Q: Into<Option<&'a P>>>(&self, menu_model: Q);

    #[cfg(feature = "v3_12")]
    fn set_popover<'a, P: IsA<Widget> + 'a, Q: Into<Option<&'a P>>>(&self, popover: Q);

    #[cfg(feature = "v3_6")]
    fn set_popup<'a, P: IsA<Widget> + 'a, Q: Into<Option<&'a P>>>(&self, menu: Q);

    #[cfg(feature = "v3_12")]
    fn set_use_popover(&self, use_popover: bool);
}

impl<O: IsA<MenuButton>> MenuButtonExt for O {
    #[cfg(feature = "v3_6")]
    fn get_align_widget(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_menu_button_get_align_widget(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_6")]
    fn get_direction(&self) -> ArrowType {
        unsafe {
            from_glib(ffi::gtk_menu_button_get_direction(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_6")]
    fn get_menu_model(&self) -> Option<gio::MenuModel> {
        unsafe {
            from_glib_none(ffi::gtk_menu_button_get_menu_model(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_12")]
    fn get_popover(&self) -> Option<Popover> {
        unsafe {
            from_glib_none(ffi::gtk_menu_button_get_popover(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_6")]
    fn get_popup(&self) -> Option<Menu> {
        unsafe {
            from_glib_none(ffi::gtk_menu_button_get_popup(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_12")]
    fn get_use_popover(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_menu_button_get_use_popover(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_6")]
    fn set_align_widget<'a, P: IsA<Widget> + 'a, Q: Into<Option<&'a P>>>(&self, align_widget: Q) {
        let align_widget = align_widget.into();
        let align_widget = align_widget.to_glib_none();
        unsafe {
            ffi::gtk_menu_button_set_align_widget(self.to_glib_none().0, align_widget.0);
        }
    }

    #[cfg(feature = "v3_6")]
    fn set_direction(&self, direction: ArrowType) {
        unsafe {
            ffi::gtk_menu_button_set_direction(self.to_glib_none().0, direction.to_glib());
        }
    }

    #[cfg(feature = "v3_6")]
    fn set_menu_model<'a, P: IsA<gio::MenuModel> + 'a, Q: Into<Option<&'a P>>>(&self, menu_model: Q) {
        let menu_model = menu_model.into();
        let menu_model = menu_model.to_glib_none();
        unsafe {
            ffi::gtk_menu_button_set_menu_model(self.to_glib_none().0, menu_model.0);
        }
    }

    #[cfg(feature = "v3_12")]
    fn set_popover<'a, P: IsA<Widget> + 'a, Q: Into<Option<&'a P>>>(&self, popover: Q) {
        let popover = popover.into();
        let popover = popover.to_glib_none();
        unsafe {
            ffi::gtk_menu_button_set_popover(self.to_glib_none().0, popover.0);
        }
    }

    #[cfg(feature = "v3_6")]
    fn set_popup<'a, P: IsA<Widget> + 'a, Q: Into<Option<&'a P>>>(&self, menu: Q) {
        let menu = menu.into();
        let menu = menu.to_glib_none();
        unsafe {
            ffi::gtk_menu_button_set_popup(self.to_glib_none().0, menu.0);
        }
    }

    #[cfg(feature = "v3_12")]
    fn set_use_popover(&self, use_popover: bool) {
        unsafe {
            ffi::gtk_menu_button_set_use_popover(self.to_glib_none().0, use_popover.to_glib());
        }
    }
}
