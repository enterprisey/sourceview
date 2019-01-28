// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(any(feature = "v3_16", feature = "dox"))]
use StyleScheme;
use ffi;
#[cfg(any(feature = "v3_16", feature = "dox"))]
use glib::object::Cast;
use glib::object::IsA;
#[cfg(any(feature = "v3_16", feature = "dox"))]
use glib::signal::SignalHandlerId;
#[cfg(any(feature = "v3_16", feature = "dox"))]
use glib::signal::connect_raw;
use glib::translate::*;
#[cfg(any(feature = "v3_16", feature = "dox"))]
use glib_ffi;
#[cfg(any(feature = "v3_16", feature = "dox"))]
use std::boxed::Box as Box_;
use std::fmt;
#[cfg(any(feature = "v3_16", feature = "dox"))]
use std::mem::transmute;

glib_wrapper! {
    pub struct StyleSchemeChooser(Interface<ffi::GtkSourceStyleSchemeChooser>);

    match fn {
        get_type => || ffi::gtk_source_style_scheme_chooser_get_type(),
    }
}

pub const NONE_STYLE_SCHEME_CHOOSER: Option<&StyleSchemeChooser> = None;

pub trait StyleSchemeChooserExt: 'static {
    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn get_style_scheme(&self) -> Option<StyleScheme>;

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn set_style_scheme<P: IsA<StyleScheme>>(&self, scheme: &P);

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn connect_property_style_scheme_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<StyleSchemeChooser>> StyleSchemeChooserExt for O {
    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn get_style_scheme(&self) -> Option<StyleScheme> {
        unsafe {
            from_glib_none(ffi::gtk_source_style_scheme_chooser_get_style_scheme(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn set_style_scheme<P: IsA<StyleScheme>>(&self, scheme: &P) {
        unsafe {
            ffi::gtk_source_style_scheme_chooser_set_style_scheme(self.as_ref().to_glib_none().0, scheme.as_ref().to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn connect_property_style_scheme_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::style-scheme\0".as_ptr() as *const _,
                Some(transmute(notify_style_scheme_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

#[cfg(any(feature = "v3_16", feature = "dox"))]
unsafe extern "C" fn notify_style_scheme_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkSourceStyleSchemeChooser, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<StyleSchemeChooser> {
    let f: &F = transmute(f);
    f(&StyleSchemeChooser::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for StyleSchemeChooser {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "StyleSchemeChooser")
    }
}
