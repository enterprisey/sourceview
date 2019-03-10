// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::StaticType;
use glib::Type;
use glib::translate::*;
use glib::value::FromValue;
use glib::value::FromValueOptional;
use glib::value::SetValue;
use glib::value::Value;
use gobject_sys;
use gtk_source_sys;

bitflags! {
    pub struct CompletionActivation: u32 {
        const NONE = 0;
        const INTERACTIVE = 1;
        const USER_REQUESTED = 2;
    }
}

#[doc(hidden)]
impl ToGlib for CompletionActivation {
    type GlibType = gtk_source_sys::GtkSourceCompletionActivation;

    fn to_glib(&self) -> gtk_source_sys::GtkSourceCompletionActivation {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<gtk_source_sys::GtkSourceCompletionActivation> for CompletionActivation {
    fn from_glib(value: gtk_source_sys::GtkSourceCompletionActivation) -> CompletionActivation {
        skip_assert_initialized!();
        CompletionActivation::from_bits_truncate(value)
    }
}

impl StaticType for CompletionActivation {
    fn static_type() -> Type {
        unsafe { from_glib(gtk_source_sys::gtk_source_completion_activation_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for CompletionActivation {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for CompletionActivation {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for CompletionActivation {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[cfg_attr(feature = "v3_24", deprecated)]
bitflags! {
    pub struct DrawSpacesFlags: u32 {
        const SPACE = 1;
        const TAB = 2;
        const NEWLINE = 4;
        const NBSP = 8;
        const LEADING = 16;
        const TEXT = 32;
        const TRAILING = 64;
        const ALL = 127;
    }
}

#[cfg_attr(feature = "v3_24", deprecated)]
#[doc(hidden)]
impl ToGlib for DrawSpacesFlags {
    type GlibType = gtk_source_sys::GtkSourceDrawSpacesFlags;

    fn to_glib(&self) -> gtk_source_sys::GtkSourceDrawSpacesFlags {
        self.bits()
    }
}

#[cfg_attr(feature = "v3_24", deprecated)]
#[doc(hidden)]
impl FromGlib<gtk_source_sys::GtkSourceDrawSpacesFlags> for DrawSpacesFlags {
    fn from_glib(value: gtk_source_sys::GtkSourceDrawSpacesFlags) -> DrawSpacesFlags {
        skip_assert_initialized!();
        DrawSpacesFlags::from_bits_truncate(value)
    }
}

#[cfg_attr(feature = "v3_24", deprecated)]
impl StaticType for DrawSpacesFlags {
    fn static_type() -> Type {
        unsafe { from_glib(gtk_source_sys::gtk_source_draw_spaces_flags_get_type()) }
    }
}

#[cfg_attr(feature = "v3_24", deprecated)]
impl<'a> FromValueOptional<'a> for DrawSpacesFlags {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

#[cfg_attr(feature = "v3_24", deprecated)]
impl<'a> FromValue<'a> for DrawSpacesFlags {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_flags(value.to_glib_none().0))
    }
}

#[cfg_attr(feature = "v3_24", deprecated)]
impl SetValue for DrawSpacesFlags {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[cfg(any(feature = "v3_14", feature = "dox"))]
bitflags! {
    pub struct FileSaverFlags: u32 {
        const NONE = 0;
        const IGNORE_INVALID_CHARS = 1;
        const IGNORE_MODIFICATION_TIME = 2;
        const CREATE_BACKUP = 4;
    }
}

#[cfg(any(feature = "v3_14", feature = "dox"))]
#[doc(hidden)]
impl ToGlib for FileSaverFlags {
    type GlibType = gtk_source_sys::GtkSourceFileSaverFlags;

    fn to_glib(&self) -> gtk_source_sys::GtkSourceFileSaverFlags {
        self.bits()
    }
}

#[cfg(any(feature = "v3_14", feature = "dox"))]
#[doc(hidden)]
impl FromGlib<gtk_source_sys::GtkSourceFileSaverFlags> for FileSaverFlags {
    fn from_glib(value: gtk_source_sys::GtkSourceFileSaverFlags) -> FileSaverFlags {
        skip_assert_initialized!();
        FileSaverFlags::from_bits_truncate(value)
    }
}

#[cfg(any(feature = "v3_14", feature = "dox"))]
impl StaticType for FileSaverFlags {
    fn static_type() -> Type {
        unsafe { from_glib(gtk_source_sys::gtk_source_file_saver_flags_get_type()) }
    }
}

#[cfg(any(feature = "v3_14", feature = "dox"))]
impl<'a> FromValueOptional<'a> for FileSaverFlags {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

#[cfg(any(feature = "v3_14", feature = "dox"))]
impl<'a> FromValue<'a> for FileSaverFlags {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_flags(value.to_glib_none().0))
    }
}

#[cfg(any(feature = "v3_14", feature = "dox"))]
impl SetValue for FileSaverFlags {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

bitflags! {
    pub struct GutterRendererState: u32 {
        const NORMAL = 0;
        const CURSOR = 1;
        const PRELIT = 2;
        const SELECTED = 4;
    }
}

#[doc(hidden)]
impl ToGlib for GutterRendererState {
    type GlibType = gtk_source_sys::GtkSourceGutterRendererState;

    fn to_glib(&self) -> gtk_source_sys::GtkSourceGutterRendererState {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<gtk_source_sys::GtkSourceGutterRendererState> for GutterRendererState {
    fn from_glib(value: gtk_source_sys::GtkSourceGutterRendererState) -> GutterRendererState {
        skip_assert_initialized!();
        GutterRendererState::from_bits_truncate(value)
    }
}

impl StaticType for GutterRendererState {
    fn static_type() -> Type {
        unsafe { from_glib(gtk_source_sys::gtk_source_gutter_renderer_state_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for GutterRendererState {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for GutterRendererState {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for GutterRendererState {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[cfg(any(feature = "v3_18", feature = "dox"))]
bitflags! {
    pub struct SortFlags: u32 {
        const NONE = 0;
        const CASE_SENSITIVE = 1;
        const REVERSE_ORDER = 2;
        const REMOVE_DUPLICATES = 4;
    }
}

#[cfg(any(feature = "v3_18", feature = "dox"))]
#[doc(hidden)]
impl ToGlib for SortFlags {
    type GlibType = gtk_source_sys::GtkSourceSortFlags;

    fn to_glib(&self) -> gtk_source_sys::GtkSourceSortFlags {
        self.bits()
    }
}

#[cfg(any(feature = "v3_18", feature = "dox"))]
#[doc(hidden)]
impl FromGlib<gtk_source_sys::GtkSourceSortFlags> for SortFlags {
    fn from_glib(value: gtk_source_sys::GtkSourceSortFlags) -> SortFlags {
        skip_assert_initialized!();
        SortFlags::from_bits_truncate(value)
    }
}

#[cfg(any(feature = "v3_18", feature = "dox"))]
impl StaticType for SortFlags {
    fn static_type() -> Type {
        unsafe { from_glib(gtk_source_sys::gtk_source_sort_flags_get_type()) }
    }
}

#[cfg(any(feature = "v3_18", feature = "dox"))]
impl<'a> FromValueOptional<'a> for SortFlags {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

#[cfg(any(feature = "v3_18", feature = "dox"))]
impl<'a> FromValue<'a> for SortFlags {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_flags(value.to_glib_none().0))
    }
}

#[cfg(any(feature = "v3_18", feature = "dox"))]
impl SetValue for SortFlags {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[cfg(any(feature = "v3_24", feature = "dox"))]
bitflags! {
    pub struct SpaceLocationFlags: u32 {
        const NONE = 0;
        const LEADING = 1;
        const INSIDE_TEXT = 2;
        const TRAILING = 4;
        const ALL = 7;
    }
}

#[cfg(any(feature = "v3_24", feature = "dox"))]
#[doc(hidden)]
impl ToGlib for SpaceLocationFlags {
    type GlibType = gtk_source_sys::GtkSourceSpaceLocationFlags;

    fn to_glib(&self) -> gtk_source_sys::GtkSourceSpaceLocationFlags {
        self.bits()
    }
}

#[cfg(any(feature = "v3_24", feature = "dox"))]
#[doc(hidden)]
impl FromGlib<gtk_source_sys::GtkSourceSpaceLocationFlags> for SpaceLocationFlags {
    fn from_glib(value: gtk_source_sys::GtkSourceSpaceLocationFlags) -> SpaceLocationFlags {
        skip_assert_initialized!();
        SpaceLocationFlags::from_bits_truncate(value)
    }
}

#[cfg(any(feature = "v3_24", feature = "dox"))]
impl StaticType for SpaceLocationFlags {
    fn static_type() -> Type {
        unsafe { from_glib(gtk_source_sys::gtk_source_space_location_flags_get_type()) }
    }
}

#[cfg(any(feature = "v3_24", feature = "dox"))]
impl<'a> FromValueOptional<'a> for SpaceLocationFlags {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

#[cfg(any(feature = "v3_24", feature = "dox"))]
impl<'a> FromValue<'a> for SpaceLocationFlags {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_flags(value.to_glib_none().0))
    }
}

#[cfg(any(feature = "v3_24", feature = "dox"))]
impl SetValue for SpaceLocationFlags {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[cfg(any(feature = "v3_24", feature = "dox"))]
bitflags! {
    pub struct SpaceTypeFlags: u32 {
        const NONE = 0;
        const SPACE = 1;
        const TAB = 2;
        const NEWLINE = 4;
        const NBSP = 8;
        const ALL = 15;
    }
}

#[cfg(any(feature = "v3_24", feature = "dox"))]
#[doc(hidden)]
impl ToGlib for SpaceTypeFlags {
    type GlibType = gtk_source_sys::GtkSourceSpaceTypeFlags;

    fn to_glib(&self) -> gtk_source_sys::GtkSourceSpaceTypeFlags {
        self.bits()
    }
}

#[cfg(any(feature = "v3_24", feature = "dox"))]
#[doc(hidden)]
impl FromGlib<gtk_source_sys::GtkSourceSpaceTypeFlags> for SpaceTypeFlags {
    fn from_glib(value: gtk_source_sys::GtkSourceSpaceTypeFlags) -> SpaceTypeFlags {
        skip_assert_initialized!();
        SpaceTypeFlags::from_bits_truncate(value)
    }
}

#[cfg(any(feature = "v3_24", feature = "dox"))]
impl StaticType for SpaceTypeFlags {
    fn static_type() -> Type {
        unsafe { from_glib(gtk_source_sys::gtk_source_space_type_flags_get_type()) }
    }
}

#[cfg(any(feature = "v3_24", feature = "dox"))]
impl<'a> FromValueOptional<'a> for SpaceTypeFlags {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

#[cfg(any(feature = "v3_24", feature = "dox"))]
impl<'a> FromValue<'a> for SpaceTypeFlags {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_flags(value.to_glib_none().0))
    }
}

#[cfg(any(feature = "v3_24", feature = "dox"))]
impl SetValue for SpaceTypeFlags {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

