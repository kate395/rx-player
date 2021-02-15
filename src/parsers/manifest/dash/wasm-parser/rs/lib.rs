extern crate core;
extern crate quick_xml;

mod mpd_reader;
mod event_enums;
mod error;
mod parse;
mod report;
mod utils;

use std::io::BufReader;
use event_enums::*;
use mpd_reader::MPDReader;

#[no_mangle]
extern "C" {
    /// Called each time a new known tag is encountered in the MPD
    /// @see TagName
    fn onTagOpen(tag_name : TagName);

    fn onTagClose(tag_name : TagName);

    /// Called when a new attribute has been parsed.
    fn onAttribute(
        // Integer identifying the attribute (linked to a specific type)
        attr_name : AttributeName,

        // Pointer to beginning of data in wasm memory
        ptr: *const u8,

        // Length of data, in bytes
        len: usize);

    fn onCustomEvent(
        evt_type : CustomEventType,

        // Pointer to beginning of data in wasm memory
        ptr: *const u8,

        // Length of data, in bytes
        len: usize);

    fn read_next(ptr : *const u8,  size : usize) -> usize;
}

#[no_mangle]
pub extern "C" fn parse() {
    let mpd_reader = MPDReader {};
    let mut buf_read = BufReader::new(mpd_reader);
    parse::process_mpd(&mut buf_read);
    // let buf = Vec::with_capacity(READING_VEC_CAPACITY);
    // parse::process_mpd(&mut mpd_reader);
}
