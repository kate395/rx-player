use core::mem;
use crate::events::AttributeName;
use crate::processor::SElement;
use crate::onAttribute;

/// Trait implemented for values considered as "reportable".
/// Reportable values can be sent to JavaScript as an attribute's value, through
/// the `report_as_attr` function.
pub trait ReportableValue {
    /// Report that value as the `attr_name` AttributeName to JS.
    /// Note that calling this function will in turn call a JS callback to
    /// receive that value through a pointer to it.
    fn report_as_attr(&self, attr_name: AttributeName);
}

// Note: I'm not "impl"ing ReportableValue generically to have more control over
// which variants are actually called.
// There should only be few ways `report_as_attr` can be called, those few impl
// blocks ensure of that.

// Note 2: Most reported raw pointers are actually generated from &T (a
// reference to that type).
// I'm not completely sure that a reference will always be structurally
// (in-memory) the exact same value than a raw pointer, so this might break in
// the futue.
// However, transmuting from &T to *const _ (from an immutable reference to an
// immutable raw pointer) seems to be a VERY widespread trick.
// So even if there's a structural change in the future (e.g. reference becoming
// a pointer PLUS some added metadata), I guess/hope that they will be careful
// as to not break this trick.

impl ReportableValue for bool {
    #[inline(always)]
    fn report_as_attr(&self, attr_name: AttributeName) {
        debug_assert!(attr_name as u64 <= u8::MAX as u64);

        let val : u8 = if *self { 1 } else { 0 };
        // UNSAFE: We're using FFI, so we don't know how the pointer is used.
        // Hopefully, the JavaScript-side should clone that value synchronously.
        unsafe { onAttribute(attr_name as u8, &val, 1); };
    }
}

impl ReportableValue for f64 {
    #[inline(always)]
    fn report_as_attr(&self, attr_name: AttributeName) {
        debug_assert!(attr_name as u64 <= u8::MAX as u64);

        // UNSAFE: We're using FFI, so we don't know how the pointer is used.
        // Hopefully, the JavaScript-side should clone that value synchronously.
        //
        // Also, we're transmuting so that &f64 (reference to f64) is actually
        // treated like a *const u8 (immutable raw pointer to an u8) as it's what
        // the JS callback expects.
        // This should not matter: Rust types are not communicated to
        // JavaScript anyway.
        unsafe { onAttribute(attr_name as u8, mem::transmute(self), 8); };
    }
}

impl ReportableValue for &[SElement] {
    #[inline(always)]
    fn report_as_attr(&self, attr_name: AttributeName) {
        debug_assert!(attr_name as u64 <= u8::MAX as u64);

        // UNSAFE: We're using FFI, so we don't know how the pointer is used.
        // Hopefully, the JavaScript-side should clone that value synchronously.
        unsafe {
            onAttribute(
                attr_name as u8,
                self.as_ptr() as *const u8,
                self.len() * mem::size_of::<SElement>());
        }
    }
}

impl<'a> ReportableValue for std::borrow::Cow<'a, [u8]> {
    #[inline(always)]
    fn report_as_attr(&self, attr_name: AttributeName) {
        debug_assert!(attr_name as u64 <= u8::MAX as u64);

        // UNSAFE: We're using FFI, so we don't know how the pointer is used.
        // Hopefully, the JavaScript-side should clone that value synchronously.
        unsafe { onAttribute(attr_name as u8, self.as_ptr(), self.len()); };
    }
}
