// This file was generated by gir (https://github.com/gtk-rs/gir)
// from /usr/share/gir-1.0
// from ../gir-1.0
// DO NOT EDIT

use glib::{prelude::*, translate::*};

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "WpConstraintType")]
pub enum ConstraintType {
    #[doc(alias = "WP_CONSTRAINT_TYPE_NONE")]
    None,
    #[doc(alias = "WP_CONSTRAINT_TYPE_PW_GLOBAL_PROPERTY")]
    PwGlobalProperty,
    #[doc(alias = "WP_CONSTRAINT_TYPE_PW_PROPERTY")]
    PwProperty,
    #[doc(alias = "WP_CONSTRAINT_TYPE_G_PROPERTY")]
    GProperty,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl IntoGlib for ConstraintType {
    type GlibType = ffi::WpConstraintType;

    #[inline]
    fn into_glib(self) -> ffi::WpConstraintType {
        match self {
            Self::None => ffi::WP_CONSTRAINT_TYPE_NONE,
            Self::PwGlobalProperty => ffi::WP_CONSTRAINT_TYPE_PW_GLOBAL_PROPERTY,
            Self::PwProperty => ffi::WP_CONSTRAINT_TYPE_PW_PROPERTY,
            Self::GProperty => ffi::WP_CONSTRAINT_TYPE_G_PROPERTY,
            Self::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::WpConstraintType> for ConstraintType {
    #[inline]
    unsafe fn from_glib(value: ffi::WpConstraintType) -> Self {
        skip_assert_initialized!();

        match value {
            ffi::WP_CONSTRAINT_TYPE_NONE => Self::None,
            ffi::WP_CONSTRAINT_TYPE_PW_GLOBAL_PROPERTY => Self::PwGlobalProperty,
            ffi::WP_CONSTRAINT_TYPE_PW_PROPERTY => Self::PwProperty,
            ffi::WP_CONSTRAINT_TYPE_G_PROPERTY => Self::GProperty,
            value => Self::__Unknown(value),
        }
    }
}

impl StaticType for ConstraintType {
    #[inline]
    #[doc(alias = "wp_constraint_type_get_type")]
    fn static_type() -> glib::Type {
        unsafe { from_glib(ffi::wp_constraint_type_get_type()) }
    }
}

impl glib::HasParamSpec for ConstraintType {
    type ParamSpec = glib::ParamSpecEnum;
    type SetValue = Self;
    type BuilderFn = fn(&str, Self) -> glib::ParamSpecEnumBuilder<Self>;

    fn param_spec_builder() -> Self::BuilderFn {
        Self::ParamSpec::builder_with_default
    }
}

impl glib::value::ValueType for ConstraintType {
    type Type = Self;
}

unsafe impl<'a> glib::value::FromValue<'a> for ConstraintType {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for ConstraintType {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl From<ConstraintType> for glib::Value {
    #[inline]
    fn from(v: ConstraintType) -> Self {
        skip_assert_initialized!();
        ToValue::to_value(&v)
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "WpConstraintVerb")]
pub enum ConstraintVerb {
    #[doc(alias = "WP_CONSTRAINT_VERB_EQUALS")]
    Equals,
    #[doc(alias = "WP_CONSTRAINT_VERB_NOT_EQUALS")]
    NotEquals,
    #[doc(alias = "WP_CONSTRAINT_VERB_IN_LIST")]
    InList,
    #[doc(alias = "WP_CONSTRAINT_VERB_IN_RANGE")]
    InRange,
    #[doc(alias = "WP_CONSTRAINT_VERB_MATCHES")]
    Matches,
    #[doc(alias = "WP_CONSTRAINT_VERB_IS_PRESENT")]
    IsPresent,
    #[doc(alias = "WP_CONSTRAINT_VERB_IS_ABSENT")]
    IsAbsent,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl IntoGlib for ConstraintVerb {
    type GlibType = ffi::WpConstraintVerb;

    #[inline]
    fn into_glib(self) -> ffi::WpConstraintVerb {
        match self {
            Self::Equals => ffi::WP_CONSTRAINT_VERB_EQUALS,
            Self::NotEquals => ffi::WP_CONSTRAINT_VERB_NOT_EQUALS,
            Self::InList => ffi::WP_CONSTRAINT_VERB_IN_LIST,
            Self::InRange => ffi::WP_CONSTRAINT_VERB_IN_RANGE,
            Self::Matches => ffi::WP_CONSTRAINT_VERB_MATCHES,
            Self::IsPresent => ffi::WP_CONSTRAINT_VERB_IS_PRESENT,
            Self::IsAbsent => ffi::WP_CONSTRAINT_VERB_IS_ABSENT,
            Self::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::WpConstraintVerb> for ConstraintVerb {
    #[inline]
    unsafe fn from_glib(value: ffi::WpConstraintVerb) -> Self {
        skip_assert_initialized!();

        match value {
            ffi::WP_CONSTRAINT_VERB_EQUALS => Self::Equals,
            ffi::WP_CONSTRAINT_VERB_NOT_EQUALS => Self::NotEquals,
            ffi::WP_CONSTRAINT_VERB_IN_LIST => Self::InList,
            ffi::WP_CONSTRAINT_VERB_IN_RANGE => Self::InRange,
            ffi::WP_CONSTRAINT_VERB_MATCHES => Self::Matches,
            ffi::WP_CONSTRAINT_VERB_IS_PRESENT => Self::IsPresent,
            ffi::WP_CONSTRAINT_VERB_IS_ABSENT => Self::IsAbsent,
            value => Self::__Unknown(value),
        }
    }
}

impl StaticType for ConstraintVerb {
    #[inline]
    #[doc(alias = "wp_constraint_verb_get_type")]
    fn static_type() -> glib::Type {
        unsafe { from_glib(ffi::wp_constraint_verb_get_type()) }
    }
}

impl glib::HasParamSpec for ConstraintVerb {
    type ParamSpec = glib::ParamSpecEnum;
    type SetValue = Self;
    type BuilderFn = fn(&str, Self) -> glib::ParamSpecEnumBuilder<Self>;

    fn param_spec_builder() -> Self::BuilderFn {
        Self::ParamSpec::builder_with_default
    }
}

impl glib::value::ValueType for ConstraintVerb {
    type Type = Self;
}

unsafe impl<'a> glib::value::FromValue<'a> for ConstraintVerb {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for ConstraintVerb {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl From<ConstraintVerb> for glib::Value {
    #[inline]
    fn from(v: ConstraintVerb) -> Self {
        skip_assert_initialized!();
        ToValue::to_value(&v)
    }
}
