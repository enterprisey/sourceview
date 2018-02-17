// This file was generated by gir (https://github.com/gtk-rs/gir @ 8b9d0bb)
// from gir-files (https://github.com/gtk-rs/gir-files @ 77d1f70)
// DO NOT EDIT

use Language;
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
    pub struct LanguageManager(Object<ffi::GtkSourceLanguageManager, ffi::GtkSourceLanguageManagerClass>);

    match fn {
        get_type => || ffi::gtk_source_language_manager_get_type(),
    }
}

impl LanguageManager {
    pub fn new() -> LanguageManager {
        unsafe {
            from_glib_full(ffi::gtk_source_language_manager_new())
        }
    }

    pub fn get_default() -> Option<LanguageManager> {
        unsafe {
            from_glib_none(ffi::gtk_source_language_manager_get_default())
        }
    }
}

impl Default for LanguageManager {
    fn default() -> Self {
        Self::new()
    }
}

pub trait LanguageManagerExt {
    fn get_language(&self, id: &str) -> Option<Language>;

    fn get_language_ids(&self) -> Vec<String>;

    fn get_search_path(&self) -> Vec<String>;

    fn guess_language<'a, 'b, P: Into<Option<&'a str>>, Q: Into<Option<&'b str>>>(&self, filename: P, content_type: Q) -> Option<Language>;

    fn set_search_path(&self, dirs: &[&str]);

    fn connect_property_language_ids_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_search_path_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<LanguageManager> + IsA<glib::object::Object>> LanguageManagerExt for O {
    fn get_language(&self, id: &str) -> Option<Language> {
        unsafe {
            from_glib_none(ffi::gtk_source_language_manager_get_language(self.to_glib_none().0, id.to_glib_none().0))
        }
    }

    fn get_language_ids(&self) -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::gtk_source_language_manager_get_language_ids(self.to_glib_none().0))
        }
    }

    fn get_search_path(&self) -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::gtk_source_language_manager_get_search_path(self.to_glib_none().0))
        }
    }

    fn guess_language<'a, 'b, P: Into<Option<&'a str>>, Q: Into<Option<&'b str>>>(&self, filename: P, content_type: Q) -> Option<Language> {
        let filename = filename.into();
        let filename = filename.to_glib_none();
        let content_type = content_type.into();
        let content_type = content_type.to_glib_none();
        unsafe {
            from_glib_none(ffi::gtk_source_language_manager_guess_language(self.to_glib_none().0, filename.0, content_type.0))
        }
    }

    fn set_search_path(&self, dirs: &[&str]) {
        unsafe {
            ffi::gtk_source_language_manager_set_search_path(self.to_glib_none().0, dirs.to_glib_none().0);
        }
    }

    fn connect_property_language_ids_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::language-ids",
                transmute(notify_language_ids_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_search_path_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::search-path",
                transmute(notify_search_path_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_language_ids_trampoline<P>(this: *mut ffi::GtkSourceLanguageManager, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<LanguageManager> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&LanguageManager::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_search_path_trampoline<P>(this: *mut ffi::GtkSourceLanguageManager, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<LanguageManager> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&LanguageManager::from_glib_borrow(this).downcast_unchecked())
}
