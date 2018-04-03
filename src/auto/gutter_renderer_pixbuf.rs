// This file was generated by gir (https://github.com/gtk-rs/gir @ 6855214)
// from gir-files (https://github.com/gtk-rs/gir-files @ cf55bdc)
// DO NOT EDIT

use GutterRenderer;
use ffi;
use gdk_pixbuf;
use gio;
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
    pub struct GutterRendererPixbuf(Object<ffi::GtkSourceGutterRendererPixbuf, ffi::GtkSourceGutterRendererPixbufClass>): GutterRenderer;

    match fn {
        get_type => || ffi::gtk_source_gutter_renderer_pixbuf_get_type(),
    }
}

impl GutterRendererPixbuf {
    pub fn new() -> GutterRendererPixbuf {
        unsafe {
            GutterRenderer::from_glib_full(ffi::gtk_source_gutter_renderer_pixbuf_new()).downcast_unchecked()
        }
    }
}

impl Default for GutterRendererPixbuf {
    fn default() -> Self {
        Self::new()
    }
}

pub trait GutterRendererPixbufExt {
    fn get_gicon(&self) -> Option<gio::Icon>;

    fn get_icon_name(&self) -> Option<String>;

    fn get_pixbuf(&self) -> Option<gdk_pixbuf::Pixbuf>;

    #[cfg_attr(feature = "v3_10", deprecated)]
    fn get_stock_id(&self) -> Option<String>;

    fn set_gicon<'a, P: IsA<gio::Icon> + 'a, Q: Into<Option<&'a P>>>(&self, icon: Q);

    fn set_icon_name<'a, P: Into<Option<&'a str>>>(&self, icon_name: P);

    fn set_pixbuf<'a, P: Into<Option<&'a gdk_pixbuf::Pixbuf>>>(&self, pixbuf: P);

    #[cfg_attr(feature = "v3_10", deprecated)]
    fn set_stock_id<'a, P: Into<Option<&'a str>>>(&self, stock_id: P);

    fn connect_property_gicon_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_icon_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_pixbuf_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg_attr(feature = "v3_10", deprecated)]
    fn connect_property_stock_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<GutterRendererPixbuf> + IsA<glib::object::Object>> GutterRendererPixbufExt for O {
    fn get_gicon(&self) -> Option<gio::Icon> {
        unsafe {
            from_glib_none(ffi::gtk_source_gutter_renderer_pixbuf_get_gicon(self.to_glib_none().0))
        }
    }

    fn get_icon_name(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_source_gutter_renderer_pixbuf_get_icon_name(self.to_glib_none().0))
        }
    }

    fn get_pixbuf(&self) -> Option<gdk_pixbuf::Pixbuf> {
        unsafe {
            from_glib_none(ffi::gtk_source_gutter_renderer_pixbuf_get_pixbuf(self.to_glib_none().0))
        }
    }

    fn get_stock_id(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_source_gutter_renderer_pixbuf_get_stock_id(self.to_glib_none().0))
        }
    }

    fn set_gicon<'a, P: IsA<gio::Icon> + 'a, Q: Into<Option<&'a P>>>(&self, icon: Q) {
        let icon = icon.into();
        let icon = icon.to_glib_none();
        unsafe {
            ffi::gtk_source_gutter_renderer_pixbuf_set_gicon(self.to_glib_none().0, icon.0);
        }
    }

    fn set_icon_name<'a, P: Into<Option<&'a str>>>(&self, icon_name: P) {
        let icon_name = icon_name.into();
        let icon_name = icon_name.to_glib_none();
        unsafe {
            ffi::gtk_source_gutter_renderer_pixbuf_set_icon_name(self.to_glib_none().0, icon_name.0);
        }
    }

    fn set_pixbuf<'a, P: Into<Option<&'a gdk_pixbuf::Pixbuf>>>(&self, pixbuf: P) {
        let pixbuf = pixbuf.into();
        let pixbuf = pixbuf.to_glib_none();
        unsafe {
            ffi::gtk_source_gutter_renderer_pixbuf_set_pixbuf(self.to_glib_none().0, pixbuf.0);
        }
    }

    fn set_stock_id<'a, P: Into<Option<&'a str>>>(&self, stock_id: P) {
        let stock_id = stock_id.into();
        let stock_id = stock_id.to_glib_none();
        unsafe {
            ffi::gtk_source_gutter_renderer_pixbuf_set_stock_id(self.to_glib_none().0, stock_id.0);
        }
    }

    fn connect_property_gicon_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::gicon",
                transmute(notify_gicon_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_icon_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::icon-name",
                transmute(notify_icon_name_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_pixbuf_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::pixbuf",
                transmute(notify_pixbuf_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_stock_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::stock-id",
                transmute(notify_stock_id_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_gicon_trampoline<P>(this: *mut ffi::GtkSourceGutterRendererPixbuf, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<GutterRendererPixbuf> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&GutterRendererPixbuf::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_icon_name_trampoline<P>(this: *mut ffi::GtkSourceGutterRendererPixbuf, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<GutterRendererPixbuf> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&GutterRendererPixbuf::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_pixbuf_trampoline<P>(this: *mut ffi::GtkSourceGutterRendererPixbuf, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<GutterRendererPixbuf> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&GutterRendererPixbuf::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_stock_id_trampoline<P>(this: *mut ffi::GtkSourceGutterRendererPixbuf, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<GutterRendererPixbuf> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&GutterRendererPixbuf::from_glib_borrow(this).downcast_unchecked())
}
