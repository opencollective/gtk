// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gio;
use glib;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::GString;
use glib_sys;
use gtk_sys;
use std;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use std::ptr;
use FileChooserAction;
use FileChooserConfirmation;
use FileFilter;
use Widget;

glib_wrapper! {
    pub struct FileChooser(Interface<gtk_sys::GtkFileChooser>);

    match fn {
        get_type => || gtk_sys::gtk_file_chooser_get_type(),
    }
}

pub const NONE_FILE_CHOOSER: Option<&FileChooser> = None;

pub trait FileChooserExt: 'static {
    fn add_filter(&self, filter: &FileFilter);

    fn add_shortcut_folder<P: AsRef<std::path::Path>>(&self, folder: P) -> Result<(), glib::Error>;

    fn add_shortcut_folder_uri(&self, uri: &str) -> Result<(), glib::Error>;

    fn get_action(&self) -> FileChooserAction;

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn get_choice(&self, id: &str) -> Option<GString>;

    fn get_create_folders(&self) -> bool;

    fn get_current_folder(&self) -> Option<std::path::PathBuf>;

    fn get_current_folder_file(&self) -> Option<gio::File>;

    fn get_current_folder_uri(&self) -> Option<GString>;

    fn get_current_name(&self) -> Option<GString>;

    fn get_do_overwrite_confirmation(&self) -> bool;

    fn get_extra_widget(&self) -> Option<Widget>;

    fn get_file(&self) -> Option<gio::File>;

    fn get_filename(&self) -> Option<std::path::PathBuf>;

    fn get_filenames(&self) -> Vec<std::path::PathBuf>;

    fn get_files(&self) -> Vec<gio::File>;

    fn get_filter(&self) -> Option<FileFilter>;

    fn get_local_only(&self) -> bool;

    fn get_preview_file(&self) -> Option<gio::File>;

    fn get_preview_filename(&self) -> Option<std::path::PathBuf>;

    fn get_preview_uri(&self) -> Option<GString>;

    fn get_preview_widget(&self) -> Option<Widget>;

    fn get_preview_widget_active(&self) -> bool;

    fn get_select_multiple(&self) -> bool;

    fn get_show_hidden(&self) -> bool;

    fn get_uri(&self) -> Option<GString>;

    fn get_uris(&self) -> Vec<GString>;

    fn get_use_preview_label(&self) -> bool;

    fn list_filters(&self) -> Vec<FileFilter>;

    fn list_shortcut_folder_uris(&self) -> Vec<GString>;

    fn list_shortcut_folders(&self) -> Vec<std::path::PathBuf>;

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn remove_choice(&self, id: &str);

    fn remove_filter(&self, filter: &FileFilter);

    fn remove_shortcut_folder<P: AsRef<std::path::Path>>(
        &self,
        folder: P,
    ) -> Result<(), glib::Error>;

    fn remove_shortcut_folder_uri(&self, uri: &str) -> Result<(), glib::Error>;

    fn select_all(&self);

    fn select_file<P: IsA<gio::File>>(&self, file: &P) -> Result<(), glib::Error>;

    fn select_filename<P: AsRef<std::path::Path>>(&self, filename: P) -> bool;

    fn select_uri(&self, uri: &str) -> bool;

    fn set_action(&self, action: FileChooserAction);

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn set_choice(&self, id: &str, option: &str);

    fn set_create_folders(&self, create_folders: bool);

    fn set_current_folder<P: AsRef<std::path::Path>>(&self, filename: P) -> bool;

    fn set_current_folder_file<P: IsA<gio::File>>(&self, file: &P) -> Result<(), glib::Error>;

    fn set_current_folder_uri(&self, uri: &str) -> bool;

    fn set_current_name<P: AsRef<std::path::Path>>(&self, name: P);

    fn set_do_overwrite_confirmation(&self, do_overwrite_confirmation: bool);

    fn set_extra_widget<P: IsA<Widget>>(&self, extra_widget: &P);

    fn set_file<P: IsA<gio::File>>(&self, file: &P) -> Result<(), glib::Error>;

    fn set_filename<P: AsRef<std::path::Path>>(&self, filename: P) -> bool;

    fn set_filter(&self, filter: &FileFilter);

    fn set_local_only(&self, local_only: bool);

    fn set_preview_widget<P: IsA<Widget>>(&self, preview_widget: &P);

    fn set_preview_widget_active(&self, active: bool);

    fn set_select_multiple(&self, select_multiple: bool);

    fn set_show_hidden(&self, show_hidden: bool);

    fn set_uri(&self, uri: &str) -> bool;

    fn set_use_preview_label(&self, use_label: bool);

    fn unselect_all(&self);

    fn unselect_file<P: IsA<gio::File>>(&self, file: &P);

    fn unselect_filename<P: AsRef<std::path::Path>>(&self, filename: P);

    fn unselect_uri(&self, uri: &str);

    fn connect_confirm_overwrite<F: Fn(&Self) -> FileChooserConfirmation + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_current_folder_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_file_activated<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_selection_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_update_preview<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_action_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_create_folders_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_do_overwrite_confirmation_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_extra_widget_notify<F: Fn(&Self) + 'static>(&self, f: F)
        -> SignalHandlerId;

    fn connect_property_filter_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_local_only_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_preview_widget_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_preview_widget_active_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_select_multiple_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_show_hidden_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_use_preview_label_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<FileChooser>> FileChooserExt for O {
    fn add_filter(&self, filter: &FileFilter) {
        unsafe {
            gtk_sys::gtk_file_chooser_add_filter(
                self.as_ref().to_glib_none().0,
                filter.to_glib_full(),
            );
        }
    }

    fn add_shortcut_folder<P: AsRef<std::path::Path>>(&self, folder: P) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = gtk_sys::gtk_file_chooser_add_shortcut_folder(
                self.as_ref().to_glib_none().0,
                folder.as_ref().to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn add_shortcut_folder_uri(&self, uri: &str) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = gtk_sys::gtk_file_chooser_add_shortcut_folder_uri(
                self.as_ref().to_glib_none().0,
                uri.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn get_action(&self) -> FileChooserAction {
        unsafe {
            from_glib(gtk_sys::gtk_file_chooser_get_action(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn get_choice(&self, id: &str) -> Option<GString> {
        unsafe {
            from_glib_none(gtk_sys::gtk_file_chooser_get_choice(
                self.as_ref().to_glib_none().0,
                id.to_glib_none().0,
            ))
        }
    }

    fn get_create_folders(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_file_chooser_get_create_folders(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_current_folder(&self) -> Option<std::path::PathBuf> {
        unsafe {
            from_glib_full(gtk_sys::gtk_file_chooser_get_current_folder(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_current_folder_file(&self) -> Option<gio::File> {
        unsafe {
            from_glib_full(gtk_sys::gtk_file_chooser_get_current_folder_file(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_current_folder_uri(&self) -> Option<GString> {
        unsafe {
            from_glib_full(gtk_sys::gtk_file_chooser_get_current_folder_uri(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_current_name(&self) -> Option<GString> {
        unsafe {
            from_glib_full(gtk_sys::gtk_file_chooser_get_current_name(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_do_overwrite_confirmation(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_file_chooser_get_do_overwrite_confirmation(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_extra_widget(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(gtk_sys::gtk_file_chooser_get_extra_widget(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_file(&self) -> Option<gio::File> {
        unsafe {
            from_glib_full(gtk_sys::gtk_file_chooser_get_file(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_filename(&self) -> Option<std::path::PathBuf> {
        unsafe {
            from_glib_full(gtk_sys::gtk_file_chooser_get_filename(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_filenames(&self) -> Vec<std::path::PathBuf> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(gtk_sys::gtk_file_chooser_get_filenames(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_files(&self) -> Vec<gio::File> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(gtk_sys::gtk_file_chooser_get_files(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_filter(&self) -> Option<FileFilter> {
        unsafe {
            from_glib_none(gtk_sys::gtk_file_chooser_get_filter(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_local_only(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_file_chooser_get_local_only(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_preview_file(&self) -> Option<gio::File> {
        unsafe {
            from_glib_full(gtk_sys::gtk_file_chooser_get_preview_file(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_preview_filename(&self) -> Option<std::path::PathBuf> {
        unsafe {
            from_glib_full(gtk_sys::gtk_file_chooser_get_preview_filename(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_preview_uri(&self) -> Option<GString> {
        unsafe {
            from_glib_full(gtk_sys::gtk_file_chooser_get_preview_uri(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_preview_widget(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(gtk_sys::gtk_file_chooser_get_preview_widget(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_preview_widget_active(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_file_chooser_get_preview_widget_active(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_select_multiple(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_file_chooser_get_select_multiple(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_show_hidden(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_file_chooser_get_show_hidden(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_uri(&self) -> Option<GString> {
        unsafe {
            from_glib_full(gtk_sys::gtk_file_chooser_get_uri(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_uris(&self) -> Vec<GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(gtk_sys::gtk_file_chooser_get_uris(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_use_preview_label(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_file_chooser_get_use_preview_label(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn list_filters(&self) -> Vec<FileFilter> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(gtk_sys::gtk_file_chooser_list_filters(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn list_shortcut_folder_uris(&self) -> Vec<GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(
                gtk_sys::gtk_file_chooser_list_shortcut_folder_uris(self.as_ref().to_glib_none().0),
            )
        }
    }

    fn list_shortcut_folders(&self) -> Vec<std::path::PathBuf> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(gtk_sys::gtk_file_chooser_list_shortcut_folders(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn remove_choice(&self, id: &str) {
        unsafe {
            gtk_sys::gtk_file_chooser_remove_choice(
                self.as_ref().to_glib_none().0,
                id.to_glib_none().0,
            );
        }
    }

    fn remove_filter(&self, filter: &FileFilter) {
        unsafe {
            gtk_sys::gtk_file_chooser_remove_filter(
                self.as_ref().to_glib_none().0,
                filter.to_glib_none().0,
            );
        }
    }

    fn remove_shortcut_folder<P: AsRef<std::path::Path>>(
        &self,
        folder: P,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = gtk_sys::gtk_file_chooser_remove_shortcut_folder(
                self.as_ref().to_glib_none().0,
                folder.as_ref().to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn remove_shortcut_folder_uri(&self, uri: &str) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = gtk_sys::gtk_file_chooser_remove_shortcut_folder_uri(
                self.as_ref().to_glib_none().0,
                uri.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn select_all(&self) {
        unsafe {
            gtk_sys::gtk_file_chooser_select_all(self.as_ref().to_glib_none().0);
        }
    }

    fn select_file<P: IsA<gio::File>>(&self, file: &P) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = gtk_sys::gtk_file_chooser_select_file(
                self.as_ref().to_glib_none().0,
                file.as_ref().to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn select_filename<P: AsRef<std::path::Path>>(&self, filename: P) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_file_chooser_select_filename(
                self.as_ref().to_glib_none().0,
                filename.as_ref().to_glib_none().0,
            ))
        }
    }

    fn select_uri(&self, uri: &str) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_file_chooser_select_uri(
                self.as_ref().to_glib_none().0,
                uri.to_glib_none().0,
            ))
        }
    }

    fn set_action(&self, action: FileChooserAction) {
        unsafe {
            gtk_sys::gtk_file_chooser_set_action(self.as_ref().to_glib_none().0, action.to_glib());
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn set_choice(&self, id: &str, option: &str) {
        unsafe {
            gtk_sys::gtk_file_chooser_set_choice(
                self.as_ref().to_glib_none().0,
                id.to_glib_none().0,
                option.to_glib_none().0,
            );
        }
    }

    fn set_create_folders(&self, create_folders: bool) {
        unsafe {
            gtk_sys::gtk_file_chooser_set_create_folders(
                self.as_ref().to_glib_none().0,
                create_folders.to_glib(),
            );
        }
    }

    fn set_current_folder<P: AsRef<std::path::Path>>(&self, filename: P) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_file_chooser_set_current_folder(
                self.as_ref().to_glib_none().0,
                filename.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_current_folder_file<P: IsA<gio::File>>(&self, file: &P) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = gtk_sys::gtk_file_chooser_set_current_folder_file(
                self.as_ref().to_glib_none().0,
                file.as_ref().to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn set_current_folder_uri(&self, uri: &str) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_file_chooser_set_current_folder_uri(
                self.as_ref().to_glib_none().0,
                uri.to_glib_none().0,
            ))
        }
    }

    fn set_current_name<P: AsRef<std::path::Path>>(&self, name: P) {
        unsafe {
            gtk_sys::gtk_file_chooser_set_current_name(
                self.as_ref().to_glib_none().0,
                name.as_ref().to_glib_none().0,
            );
        }
    }

    fn set_do_overwrite_confirmation(&self, do_overwrite_confirmation: bool) {
        unsafe {
            gtk_sys::gtk_file_chooser_set_do_overwrite_confirmation(
                self.as_ref().to_glib_none().0,
                do_overwrite_confirmation.to_glib(),
            );
        }
    }

    fn set_extra_widget<P: IsA<Widget>>(&self, extra_widget: &P) {
        unsafe {
            gtk_sys::gtk_file_chooser_set_extra_widget(
                self.as_ref().to_glib_none().0,
                extra_widget.as_ref().to_glib_none().0,
            );
        }
    }

    fn set_file<P: IsA<gio::File>>(&self, file: &P) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = gtk_sys::gtk_file_chooser_set_file(
                self.as_ref().to_glib_none().0,
                file.as_ref().to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn set_filename<P: AsRef<std::path::Path>>(&self, filename: P) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_file_chooser_set_filename(
                self.as_ref().to_glib_none().0,
                filename.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_filter(&self, filter: &FileFilter) {
        unsafe {
            gtk_sys::gtk_file_chooser_set_filter(
                self.as_ref().to_glib_none().0,
                filter.to_glib_none().0,
            );
        }
    }

    fn set_local_only(&self, local_only: bool) {
        unsafe {
            gtk_sys::gtk_file_chooser_set_local_only(
                self.as_ref().to_glib_none().0,
                local_only.to_glib(),
            );
        }
    }

    fn set_preview_widget<P: IsA<Widget>>(&self, preview_widget: &P) {
        unsafe {
            gtk_sys::gtk_file_chooser_set_preview_widget(
                self.as_ref().to_glib_none().0,
                preview_widget.as_ref().to_glib_none().0,
            );
        }
    }

    fn set_preview_widget_active(&self, active: bool) {
        unsafe {
            gtk_sys::gtk_file_chooser_set_preview_widget_active(
                self.as_ref().to_glib_none().0,
                active.to_glib(),
            );
        }
    }

    fn set_select_multiple(&self, select_multiple: bool) {
        unsafe {
            gtk_sys::gtk_file_chooser_set_select_multiple(
                self.as_ref().to_glib_none().0,
                select_multiple.to_glib(),
            );
        }
    }

    fn set_show_hidden(&self, show_hidden: bool) {
        unsafe {
            gtk_sys::gtk_file_chooser_set_show_hidden(
                self.as_ref().to_glib_none().0,
                show_hidden.to_glib(),
            );
        }
    }

    fn set_uri(&self, uri: &str) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_file_chooser_set_uri(
                self.as_ref().to_glib_none().0,
                uri.to_glib_none().0,
            ))
        }
    }

    fn set_use_preview_label(&self, use_label: bool) {
        unsafe {
            gtk_sys::gtk_file_chooser_set_use_preview_label(
                self.as_ref().to_glib_none().0,
                use_label.to_glib(),
            );
        }
    }

    fn unselect_all(&self) {
        unsafe {
            gtk_sys::gtk_file_chooser_unselect_all(self.as_ref().to_glib_none().0);
        }
    }

    fn unselect_file<P: IsA<gio::File>>(&self, file: &P) {
        unsafe {
            gtk_sys::gtk_file_chooser_unselect_file(
                self.as_ref().to_glib_none().0,
                file.as_ref().to_glib_none().0,
            );
        }
    }

    fn unselect_filename<P: AsRef<std::path::Path>>(&self, filename: P) {
        unsafe {
            gtk_sys::gtk_file_chooser_unselect_filename(
                self.as_ref().to_glib_none().0,
                filename.as_ref().to_glib_none().0,
            );
        }
    }

    fn unselect_uri(&self, uri: &str) {
        unsafe {
            gtk_sys::gtk_file_chooser_unselect_uri(
                self.as_ref().to_glib_none().0,
                uri.to_glib_none().0,
            );
        }
    }

    fn connect_confirm_overwrite<F: Fn(&Self) -> FileChooserConfirmation + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn confirm_overwrite_trampoline<
            P,
            F: Fn(&P) -> FileChooserConfirmation + 'static,
        >(
            this: *mut gtk_sys::GtkFileChooser,
            f: glib_sys::gpointer,
        ) -> gtk_sys::GtkFileChooserConfirmation
        where
            P: IsA<FileChooser>,
        {
            let f: &F = &*(f as *const F);
            f(&FileChooser::from_glib_borrow(this).unsafe_cast_ref()).to_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"confirm-overwrite\0".as_ptr() as *const _,
                Some(transmute(confirm_overwrite_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_current_folder_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn current_folder_changed_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkFileChooser,
            f: glib_sys::gpointer,
        ) where
            P: IsA<FileChooser>,
        {
            let f: &F = &*(f as *const F);
            f(&FileChooser::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"current-folder-changed\0".as_ptr() as *const _,
                Some(transmute(
                    current_folder_changed_trampoline::<Self, F> as usize,
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_file_activated<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn file_activated_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkFileChooser,
            f: glib_sys::gpointer,
        ) where
            P: IsA<FileChooser>,
        {
            let f: &F = &*(f as *const F);
            f(&FileChooser::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"file-activated\0".as_ptr() as *const _,
                Some(transmute(file_activated_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_selection_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn selection_changed_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkFileChooser,
            f: glib_sys::gpointer,
        ) where
            P: IsA<FileChooser>,
        {
            let f: &F = &*(f as *const F);
            f(&FileChooser::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"selection-changed\0".as_ptr() as *const _,
                Some(transmute(selection_changed_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_update_preview<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn update_preview_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkFileChooser,
            f: glib_sys::gpointer,
        ) where
            P: IsA<FileChooser>,
        {
            let f: &F = &*(f as *const F);
            f(&FileChooser::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"update-preview\0".as_ptr() as *const _,
                Some(transmute(update_preview_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_action_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_action_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkFileChooser,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<FileChooser>,
        {
            let f: &F = &*(f as *const F);
            f(&FileChooser::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::action\0".as_ptr() as *const _,
                Some(transmute(notify_action_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_create_folders_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_create_folders_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkFileChooser,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<FileChooser>,
        {
            let f: &F = &*(f as *const F);
            f(&FileChooser::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::create-folders\0".as_ptr() as *const _,
                Some(transmute(
                    notify_create_folders_trampoline::<Self, F> as usize,
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_do_overwrite_confirmation_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_do_overwrite_confirmation_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkFileChooser,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<FileChooser>,
        {
            let f: &F = &*(f as *const F);
            f(&FileChooser::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::do-overwrite-confirmation\0".as_ptr() as *const _,
                Some(transmute(
                    notify_do_overwrite_confirmation_trampoline::<Self, F> as usize,
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_extra_widget_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_extra_widget_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkFileChooser,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<FileChooser>,
        {
            let f: &F = &*(f as *const F);
            f(&FileChooser::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::extra-widget\0".as_ptr() as *const _,
                Some(transmute(
                    notify_extra_widget_trampoline::<Self, F> as usize,
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_filter_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_filter_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkFileChooser,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<FileChooser>,
        {
            let f: &F = &*(f as *const F);
            f(&FileChooser::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::filter\0".as_ptr() as *const _,
                Some(transmute(notify_filter_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_local_only_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_local_only_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkFileChooser,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<FileChooser>,
        {
            let f: &F = &*(f as *const F);
            f(&FileChooser::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::local-only\0".as_ptr() as *const _,
                Some(transmute(notify_local_only_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_preview_widget_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_preview_widget_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkFileChooser,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<FileChooser>,
        {
            let f: &F = &*(f as *const F);
            f(&FileChooser::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::preview-widget\0".as_ptr() as *const _,
                Some(transmute(
                    notify_preview_widget_trampoline::<Self, F> as usize,
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_preview_widget_active_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_preview_widget_active_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkFileChooser,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<FileChooser>,
        {
            let f: &F = &*(f as *const F);
            f(&FileChooser::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::preview-widget-active\0".as_ptr() as *const _,
                Some(transmute(
                    notify_preview_widget_active_trampoline::<Self, F> as usize,
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_select_multiple_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_select_multiple_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkFileChooser,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<FileChooser>,
        {
            let f: &F = &*(f as *const F);
            f(&FileChooser::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::select-multiple\0".as_ptr() as *const _,
                Some(transmute(
                    notify_select_multiple_trampoline::<Self, F> as usize,
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_show_hidden_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_hidden_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkFileChooser,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<FileChooser>,
        {
            let f: &F = &*(f as *const F);
            f(&FileChooser::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::show-hidden\0".as_ptr() as *const _,
                Some(transmute(notify_show_hidden_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_use_preview_label_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_use_preview_label_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkFileChooser,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<FileChooser>,
        {
            let f: &F = &*(f as *const F);
            f(&FileChooser::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::use-preview-label\0".as_ptr() as *const _,
                Some(transmute(
                    notify_use_preview_label_trampoline::<Self, F> as usize,
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for FileChooser {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FileChooser")
    }
}
