// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Buffer;
use View;
use glib::GString;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_sys;
use gtk;
use gtk_source_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct PrintCompositor(Object<gtk_source_sys::GtkSourcePrintCompositor, gtk_source_sys::GtkSourcePrintCompositorClass, PrintCompositorClass>);

    match fn {
        get_type => || gtk_source_sys::gtk_source_print_compositor_get_type(),
    }
}

impl PrintCompositor {
    pub fn new<P: IsA<Buffer>>(buffer: &P) -> PrintCompositor {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(gtk_source_sys::gtk_source_print_compositor_new(buffer.as_ref().to_glib_none().0))
        }
    }

    pub fn new_from_view<P: IsA<View>>(view: &P) -> PrintCompositor {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(gtk_source_sys::gtk_source_print_compositor_new_from_view(view.as_ref().to_glib_none().0))
        }
    }
}

pub const NONE_PRINT_COMPOSITOR: Option<&PrintCompositor> = None;

pub trait PrintCompositorExt: 'static {
    fn draw_page(&self, context: &gtk::PrintContext, page_nr: i32);

    fn get_body_font_name(&self) -> Option<GString>;

    fn get_bottom_margin(&self, unit: gtk::Unit) -> f64;

    fn get_buffer(&self) -> Option<Buffer>;

    fn get_footer_font_name(&self) -> Option<GString>;

    fn get_header_font_name(&self) -> Option<GString>;

    fn get_highlight_syntax(&self) -> bool;

    fn get_left_margin(&self, unit: gtk::Unit) -> f64;

    fn get_line_numbers_font_name(&self) -> Option<GString>;

    fn get_n_pages(&self) -> i32;

    fn get_pagination_progress(&self) -> f64;

    fn get_print_footer(&self) -> bool;

    fn get_print_header(&self) -> bool;

    fn get_print_line_numbers(&self) -> u32;

    fn get_right_margin(&self, unit: gtk::Unit) -> f64;

    fn get_tab_width(&self) -> u32;

    fn get_top_margin(&self, unit: gtk::Unit) -> f64;

    fn get_wrap_mode(&self) -> gtk::WrapMode;

    fn paginate(&self, context: &gtk::PrintContext) -> bool;

    fn set_body_font_name(&self, font_name: &str);

    fn set_bottom_margin(&self, margin: f64, unit: gtk::Unit);

    fn set_footer_font_name(&self, font_name: Option<&str>);

    fn set_footer_format(&self, separator: bool, left: Option<&str>, center: Option<&str>, right: Option<&str>);

    fn set_header_font_name(&self, font_name: Option<&str>);

    fn set_header_format(&self, separator: bool, left: Option<&str>, center: Option<&str>, right: Option<&str>);

    fn set_highlight_syntax(&self, highlight: bool);

    fn set_left_margin(&self, margin: f64, unit: gtk::Unit);

    fn set_line_numbers_font_name(&self, font_name: Option<&str>);

    fn set_print_footer(&self, print: bool);

    fn set_print_header(&self, print: bool);

    fn set_print_line_numbers(&self, interval: u32);

    fn set_right_margin(&self, margin: f64, unit: gtk::Unit);

    fn set_tab_width(&self, width: u32);

    fn set_top_margin(&self, margin: f64, unit: gtk::Unit);

    fn set_wrap_mode(&self, wrap_mode: gtk::WrapMode);

    fn connect_property_body_font_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_footer_font_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_header_font_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_highlight_syntax_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_line_numbers_font_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_n_pages_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_print_footer_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_print_header_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_print_line_numbers_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_tab_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_wrap_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<PrintCompositor>> PrintCompositorExt for O {
    fn draw_page(&self, context: &gtk::PrintContext, page_nr: i32) {
        unsafe {
            gtk_source_sys::gtk_source_print_compositor_draw_page(self.as_ref().to_glib_none().0, context.to_glib_none().0, page_nr);
        }
    }

    fn get_body_font_name(&self) -> Option<GString> {
        unsafe {
            from_glib_full(gtk_source_sys::gtk_source_print_compositor_get_body_font_name(self.as_ref().to_glib_none().0))
        }
    }

    fn get_bottom_margin(&self, unit: gtk::Unit) -> f64 {
        unsafe {
            gtk_source_sys::gtk_source_print_compositor_get_bottom_margin(self.as_ref().to_glib_none().0, unit.to_glib())
        }
    }

    fn get_buffer(&self) -> Option<Buffer> {
        unsafe {
            from_glib_none(gtk_source_sys::gtk_source_print_compositor_get_buffer(self.as_ref().to_glib_none().0))
        }
    }

    fn get_footer_font_name(&self) -> Option<GString> {
        unsafe {
            from_glib_full(gtk_source_sys::gtk_source_print_compositor_get_footer_font_name(self.as_ref().to_glib_none().0))
        }
    }

    fn get_header_font_name(&self) -> Option<GString> {
        unsafe {
            from_glib_full(gtk_source_sys::gtk_source_print_compositor_get_header_font_name(self.as_ref().to_glib_none().0))
        }
    }

    fn get_highlight_syntax(&self) -> bool {
        unsafe {
            from_glib(gtk_source_sys::gtk_source_print_compositor_get_highlight_syntax(self.as_ref().to_glib_none().0))
        }
    }

    fn get_left_margin(&self, unit: gtk::Unit) -> f64 {
        unsafe {
            gtk_source_sys::gtk_source_print_compositor_get_left_margin(self.as_ref().to_glib_none().0, unit.to_glib())
        }
    }

    fn get_line_numbers_font_name(&self) -> Option<GString> {
        unsafe {
            from_glib_full(gtk_source_sys::gtk_source_print_compositor_get_line_numbers_font_name(self.as_ref().to_glib_none().0))
        }
    }

    fn get_n_pages(&self) -> i32 {
        unsafe {
            gtk_source_sys::gtk_source_print_compositor_get_n_pages(self.as_ref().to_glib_none().0)
        }
    }

    fn get_pagination_progress(&self) -> f64 {
        unsafe {
            gtk_source_sys::gtk_source_print_compositor_get_pagination_progress(self.as_ref().to_glib_none().0)
        }
    }

    fn get_print_footer(&self) -> bool {
        unsafe {
            from_glib(gtk_source_sys::gtk_source_print_compositor_get_print_footer(self.as_ref().to_glib_none().0))
        }
    }

    fn get_print_header(&self) -> bool {
        unsafe {
            from_glib(gtk_source_sys::gtk_source_print_compositor_get_print_header(self.as_ref().to_glib_none().0))
        }
    }

    fn get_print_line_numbers(&self) -> u32 {
        unsafe {
            gtk_source_sys::gtk_source_print_compositor_get_print_line_numbers(self.as_ref().to_glib_none().0)
        }
    }

    fn get_right_margin(&self, unit: gtk::Unit) -> f64 {
        unsafe {
            gtk_source_sys::gtk_source_print_compositor_get_right_margin(self.as_ref().to_glib_none().0, unit.to_glib())
        }
    }

    fn get_tab_width(&self) -> u32 {
        unsafe {
            gtk_source_sys::gtk_source_print_compositor_get_tab_width(self.as_ref().to_glib_none().0)
        }
    }

    fn get_top_margin(&self, unit: gtk::Unit) -> f64 {
        unsafe {
            gtk_source_sys::gtk_source_print_compositor_get_top_margin(self.as_ref().to_glib_none().0, unit.to_glib())
        }
    }

    fn get_wrap_mode(&self) -> gtk::WrapMode {
        unsafe {
            from_glib(gtk_source_sys::gtk_source_print_compositor_get_wrap_mode(self.as_ref().to_glib_none().0))
        }
    }

    fn paginate(&self, context: &gtk::PrintContext) -> bool {
        unsafe {
            from_glib(gtk_source_sys::gtk_source_print_compositor_paginate(self.as_ref().to_glib_none().0, context.to_glib_none().0))
        }
    }

    fn set_body_font_name(&self, font_name: &str) {
        unsafe {
            gtk_source_sys::gtk_source_print_compositor_set_body_font_name(self.as_ref().to_glib_none().0, font_name.to_glib_none().0);
        }
    }

    fn set_bottom_margin(&self, margin: f64, unit: gtk::Unit) {
        unsafe {
            gtk_source_sys::gtk_source_print_compositor_set_bottom_margin(self.as_ref().to_glib_none().0, margin, unit.to_glib());
        }
    }

    fn set_footer_font_name(&self, font_name: Option<&str>) {
        unsafe {
            gtk_source_sys::gtk_source_print_compositor_set_footer_font_name(self.as_ref().to_glib_none().0, font_name.to_glib_none().0);
        }
    }

    fn set_footer_format(&self, separator: bool, left: Option<&str>, center: Option<&str>, right: Option<&str>) {
        unsafe {
            gtk_source_sys::gtk_source_print_compositor_set_footer_format(self.as_ref().to_glib_none().0, separator.to_glib(), left.to_glib_none().0, center.to_glib_none().0, right.to_glib_none().0);
        }
    }

    fn set_header_font_name(&self, font_name: Option<&str>) {
        unsafe {
            gtk_source_sys::gtk_source_print_compositor_set_header_font_name(self.as_ref().to_glib_none().0, font_name.to_glib_none().0);
        }
    }

    fn set_header_format(&self, separator: bool, left: Option<&str>, center: Option<&str>, right: Option<&str>) {
        unsafe {
            gtk_source_sys::gtk_source_print_compositor_set_header_format(self.as_ref().to_glib_none().0, separator.to_glib(), left.to_glib_none().0, center.to_glib_none().0, right.to_glib_none().0);
        }
    }

    fn set_highlight_syntax(&self, highlight: bool) {
        unsafe {
            gtk_source_sys::gtk_source_print_compositor_set_highlight_syntax(self.as_ref().to_glib_none().0, highlight.to_glib());
        }
    }

    fn set_left_margin(&self, margin: f64, unit: gtk::Unit) {
        unsafe {
            gtk_source_sys::gtk_source_print_compositor_set_left_margin(self.as_ref().to_glib_none().0, margin, unit.to_glib());
        }
    }

    fn set_line_numbers_font_name(&self, font_name: Option<&str>) {
        unsafe {
            gtk_source_sys::gtk_source_print_compositor_set_line_numbers_font_name(self.as_ref().to_glib_none().0, font_name.to_glib_none().0);
        }
    }

    fn set_print_footer(&self, print: bool) {
        unsafe {
            gtk_source_sys::gtk_source_print_compositor_set_print_footer(self.as_ref().to_glib_none().0, print.to_glib());
        }
    }

    fn set_print_header(&self, print: bool) {
        unsafe {
            gtk_source_sys::gtk_source_print_compositor_set_print_header(self.as_ref().to_glib_none().0, print.to_glib());
        }
    }

    fn set_print_line_numbers(&self, interval: u32) {
        unsafe {
            gtk_source_sys::gtk_source_print_compositor_set_print_line_numbers(self.as_ref().to_glib_none().0, interval);
        }
    }

    fn set_right_margin(&self, margin: f64, unit: gtk::Unit) {
        unsafe {
            gtk_source_sys::gtk_source_print_compositor_set_right_margin(self.as_ref().to_glib_none().0, margin, unit.to_glib());
        }
    }

    fn set_tab_width(&self, width: u32) {
        unsafe {
            gtk_source_sys::gtk_source_print_compositor_set_tab_width(self.as_ref().to_glib_none().0, width);
        }
    }

    fn set_top_margin(&self, margin: f64, unit: gtk::Unit) {
        unsafe {
            gtk_source_sys::gtk_source_print_compositor_set_top_margin(self.as_ref().to_glib_none().0, margin, unit.to_glib());
        }
    }

    fn set_wrap_mode(&self, wrap_mode: gtk::WrapMode) {
        unsafe {
            gtk_source_sys::gtk_source_print_compositor_set_wrap_mode(self.as_ref().to_glib_none().0, wrap_mode.to_glib());
        }
    }

    fn connect_property_body_font_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::body-font-name\0".as_ptr() as *const _,
                Some(transmute(notify_body_font_name_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_footer_font_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::footer-font-name\0".as_ptr() as *const _,
                Some(transmute(notify_footer_font_name_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_header_font_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::header-font-name\0".as_ptr() as *const _,
                Some(transmute(notify_header_font_name_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_highlight_syntax_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::highlight-syntax\0".as_ptr() as *const _,
                Some(transmute(notify_highlight_syntax_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_line_numbers_font_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::line-numbers-font-name\0".as_ptr() as *const _,
                Some(transmute(notify_line_numbers_font_name_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_n_pages_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::n-pages\0".as_ptr() as *const _,
                Some(transmute(notify_n_pages_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_print_footer_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::print-footer\0".as_ptr() as *const _,
                Some(transmute(notify_print_footer_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_print_header_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::print-header\0".as_ptr() as *const _,
                Some(transmute(notify_print_header_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_print_line_numbers_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::print-line-numbers\0".as_ptr() as *const _,
                Some(transmute(notify_print_line_numbers_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_tab_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::tab-width\0".as_ptr() as *const _,
                Some(transmute(notify_tab_width_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_wrap_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::wrap-mode\0".as_ptr() as *const _,
                Some(transmute(notify_wrap_mode_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn notify_body_font_name_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_source_sys::GtkSourcePrintCompositor, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<PrintCompositor> {
    let f: &F = &*(f as *const F);
    f(&PrintCompositor::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_footer_font_name_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_source_sys::GtkSourcePrintCompositor, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<PrintCompositor> {
    let f: &F = &*(f as *const F);
    f(&PrintCompositor::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_header_font_name_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_source_sys::GtkSourcePrintCompositor, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<PrintCompositor> {
    let f: &F = &*(f as *const F);
    f(&PrintCompositor::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_highlight_syntax_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_source_sys::GtkSourcePrintCompositor, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<PrintCompositor> {
    let f: &F = &*(f as *const F);
    f(&PrintCompositor::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_line_numbers_font_name_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_source_sys::GtkSourcePrintCompositor, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<PrintCompositor> {
    let f: &F = &*(f as *const F);
    f(&PrintCompositor::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_n_pages_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_source_sys::GtkSourcePrintCompositor, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<PrintCompositor> {
    let f: &F = &*(f as *const F);
    f(&PrintCompositor::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_print_footer_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_source_sys::GtkSourcePrintCompositor, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<PrintCompositor> {
    let f: &F = &*(f as *const F);
    f(&PrintCompositor::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_print_header_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_source_sys::GtkSourcePrintCompositor, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<PrintCompositor> {
    let f: &F = &*(f as *const F);
    f(&PrintCompositor::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_print_line_numbers_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_source_sys::GtkSourcePrintCompositor, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<PrintCompositor> {
    let f: &F = &*(f as *const F);
    f(&PrintCompositor::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_tab_width_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_source_sys::GtkSourcePrintCompositor, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<PrintCompositor> {
    let f: &F = &*(f as *const F);
    f(&PrintCompositor::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_wrap_mode_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_source_sys::GtkSourcePrintCompositor, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<PrintCompositor> {
    let f: &F = &*(f as *const F);
    f(&PrintCompositor::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for PrintCompositor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PrintCompositor")
    }
}
