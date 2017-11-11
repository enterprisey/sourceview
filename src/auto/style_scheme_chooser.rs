// This file was generated by gir (e58a8db) from gir-files (469db10)
// DO NOT EDIT

#[cfg(any(feature = "v3_16", feature = "dox"))]
use StyleScheme;
use ffi;
#[cfg(any(feature = "v3_16", feature = "dox"))]
use glib;
#[cfg(any(feature = "v3_16", feature = "dox"))]
use glib::object::Downcast;
use glib::object::IsA;
#[cfg(any(feature = "v3_16", feature = "dox"))]
use glib::signal::SignalHandlerId;
#[cfg(any(feature = "v3_16", feature = "dox"))]
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
#[cfg(any(feature = "v3_16", feature = "dox"))]
use std::boxed::Box as Box_;
use std::mem;
#[cfg(any(feature = "v3_16", feature = "dox"))]
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct StyleSchemeChooser(Object<ffi::GtkSourceStyleSchemeChooser, ffi::GtkSourceStyleSchemeChooserInterface>);

    match fn {
        get_type => || ffi::gtk_source_style_scheme_chooser_get_type(),
    }
}

pub trait StyleSchemeChooserExt {
    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn get_style_scheme(&self) -> Option<StyleScheme>;

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn set_style_scheme(&self, scheme: &StyleScheme);

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn connect_property_style_scheme_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<StyleSchemeChooser> + IsA<glib::object::Object>> StyleSchemeChooserExt for O {
    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn get_style_scheme(&self) -> Option<StyleScheme> {
        unsafe {
            from_glib_none(ffi::gtk_source_style_scheme_chooser_get_style_scheme(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn set_style_scheme(&self, scheme: &StyleScheme) {
        unsafe {
            ffi::gtk_source_style_scheme_chooser_set_style_scheme(self.to_glib_none().0, scheme.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn connect_property_style_scheme_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::style-scheme",
                transmute(notify_style_scheme_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

#[cfg(any(feature = "v3_16", feature = "dox"))]
unsafe extern "C" fn notify_style_scheme_trampoline<P>(this: *mut ffi::GtkSourceStyleSchemeChooser, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<StyleSchemeChooser> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&StyleSchemeChooser::from_glib_borrow(this).downcast_unchecked())
}
