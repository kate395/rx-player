extern crate core;
extern crate quick_xml;

mod events;
mod errors;
mod processor;
mod reader;
mod reportable;
mod utils;

pub use errors::{ParsingError, Result};

use std::io::BufReader;
use events::*;
use processor::MPDProcessor;
use reader::MPDReader;

#[no_mangle]
extern "C" {
    /// Called each time a new known tag is encountered in the MPD.
    /// The `tag_name` corresponds to the value of the TagName enum (@see
    /// events), casted as a single byte.
    fn onTagOpen(tag_name : u8);

    /// Called each time a previously-opened known tag is encountered in the MPD
    /// now closed.
    /// The `tag_name` corresponds to the value of the TagName enum (@see
    /// events), casted as a single byte.
    fn onTagClose(tag_name : u8);

    /// Called when a new attribute has been parsed.
    fn onAttribute(
        // Integer identifying the attribute (linked to a specific type)
        attr_name : u8,

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
    let buf_read = BufReader::new(MPDReader {});
    let mut processor = MPDProcessor::new(buf_read);
    processor.process_mpd_from_root();
}
