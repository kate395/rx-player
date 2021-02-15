use core::mem;
use super::{
    AttributeName,
    TagName,
    onAttribute,
    onTagClose,
    onTagOpen,
};

pub trait ReportableValue {
    fn report_as_attr(&self, attr_name: AttributeName);
}

// Note: I'm not "impl"ing ReportableValue generically to have more control over
// which variants are actually called.
// There should only be few ways `report_as_attr` can be called, those few impl
// blocks ensure of that.

impl ReportableValue for bool {
    #[inline(always)]
    fn report_as_attr(&self, attr_name: AttributeName) {
        unsafe { onAttribute(attr_name, mem::transmute(self), 8); };
    }
}

impl ReportableValue for f64 {
    #[inline(always)]
    fn report_as_attr(&self, attr_name: AttributeName) {
        unsafe { onAttribute(attr_name, mem::transmute(self), 8); };
    }
}

// impl ReportableValue for &[u8] {
//     #[inline(always)]
//     fn report_as_attr(&self, attr_name: AttributeName) {
//         unsafe { onAttribute(attr_name, mem::transmute(self), 8); };
//     }
// }

impl ReportableValue for super::parse::SElement {
    #[inline(always)]
    fn report_as_attr(&self, attr_name: AttributeName) {
        unsafe {
            onAttribute(
                attr_name,
                self as *const super::parse::SElement as *const u8,
                mem::size_of::<super::parse::SElement>());
        }
    }
}

impl<'a> ReportableValue for std::borrow::Cow<'a, [u8]> {
    #[inline(always)]
    fn report_as_attr(&self, attr_name: AttributeName) {
        unsafe { onAttribute(attr_name, mem::transmute(self.as_ptr()), self.len()); };
    }
}

#[inline(always)]
pub fn report_opening_tag(tag_name : TagName) {
    unsafe { onTagOpen(tag_name) };
}

#[inline(always)]
pub fn report_closing_tag(tag_name : TagName) {
    unsafe { onTagClose(tag_name) };
}
