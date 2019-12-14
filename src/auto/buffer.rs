// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::GString;
use glib::StaticType;
use glib::ToValue;
use glib::Value;
use glib_sys;
use gobject_sys;
use gtk;
use gtk_source_sys;
use gtk_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
#[cfg(any(feature = "v3_12", feature = "dox"))]
use ChangeCaseType;
use Language;
use Mark;
#[cfg(any(feature = "v3_18", feature = "dox"))]
use SortFlags;
use StyleScheme;
use UndoManager;

glib_wrapper! {
    pub struct Buffer(Object<gtk_source_sys::GtkSourceBuffer, gtk_source_sys::GtkSourceBufferClass, BufferClass>) @extends gtk::TextBuffer;

    match fn {
        get_type => || gtk_source_sys::gtk_source_buffer_get_type(),
    }
}

impl Buffer {
    pub fn new<P: IsA<gtk::TextTagTable>>(table: Option<&P>) -> Buffer {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(gtk_source_sys::gtk_source_buffer_new(
                table.map(|p| p.as_ref()).to_glib_none().0,
            ))
        }
    }

    pub fn new_with_language<P: IsA<Language>>(language: &P) -> Buffer {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(gtk_source_sys::gtk_source_buffer_new_with_language(
                language.as_ref().to_glib_none().0,
            ))
        }
    }
}

#[derive(Clone, Default)]
pub struct BufferBuilder {
    highlight_matching_brackets: Option<bool>,
    highlight_syntax: Option<bool>,
    #[cfg(any(feature = "v3_14", feature = "dox"))]
    implicit_trailing_newline: Option<bool>,
    language: Option<Language>,
    max_undo_levels: Option<i32>,
    style_scheme: Option<StyleScheme>,
    undo_manager: Option<UndoManager>,
    tag_table: Option<gtk::TextTagTable>,
    text: Option<String>,
}

impl BufferBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> Buffer {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref highlight_matching_brackets) = self.highlight_matching_brackets {
            properties.push(("highlight-matching-brackets", highlight_matching_brackets));
        }
        if let Some(ref highlight_syntax) = self.highlight_syntax {
            properties.push(("highlight-syntax", highlight_syntax));
        }
        #[cfg(any(feature = "v3_14", feature = "dox"))]
        {
            if let Some(ref implicit_trailing_newline) = self.implicit_trailing_newline {
                properties.push(("implicit-trailing-newline", implicit_trailing_newline));
            }
        }
        if let Some(ref language) = self.language {
            properties.push(("language", language));
        }
        if let Some(ref max_undo_levels) = self.max_undo_levels {
            properties.push(("max-undo-levels", max_undo_levels));
        }
        if let Some(ref style_scheme) = self.style_scheme {
            properties.push(("style-scheme", style_scheme));
        }
        if let Some(ref undo_manager) = self.undo_manager {
            properties.push(("undo-manager", undo_manager));
        }
        if let Some(ref tag_table) = self.tag_table {
            properties.push(("tag-table", tag_table));
        }
        if let Some(ref text) = self.text {
            properties.push(("text", text));
        }
        glib::Object::new(Buffer::static_type(), &properties)
            .expect("object new")
            .downcast()
            .expect("downcast")
    }

    pub fn highlight_matching_brackets(mut self, highlight_matching_brackets: bool) -> Self {
        self.highlight_matching_brackets = Some(highlight_matching_brackets);
        self
    }

    pub fn highlight_syntax(mut self, highlight_syntax: bool) -> Self {
        self.highlight_syntax = Some(highlight_syntax);
        self
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    pub fn implicit_trailing_newline(mut self, implicit_trailing_newline: bool) -> Self {
        self.implicit_trailing_newline = Some(implicit_trailing_newline);
        self
    }

    pub fn language<P: IsA<Language>>(mut self, language: &P) -> Self {
        self.language = Some(language.clone().upcast());
        self
    }

    pub fn max_undo_levels(mut self, max_undo_levels: i32) -> Self {
        self.max_undo_levels = Some(max_undo_levels);
        self
    }

    pub fn style_scheme<P: IsA<StyleScheme>>(mut self, style_scheme: &P) -> Self {
        self.style_scheme = Some(style_scheme.clone().upcast());
        self
    }

    pub fn undo_manager<P: IsA<UndoManager>>(mut self, undo_manager: &P) -> Self {
        self.undo_manager = Some(undo_manager.clone().upcast());
        self
    }

    pub fn tag_table<P: IsA<gtk::TextTagTable>>(mut self, tag_table: &P) -> Self {
        self.tag_table = Some(tag_table.clone().upcast());
        self
    }

    pub fn text(mut self, text: &str) -> Self {
        self.text = Some(text.to_string());
        self
    }
}

pub const NONE_BUFFER: Option<&Buffer> = None;

pub trait BufferExt: 'static {
    fn backward_iter_to_source_mark(
        &self,
        iter: &mut gtk::TextIter,
        category: Option<&str>,
    ) -> bool;

    fn begin_not_undoable_action(&self);

    fn can_redo(&self) -> bool;

    fn can_undo(&self) -> bool;

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn change_case(
        &self,
        case_type: ChangeCaseType,
        start: &mut gtk::TextIter,
        end: &mut gtk::TextIter,
    );

    fn create_source_mark(
        &self,
        name: Option<&str>,
        category: &str,
        where_: &gtk::TextIter,
    ) -> Option<Mark>;

    //#[cfg(any(feature = "v3_20", feature = "dox"))]
    //fn create_source_tag(&self, tag_name: Option<&str>, first_property_name: Option<&str>, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> Option<gtk::TextTag>;

    fn end_not_undoable_action(&self);

    fn ensure_highlight(&self, start: &gtk::TextIter, end: &gtk::TextIter);

    fn forward_iter_to_source_mark(&self, iter: &mut gtk::TextIter, category: Option<&str>)
        -> bool;

    fn get_context_classes_at_iter(&self, iter: &gtk::TextIter) -> Vec<GString>;

    fn get_highlight_matching_brackets(&self) -> bool;

    fn get_highlight_syntax(&self) -> bool;

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn get_implicit_trailing_newline(&self) -> bool;

    fn get_language(&self) -> Option<Language>;

    fn get_max_undo_levels(&self) -> i32;

    fn get_source_marks_at_iter(
        &self,
        iter: &mut gtk::TextIter,
        category: Option<&str>,
    ) -> Vec<Mark>;

    fn get_source_marks_at_line(&self, line: i32, category: Option<&str>) -> Vec<Mark>;

    fn get_style_scheme(&self) -> Option<StyleScheme>;

    fn get_undo_manager(&self) -> Option<UndoManager>;

    fn iter_backward_to_context_class_toggle(
        &self,
        iter: &mut gtk::TextIter,
        context_class: &str,
    ) -> bool;

    fn iter_forward_to_context_class_toggle(
        &self,
        iter: &mut gtk::TextIter,
        context_class: &str,
    ) -> bool;

    fn iter_has_context_class(&self, iter: &gtk::TextIter, context_class: &str) -> bool;

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn join_lines(&self, start: &mut gtk::TextIter, end: &mut gtk::TextIter);

    fn redo(&self);

    fn remove_source_marks(
        &self,
        start: &gtk::TextIter,
        end: &gtk::TextIter,
        category: Option<&str>,
    );

    fn set_highlight_matching_brackets(&self, highlight: bool);

    fn set_highlight_syntax(&self, highlight: bool);

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn set_implicit_trailing_newline(&self, implicit_trailing_newline: bool);

    fn set_language<P: IsA<Language>>(&self, language: Option<&P>);

    fn set_max_undo_levels(&self, max_undo_levels: i32);

    fn set_style_scheme<P: IsA<StyleScheme>>(&self, scheme: Option<&P>);

    fn set_undo_manager<P: IsA<UndoManager>>(&self, manager: Option<&P>);

    #[cfg(any(feature = "v3_18", feature = "dox"))]
    fn sort_lines(
        &self,
        start: &mut gtk::TextIter,
        end: &mut gtk::TextIter,
        flags: SortFlags,
        column: i32,
    );

    fn undo(&self);

    fn get_property_can_redo(&self) -> bool;

    fn get_property_can_undo(&self) -> bool;

    //fn connect_bracket_matched<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId;

    fn connect_highlight_updated<F: Fn(&Self, &gtk::TextIter, &gtk::TextIter) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_redo<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_source_mark_updated<F: Fn(&Self, &gtk::TextMark) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_undo<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_can_redo_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_can_undo_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_highlight_matching_brackets_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_highlight_syntax_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn connect_property_implicit_trailing_newline_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_language_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_max_undo_levels_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_style_scheme_notify<F: Fn(&Self) + 'static>(&self, f: F)
        -> SignalHandlerId;

    fn connect_property_undo_manager_notify<F: Fn(&Self) + 'static>(&self, f: F)
        -> SignalHandlerId;
}

impl<O: IsA<Buffer>> BufferExt for O {
    fn backward_iter_to_source_mark(
        &self,
        iter: &mut gtk::TextIter,
        category: Option<&str>,
    ) -> bool {
        unsafe {
            from_glib(
                gtk_source_sys::gtk_source_buffer_backward_iter_to_source_mark(
                    self.as_ref().to_glib_none().0,
                    iter.to_glib_none_mut().0,
                    category.to_glib_none().0,
                ),
            )
        }
    }

    fn begin_not_undoable_action(&self) {
        unsafe {
            gtk_source_sys::gtk_source_buffer_begin_not_undoable_action(
                self.as_ref().to_glib_none().0,
            );
        }
    }

    fn can_redo(&self) -> bool {
        unsafe {
            from_glib(gtk_source_sys::gtk_source_buffer_can_redo(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn can_undo(&self) -> bool {
        unsafe {
            from_glib(gtk_source_sys::gtk_source_buffer_can_undo(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn change_case(
        &self,
        case_type: ChangeCaseType,
        start: &mut gtk::TextIter,
        end: &mut gtk::TextIter,
    ) {
        unsafe {
            gtk_source_sys::gtk_source_buffer_change_case(
                self.as_ref().to_glib_none().0,
                case_type.to_glib(),
                start.to_glib_none_mut().0,
                end.to_glib_none_mut().0,
            );
        }
    }

    fn create_source_mark(
        &self,
        name: Option<&str>,
        category: &str,
        where_: &gtk::TextIter,
    ) -> Option<Mark> {
        unsafe {
            from_glib_none(gtk_source_sys::gtk_source_buffer_create_source_mark(
                self.as_ref().to_glib_none().0,
                name.to_glib_none().0,
                category.to_glib_none().0,
                where_.to_glib_none().0,
            ))
        }
    }

    //#[cfg(any(feature = "v3_20", feature = "dox"))]
    //fn create_source_tag(&self, tag_name: Option<&str>, first_property_name: Option<&str>, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> Option<gtk::TextTag> {
    //    unsafe { TODO: call gtk_source_sys:gtk_source_buffer_create_source_tag() }
    //}

    fn end_not_undoable_action(&self) {
        unsafe {
            gtk_source_sys::gtk_source_buffer_end_not_undoable_action(
                self.as_ref().to_glib_none().0,
            );
        }
    }

    fn ensure_highlight(&self, start: &gtk::TextIter, end: &gtk::TextIter) {
        unsafe {
            gtk_source_sys::gtk_source_buffer_ensure_highlight(
                self.as_ref().to_glib_none().0,
                start.to_glib_none().0,
                end.to_glib_none().0,
            );
        }
    }

    fn forward_iter_to_source_mark(
        &self,
        iter: &mut gtk::TextIter,
        category: Option<&str>,
    ) -> bool {
        unsafe {
            from_glib(
                gtk_source_sys::gtk_source_buffer_forward_iter_to_source_mark(
                    self.as_ref().to_glib_none().0,
                    iter.to_glib_none_mut().0,
                    category.to_glib_none().0,
                ),
            )
        }
    }

    fn get_context_classes_at_iter(&self, iter: &gtk::TextIter) -> Vec<GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(
                gtk_source_sys::gtk_source_buffer_get_context_classes_at_iter(
                    self.as_ref().to_glib_none().0,
                    iter.to_glib_none().0,
                ),
            )
        }
    }

    fn get_highlight_matching_brackets(&self) -> bool {
        unsafe {
            from_glib(
                gtk_source_sys::gtk_source_buffer_get_highlight_matching_brackets(
                    self.as_ref().to_glib_none().0,
                ),
            )
        }
    }

    fn get_highlight_syntax(&self) -> bool {
        unsafe {
            from_glib(gtk_source_sys::gtk_source_buffer_get_highlight_syntax(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn get_implicit_trailing_newline(&self) -> bool {
        unsafe {
            from_glib(
                gtk_source_sys::gtk_source_buffer_get_implicit_trailing_newline(
                    self.as_ref().to_glib_none().0,
                ),
            )
        }
    }

    fn get_language(&self) -> Option<Language> {
        unsafe {
            from_glib_none(gtk_source_sys::gtk_source_buffer_get_language(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_max_undo_levels(&self) -> i32 {
        unsafe {
            gtk_source_sys::gtk_source_buffer_get_max_undo_levels(self.as_ref().to_glib_none().0)
        }
    }

    fn get_source_marks_at_iter(
        &self,
        iter: &mut gtk::TextIter,
        category: Option<&str>,
    ) -> Vec<Mark> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(
                gtk_source_sys::gtk_source_buffer_get_source_marks_at_iter(
                    self.as_ref().to_glib_none().0,
                    iter.to_glib_none_mut().0,
                    category.to_glib_none().0,
                ),
            )
        }
    }

    fn get_source_marks_at_line(&self, line: i32, category: Option<&str>) -> Vec<Mark> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(
                gtk_source_sys::gtk_source_buffer_get_source_marks_at_line(
                    self.as_ref().to_glib_none().0,
                    line,
                    category.to_glib_none().0,
                ),
            )
        }
    }

    fn get_style_scheme(&self) -> Option<StyleScheme> {
        unsafe {
            from_glib_none(gtk_source_sys::gtk_source_buffer_get_style_scheme(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_undo_manager(&self) -> Option<UndoManager> {
        unsafe {
            from_glib_none(gtk_source_sys::gtk_source_buffer_get_undo_manager(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn iter_backward_to_context_class_toggle(
        &self,
        iter: &mut gtk::TextIter,
        context_class: &str,
    ) -> bool {
        unsafe {
            from_glib(
                gtk_source_sys::gtk_source_buffer_iter_backward_to_context_class_toggle(
                    self.as_ref().to_glib_none().0,
                    iter.to_glib_none_mut().0,
                    context_class.to_glib_none().0,
                ),
            )
        }
    }

    fn iter_forward_to_context_class_toggle(
        &self,
        iter: &mut gtk::TextIter,
        context_class: &str,
    ) -> bool {
        unsafe {
            from_glib(
                gtk_source_sys::gtk_source_buffer_iter_forward_to_context_class_toggle(
                    self.as_ref().to_glib_none().0,
                    iter.to_glib_none_mut().0,
                    context_class.to_glib_none().0,
                ),
            )
        }
    }

    fn iter_has_context_class(&self, iter: &gtk::TextIter, context_class: &str) -> bool {
        unsafe {
            from_glib(gtk_source_sys::gtk_source_buffer_iter_has_context_class(
                self.as_ref().to_glib_none().0,
                iter.to_glib_none().0,
                context_class.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn join_lines(&self, start: &mut gtk::TextIter, end: &mut gtk::TextIter) {
        unsafe {
            gtk_source_sys::gtk_source_buffer_join_lines(
                self.as_ref().to_glib_none().0,
                start.to_glib_none_mut().0,
                end.to_glib_none_mut().0,
            );
        }
    }

    fn redo(&self) {
        unsafe {
            gtk_source_sys::gtk_source_buffer_redo(self.as_ref().to_glib_none().0);
        }
    }

    fn remove_source_marks(
        &self,
        start: &gtk::TextIter,
        end: &gtk::TextIter,
        category: Option<&str>,
    ) {
        unsafe {
            gtk_source_sys::gtk_source_buffer_remove_source_marks(
                self.as_ref().to_glib_none().0,
                start.to_glib_none().0,
                end.to_glib_none().0,
                category.to_glib_none().0,
            );
        }
    }

    fn set_highlight_matching_brackets(&self, highlight: bool) {
        unsafe {
            gtk_source_sys::gtk_source_buffer_set_highlight_matching_brackets(
                self.as_ref().to_glib_none().0,
                highlight.to_glib(),
            );
        }
    }

    fn set_highlight_syntax(&self, highlight: bool) {
        unsafe {
            gtk_source_sys::gtk_source_buffer_set_highlight_syntax(
                self.as_ref().to_glib_none().0,
                highlight.to_glib(),
            );
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn set_implicit_trailing_newline(&self, implicit_trailing_newline: bool) {
        unsafe {
            gtk_source_sys::gtk_source_buffer_set_implicit_trailing_newline(
                self.as_ref().to_glib_none().0,
                implicit_trailing_newline.to_glib(),
            );
        }
    }

    fn set_language<P: IsA<Language>>(&self, language: Option<&P>) {
        unsafe {
            gtk_source_sys::gtk_source_buffer_set_language(
                self.as_ref().to_glib_none().0,
                language.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    fn set_max_undo_levels(&self, max_undo_levels: i32) {
        unsafe {
            gtk_source_sys::gtk_source_buffer_set_max_undo_levels(
                self.as_ref().to_glib_none().0,
                max_undo_levels,
            );
        }
    }

    fn set_style_scheme<P: IsA<StyleScheme>>(&self, scheme: Option<&P>) {
        unsafe {
            gtk_source_sys::gtk_source_buffer_set_style_scheme(
                self.as_ref().to_glib_none().0,
                scheme.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    fn set_undo_manager<P: IsA<UndoManager>>(&self, manager: Option<&P>) {
        unsafe {
            gtk_source_sys::gtk_source_buffer_set_undo_manager(
                self.as_ref().to_glib_none().0,
                manager.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v3_18", feature = "dox"))]
    fn sort_lines(
        &self,
        start: &mut gtk::TextIter,
        end: &mut gtk::TextIter,
        flags: SortFlags,
        column: i32,
    ) {
        unsafe {
            gtk_source_sys::gtk_source_buffer_sort_lines(
                self.as_ref().to_glib_none().0,
                start.to_glib_none_mut().0,
                end.to_glib_none_mut().0,
                flags.to_glib(),
                column,
            );
        }
    }

    fn undo(&self) {
        unsafe {
            gtk_source_sys::gtk_source_buffer_undo(self.as_ref().to_glib_none().0);
        }
    }

    fn get_property_can_redo(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"can-redo\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `can-redo` getter")
                .unwrap()
        }
    }

    fn get_property_can_undo(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"can-undo\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `can-undo` getter")
                .unwrap()
        }
    }

    //fn connect_bracket_matched<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Ignored state: GtkSource.BracketMatchType
    //}

    fn connect_highlight_updated<F: Fn(&Self, &gtk::TextIter, &gtk::TextIter) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn highlight_updated_trampoline<
            P,
            F: Fn(&P, &gtk::TextIter, &gtk::TextIter) + 'static,
        >(
            this: *mut gtk_source_sys::GtkSourceBuffer,
            start: *mut gtk_sys::GtkTextIter,
            end: *mut gtk_sys::GtkTextIter,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Buffer>,
        {
            let f: &F = &*(f as *const F);
            f(
                &Buffer::from_glib_borrow(this).unsafe_cast(),
                &from_glib_borrow(start),
                &from_glib_borrow(end),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"highlight-updated\0".as_ptr() as *const _,
                Some(transmute(highlight_updated_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_redo<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn redo_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_source_sys::GtkSourceBuffer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Buffer>,
        {
            let f: &F = &*(f as *const F);
            f(&Buffer::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"redo\0".as_ptr() as *const _,
                Some(transmute(redo_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_source_mark_updated<F: Fn(&Self, &gtk::TextMark) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn source_mark_updated_trampoline<
            P,
            F: Fn(&P, &gtk::TextMark) + 'static,
        >(
            this: *mut gtk_source_sys::GtkSourceBuffer,
            mark: *mut gtk_sys::GtkTextMark,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Buffer>,
        {
            let f: &F = &*(f as *const F);
            f(
                &Buffer::from_glib_borrow(this).unsafe_cast(),
                &from_glib_borrow(mark),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"source-mark-updated\0".as_ptr() as *const _,
                Some(transmute(
                    source_mark_updated_trampoline::<Self, F> as usize,
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_undo<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn undo_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_source_sys::GtkSourceBuffer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Buffer>,
        {
            let f: &F = &*(f as *const F);
            f(&Buffer::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"undo\0".as_ptr() as *const _,
                Some(transmute(undo_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_can_redo_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_can_redo_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_source_sys::GtkSourceBuffer,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Buffer>,
        {
            let f: &F = &*(f as *const F);
            f(&Buffer::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::can-redo\0".as_ptr() as *const _,
                Some(transmute(notify_can_redo_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_can_undo_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_can_undo_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_source_sys::GtkSourceBuffer,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Buffer>,
        {
            let f: &F = &*(f as *const F);
            f(&Buffer::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::can-undo\0".as_ptr() as *const _,
                Some(transmute(notify_can_undo_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_highlight_matching_brackets_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_highlight_matching_brackets_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_source_sys::GtkSourceBuffer,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Buffer>,
        {
            let f: &F = &*(f as *const F);
            f(&Buffer::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::highlight-matching-brackets\0".as_ptr() as *const _,
                Some(transmute(
                    notify_highlight_matching_brackets_trampoline::<Self, F> as usize,
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_highlight_syntax_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_highlight_syntax_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_source_sys::GtkSourceBuffer,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Buffer>,
        {
            let f: &F = &*(f as *const F);
            f(&Buffer::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::highlight-syntax\0".as_ptr() as *const _,
                Some(transmute(
                    notify_highlight_syntax_trampoline::<Self, F> as usize,
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn connect_property_implicit_trailing_newline_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_implicit_trailing_newline_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_source_sys::GtkSourceBuffer,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Buffer>,
        {
            let f: &F = &*(f as *const F);
            f(&Buffer::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::implicit-trailing-newline\0".as_ptr() as *const _,
                Some(transmute(
                    notify_implicit_trailing_newline_trampoline::<Self, F> as usize,
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_language_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_language_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_source_sys::GtkSourceBuffer,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Buffer>,
        {
            let f: &F = &*(f as *const F);
            f(&Buffer::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::language\0".as_ptr() as *const _,
                Some(transmute(notify_language_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_max_undo_levels_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_max_undo_levels_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_source_sys::GtkSourceBuffer,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Buffer>,
        {
            let f: &F = &*(f as *const F);
            f(&Buffer::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::max-undo-levels\0".as_ptr() as *const _,
                Some(transmute(
                    notify_max_undo_levels_trampoline::<Self, F> as usize,
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_style_scheme_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_style_scheme_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_source_sys::GtkSourceBuffer,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Buffer>,
        {
            let f: &F = &*(f as *const F);
            f(&Buffer::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::style-scheme\0".as_ptr() as *const _,
                Some(transmute(
                    notify_style_scheme_trampoline::<Self, F> as usize,
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_undo_manager_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_undo_manager_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_source_sys::GtkSourceBuffer,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Buffer>,
        {
            let f: &F = &*(f as *const F);
            f(&Buffer::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::undo-manager\0".as_ptr() as *const _,
                Some(transmute(
                    notify_undo_manager_trampoline::<Self, F> as usize,
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Buffer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Buffer")
    }
}
