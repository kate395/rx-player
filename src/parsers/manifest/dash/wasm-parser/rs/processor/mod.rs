use std::io::BufReader;
use quick_xml::Reader;
use quick_xml::events::Event;

mod attributes;

use crate::events::*;
use crate::errors::{ ParsingError, Result };
use crate::reader::MPDReader;
use crate::utils::parse_u64;

pub struct MPDProcessor {
    reader : quick_xml::Reader<BufReader<MPDReader>>,
    reader_buf : Vec<u8>,
    ss_buf : Vec<SElement>,
}

impl MPDProcessor {
    pub fn new(reader : BufReader<MPDReader>) -> MPDProcessor {
        let mut reader = Reader::from_reader(reader);
        reader.trim_text(true);
        MPDProcessor {
            reader,
            reader_buf: Vec::new(),
            ss_buf: Vec::new(),
        }
    }

    pub fn process_mpd_from_root(&mut self) {
        loop {
            match self.read_next_event() {
                Ok(Event::Start(tag)) if tag.name() == b"MPD" => {
                    TagName::MPD.tag_open();
                    attributes::report_mpd_attrs(&tag);
                    self.process_mpd_inner();
                },
                Ok(Event::Empty(tag)) if tag.name() == b"MPD" => {
                    TagName::MPD.tag_open();
                    attributes::report_mpd_attrs(&tag);
                    TagName::MPD.tag_close();
                },
                Ok(Event::End(tag)) if tag.name() == b"MPD" => TagName::MPD.tag_close(),
                Ok(Event::Eof) => break,
                Err(e) => ParsingError::from(e).report_err(),
                _ => (),
            }
        }
        self.reader_buf.clear();
    }

    #[inline(always)]
    fn read_next_event(&mut self) -> quick_xml::Result<quick_xml::events::Event> {
        if !self.reader_buf.is_empty() {
            self.reader_buf.clear();
        }
        self.reader.read_event(&mut self.reader_buf)
    }

    // pub fn process_mpd_from_xlink(&mut self) {
    //     self.reader_buf.clear();
    //     loop {
    //         match self.read_next_event() {
    //             Ok(Event::Start(tag)) if tag.name() == b"Period" => {
    //                 TagName::Period.tag_open();
    //                 report_period_attrs(&tag);
    //                 self.process_period_children();
    //             },
    //             Ok(Event::Empty(tag)) if tag.name() == b"Period" => {
    //                 TagName::Period.tag_open();
    //                 report_period_attrs(&tag);
    //                 TagName::Period.tag_close();
    //             },
    //             Ok(Event::End(tag)) if tag.name() == b"Period" => {
    //                 TagName::Period.tag_close();
    //             },
    //             Ok(Event::Eof) => break, // exits the loop when reaching end of file
    //             Err(e) => ParsingError::from(e).report_err(),
    //             _ => (),
    //         }
    //         self.reader_buf.clear();
    //     }
    // }

    fn process_mpd_inner(&mut self) {
        loop {
            match self.read_next_event() {
                Ok(Event::Start(tag)) => match tag.name() {
                    b"BaseURL" => {
                        TagName::BaseURL.tag_open();
                        attributes::report_base_url_attrs(&tag);
                        self.process_base_url();
                    },
                    b"Location" => self.process_location_tag(),
                    b"Period" => {
                        TagName::Period.tag_open();
                        attributes::report_period_attrs(&tag);
                        self.process_period_children();
                    },
                    b"UTCTiming" => {
                        // let _scheme = parse_scheme(&tag);
                        // XXX TODO communicate scheme
                    },
                    _ => {}
                },
                Ok(Event::Empty(tag)) => match tag.name() {
                    b"BaseURL" => {
                        TagName::BaseURL.tag_open();
                        attributes::report_base_url_attrs(&tag);
                        TagName::BaseURL.tag_close();
                    },
                    b"Period" => {
                        TagName::Period.tag_open();
                        attributes::report_period_attrs(&tag);
                        TagName::Period.tag_close();
                    },
                    b"Location" => self.process_location_tag(),
                    b"UTCTiming" => {
                        // let _scheme = parse_scheme(&tag);
                        // XXX TODO communicate scheme
                    },
                    _ => {}
                },
                Ok(Event::End(tag)) if tag.name() == b"MPD" => {
                    TagName::MPD.tag_close();
                    break;
                },
                Ok(Event::Eof) => {
                    ParsingError("Unexpected end of file in a MPD.".to_owned())
                        .report_err();
                    break;
                }
                Err(e) => ParsingError::from(e).report_err(),
                _ => (),
            }
        }
    }

    fn process_period_children(&mut self) {
        loop {
            match self.read_next_event() {
                Ok(Event::Start(tag)) => match tag.name() {
                    b"BaseURL" => {
                        TagName::BaseURL.tag_open();
                        attributes::report_base_url_attrs(&tag);
                        self.process_base_url();
                    },
                    b"AdaptationSet" => {
                        TagName::AdaptationSet.tag_open();
                        attributes::report_adaptation_set_attrs(&tag);
                        self.process_adaptation_set_children();
                    },
                    b"EventStream" => {
                        // XXX TODO EventStream
                    },
                    b"SegmentTemplate" => {
                        TagName::SegmentTemplate.tag_open();
                        self.process_segment_template_children();
                        // XXX TODO SegmentTemplate
                    },
                    _ => {}
                },
                Ok(Event::Empty(tag)) => match tag.name() {
                    b"AdaptationSet" => {
                        TagName::AdaptationSet.tag_open();
                        attributes::report_adaptation_set_attrs(&tag);
                        TagName::AdaptationSet.tag_close();
                    },
                    b"BaseURL" => {
                        TagName::BaseURL.tag_open();
                        attributes::report_base_url_attrs(&tag);
                        TagName::BaseURL.tag_close();
                    },
                    b"EventStream" => {
                        // XXX TODO EventStream
                    },
                    b"SegmentTemplate" => {
                        TagName::SegmentTemplate.tag_open();
                        TagName::SegmentTemplate.tag_close();
                        // XXX TODO SegmentTemplate
                    },
                    _ => {}
                },
                Ok(Event::End(tag)) if tag.name() == b"Period" => {
                    TagName::Period.tag_close();
                    break;
                },
                Ok(Event::Eof) => {
                    ParsingError("Unexpected end of file in a Period.".to_owned())
                        .report_err();
                    break;
                }
                Err(e) => ParsingError::from(e).report_err(),
                _ => (),
            }
        }
    }

    fn process_adaptation_set_children(&mut self) {
        loop {
            match self.read_next_event() {
                Ok(Event::Start(tag)) => match tag.name() {
                    b"Accessibility" => {
                        // let _scheme = parse_scheme(&tag);
                        // XXX TODO communicate scheme
                    },
                    b"BaseURL" => {
                        TagName::BaseURL.tag_open();
                        attributes::report_base_url_attrs(&tag);
                        self.process_base_url();
                    },
                    b"ContentComponent" => {
                        // XXX TODO parse_content_component
                    },
                    b"ContentProtection" => {
                        // XXX TODO whole category?
                    },
                    b"EssentialProperty" => {
                        // let _scheme = parse_scheme(&tag);
                        // XXX TODO communicate scheme
                    },
                    b"Representation" => {
                        TagName::Representation.tag_open();
                        attributes::report_representation_attrs(&tag);
                        self.process_representation_children();
                    },
                    b"Role" => {
                        // let _scheme = parse_scheme(&tag);
                        // XXX TODO communicate scheme
                    },
                    b"SupplementalProperty" => {
                        // let _scheme = parse_scheme(&tag);
                        // XXX TODO communicate scheme
                    },
                    b"SegmentBase" => {
                        // XXX TODO whole category
                    },
                    b"SegmentList" => {
                        // XXX TODO whole category
                    },
                    b"SegmentTemplate" => {
                        TagName::SegmentTemplate.tag_open();
                        self.process_segment_template_children();
                        // XXX TODO SegmentTemplate
                    },
                    _ => {}
                },
                Ok(Event::Empty(tag)) => match tag.name() {
                    b"Accessibility" => {
                        // let _scheme = parse_scheme(&tag);
                        // XXX TODO communicate scheme
                    },
                    b"BaseURL" => {
                        TagName::BaseURL.tag_open();
                        attributes::report_base_url_attrs(&tag);
                        TagName::BaseURL.tag_close();
                    },
                    b"Representation" => {
                        TagName::Representation.tag_open();
                        attributes::report_representation_attrs(&tag);
                        TagName::Representation.tag_close();
                    },
                    b"SegmentTemplate" => {
                        TagName::SegmentTemplate.tag_open();
                        TagName::SegmentTemplate.tag_close();
                        // XXX TODO SegmentTemplate
                    },
                    _ => {}
                },
                Ok(Event::End(tag)) if tag.name() == b"AdaptationSet" => {
                    TagName::AdaptationSet.tag_close();
                    break;
                },
                Ok(Event::Eof) => {
                    ParsingError("Unexpected end of file in an AdaptationSet.".to_owned())
                        .report_err();
                    break;
                }
                Err(e) => ParsingError::from(e).report_err(),
                _ => (),
            }
            self.reader_buf.clear();
        }
    }

    fn process_representation_children(&mut self) {
        loop {
            match self.read_next_event() {
                Ok(Event::Start(tag)) => match tag.name() {
                    b"BaseURL" => {
                        TagName::BaseURL.tag_open();
                        attributes::report_base_url_attrs(&tag);
                        self.process_base_url();
                    },
                    b"SegmentBase" => {
                        // XXX TODO whole category
                    },
                    b"SegmentList" => {
                        // XXX TODO whole category
                    },
                    b"SegmentTemplate" => {
                        TagName::SegmentTemplate.tag_open();
                        self.process_segment_template_children();
                        // XXX TODO SegmentTemplate
                    },
                    _ => {}
                },
                Ok(Event::Empty(tag)) => match tag.name() {
                    b"BaseURL" => {
                        TagName::BaseURL.tag_open();
                        attributes::report_base_url_attrs(&tag);
                        TagName::BaseURL.tag_close();
                    },
                    b"SegmentTemplate" => {
                        TagName::SegmentTemplate.tag_open();
                        TagName::SegmentTemplate.tag_close();
                        // XXX TODO SegmentTemplate
                    },
                    _ => {}
                },
                Ok(Event::End(tag)) if tag.name() == b"Representation" => {
                    TagName::Representation.tag_close();
                    break;
                },
                Ok(Event::Eof) => {
                    ParsingError("Unexpected end of file in a Representation.".to_owned())
                        .report_err();
                    break;
                }
                Err(e) => ParsingError::from(e).report_err(),
                _ => (),
            }
            self.reader_buf.clear();
        }
    }

    fn process_segment_template_children(&mut self) {
        loop {
            match self.read_next_event() {
                Ok(Event::Start(tag)) | Ok(Event::Empty(tag))
                    if tag.name() == b"SegmentTimeline" =>
                        self.process_segment_timeline(),
                Ok(Event::End(tag)) if tag.name() == b"SegmentTemplate" => {
                    TagName::SegmentTemplate.tag_close();
                    break;
                },
                Ok(Event::Eof) => {
                    ParsingError("Unexpected end of file in a SegmentTemplate.".to_owned())
                        .report_err();
                    break;
                }
                Err(e) => ParsingError::from(e).report_err(),
                _ => (),
            }
            self.reader_buf.clear();
        }
    }

    fn process_segment_timeline(&mut self) {
        // Will store the ending timestamp of the previous <S> element, starting
        // at `0`.
        // Most subsequent <S> elements won't explicitly indicate a starting
        // timestamp which indicates that they start at the end of the previous
        // <S> element (its starting timestamp + its duration).
        let mut previous_s_base : f64 = 0.;

        loop {
            match self.read_next_event() {
                Ok(Event::Start(tag)) | Ok(Event::Empty(tag)) if tag.name() == b"S" => {
                    match SElement::from_xml(&tag, previous_s_base) {
                        Ok(s_element) => {
                            previous_s_base = s_element.start + s_element.duration;
                            self.ss_buf.push(s_element);
                        },
                        Err(err) => err.report_err(),
                    }
                },
                Ok(Event::End(tag)) if tag.name() == b"SegmentTimeline" => {
                    AttributeName::SegmentTimeline.report(self.ss_buf.as_slice());
                    break;
                },
                Ok(Event::Eof) => {
                    ParsingError("Unexpected end of file in a SegmentTimeline.".to_owned())
                        .report_err();
                    break;
                }
                Err(e) => ParsingError::from(e).report_err(),
                _ => (),
            }
        }
        self.ss_buf.clear();
    }

    fn process_location_tag(&mut self) {
        loop {
            match self.read_next_event() {
                Ok(Event::Text(t)) => if t.len() > 0 {
                    match t.unescaped() {
                        Ok(unescaped) => AttributeName::Location.report(unescaped),
                        Err(err) => ParsingError::from(err).report_err(),
                    }
                },
                Ok(Event::End(tag)) if tag.name() == b"Location" => {
                    break;
                },
                Ok(Event::Eof) => {
                    ParsingError("Unexpected end of file in a Location tag.".to_owned())
                        .report_err();
                    break;
                }
                Err(e) => ParsingError::from(e).report_err(),
                _ => (),
            }
            self.reader_buf.clear();
        }
    }

    fn process_base_url(&mut self) {
        loop {
            match self.read_next_event() {
                Ok(Event::Text(t)) => if t.len() > 0 {
                    match t.unescaped() {
                        Ok(unescaped) => AttributeName::Text.report(unescaped),
                        Err(err) => ParsingError::from(err).report_err(),
                    }
                },
                Ok(Event::End(tag)) if tag.name() == b"BaseURL" => {
                    TagName::BaseURL.tag_close();
                    break;
                },
                Ok(Event::Eof) => {
                    ParsingError("Unexpected end of file in a BaseURL.".to_owned())
                        .report_err();
                    break;
                }
                Err(e) => ParsingError::from(e).report_err(),
                _ => (),
            }
            self.reader_buf.clear();
        }
    }
}

/// Represents a parsed <S> node in the MPD.
/// Attributes are defined as f64 despite being u64 to simplify Rust-to-JS
/// communication.
#[repr(C)] // Used in FFI
#[derive(Debug, Clone, Copy, Default)]
pub struct SElement {
  start : f64,
  duration : f64,
  repeat_count : f64
}

impl SElement {
    #[inline(always)]
    fn from_xml(
        e : &quick_xml::events::BytesStart,
        previous_s_base : f64
    ) -> Result<SElement> {
        let mut parsed_s = SElement::default();
        let mut has_t = false;

        for res_attr in e.attributes() {
            match res_attr {
                Ok(attr) => {
                    let key = attr.key;
                    match key {
                        b"t" => {
                            parsed_s.start = parse_u64(&attr.value)? as f64;
                            has_t = true;
                        },
                        b"d" => {
                            parsed_s.duration = parse_u64(&attr.value)? as f64;
                        },
                        b"r" => {
                            parsed_s.repeat_count = parse_u64(&attr.value)? as f64;
                        },
                        _ => {},
                    }
                },
                Err(err) => ParsingError::from(err).report_err(),
            };
        }
        if !has_t {
            parsed_s.start = previous_s_base;
        }
        Ok(parsed_s)
    }
}

/// Attributes are defined as f64 despite being u64 to simplify Rust-to-JS
/// communication.
#[repr(C)] // Used in FFI
#[derive(Debug, Clone, Copy, Default)]
struct SegmentBaseSegment {
  start : f64,
  duration : f64,
  repeat_count : f64,
  range : (f64, f64),
}

// pub fn process_mpd(reader : &mut BufReader<MPDReader>) {
//     let mut reader = Reader::from_reader(reader);
//     let mut buf = Vec::new();
//     reader.trim_text(true);
//     loop {
//         match reader.read_event(&mut buf) {
//             Ok(Event::Start(tag)) => process_new_tag(&mut reader, &tag, false),
//             Ok(Event::Empty(tag)) => process_new_tag(&mut reader, &tag, true),
//             Ok(Event::End(tag)) => process_closing_tag(&tag),
//             Ok(Event::Eof) => break, // exits the loop when reaching end of file
//             Err(e) => ParsingError::from(e).report_err(),
//             _ => (),
//         }
//         // report_error(format!("LOOPED {}", buf.len()));
//         buf.clear();
//     }
// }

// fn process_segment_timeline(
//     reader : &mut quick_xml::Reader<&mut BufReader<MPDReader>>,
//     ss : &mut Vec<SElement>
// ) {
//     // let start = std::time::Instant::now();
//     let mut buf = Vec::new();

//     // Will store the ending timestamp of the previous <S> element, starting
//     // at `0`.
//     // Most subsequent <S> elements won't explicitly indicate a starting
//     // timestamp which indicates that they start at the end of the previous
//     // <S> element (its starting timestamp + its duration).
//     let mut previous_s_base : f64 = 0.;

//     loop {
//         match reader.read_event(&mut buf) {
//             Ok(Event::Start(tag)) | Ok(Event::Empty(tag)) if tag.name() == b"S" => {
//                 match SElement::from_xml(&tag, previous_s_base) {
//                     Ok(s_element) => {
//                         previous_s_base = s_element.start + s_element.duration;
//                         ss.push(s_element);
//                     },
//                     Err(err) => err.report_err(),
//                 }
//             },
//             Ok(Event::End(tag)) if tag.name() == b"SegmentTimeline" => {
//                 ss.report_as_attr(AttributeName::SegmentTimeline);
//                 break;
//             },
//             Ok(Event::Eof) => {
//                 ParsingError("Unexpected end of file in a SegmentTimeline.".to_owned())
//                  .report_err();
//                 break;
//             }
//             Err(e) => ParsingError::from(e).report_err(),
//             _ => (),
//         }
//         buf.clear();
//     }
//     ss.clear();
// }

// struct Scheme<'a> {
//   scheme_id_uri : Option<&'a [u8]>,
//   value : Option<&'a [u8]>,
// }

// fn parse_scheme<'a>(
//     tag_bs : &'a quick_xml::events::BytesStart
// ) -> Result<Scheme<'a>> {
//     let mut scheme_id_uri : Option<&'a [u8]> = None;
//     let mut value : Option<&'a [u8]> = None;
//     for res_attr in tag_bs.attributes() {
//         match res_attr {
//             Ok(attr) => {
//                 let key = attr.key;
//                 match key {
//                     b"schemeIdUri" => scheme_id_uri = Some(attr.value.as_ref()),
//                     b"value" => value = Some(attr.value.as_ref()),
//                     _ => {},
//                 }
//             },
//             Err(err) => return Err(ParsingError::from(err)),
//         };
//     };
//     return Ok(Scheme { scheme_id_uri, value });
// }

// fn process_scheme_attrs(tag_bs : &quick_xml::events::BytesStart) {
//     use AttributeName::*;
//     for res_attr in tag_bs.attributes() {
//         match res_attr {
//             Ok(attr) => {
//                 let key = attr.key;
//                 match key {
//                     b"schemeIdUri" => attr.value.report_as_attr(SchemeIdUri),
//                     b"value" => attr.value.report_as_attr(SchemeValue),
//                     _ => {},
//                 }
//             },
//             Err(err) => ParsingError::from(err).report_err(),
//         };
//     };
// }

// fn process_new_tag(
//     reader : &mut quick_xml::Reader<&mut BufReader<MPDReader>>,
//     tag_bs : &quick_xml::events::BytesStart,
//     is_empty_tag : bool,
// ) {
//     use super::report::{report_opening_tag, report_closing_tag};

//     // Create a common Vec for storing parsed <S> elements - from a
//     // possible SegmentTimeline, as they can be very numerous and should be in
//     // roughly similar numbers between SegmentTimelines, at least for the same
//     // Period.
//     // Re-using the same Vec (cleared between SegmentTimelines) allows us to
//     // profit from the maximum capacity between them without needing to
//     // re-allocating each time.
//     let mut ss_vec = Vec::new();

//     match tag_bs.name() {
//         // b"S" => parse_s_tag(&tag_bs, previous_s_base),
//         b"Representation" => process_representation(reader, &tag_bs),
//         b"AdaptationSet" => {
//             report_opening_tag(TagName::AdaptationSet);
//             report_adaptation_set_attrs(&tag_bs);
//             if is_empty_tag { report_closing_tag(TagName::AdaptationSet); }
//         },
//         b"MPD" => {
//             report_opening_tag(TagName::MPD);
//             process_mpd_attrs(&tag_bs);
//             if is_empty_tag { report_closing_tag(TagName::MPD); }
//         },
//         b"SegmentTemplate" => {
//             report_opening_tag(TagName::SegmentTemplate);

//             if is_empty_tag { report_closing_tag(TagName::SegmentTemplate); }
//         },
//         b"SegmentBase" => {
//             report_opening_tag(TagName::SegmentBase);

//             if is_empty_tag { report_closing_tag(TagName::SegmentBase); }
//         },
//         b"SegmentTimeline" => process_segment_timeline(reader, &mut ss_vec),
//         b"SegmentList" => {
//             report_opening_tag(TagName::SegmentList);

//             if is_empty_tag { report_closing_tag(TagName::SegmentList); }
//         },
//         b"Location" => report_opening_tag(TagName::Location),
//         b"Period" => {
//             report_opening_tag(TagName::Period);
//             report_period_attrs(&tag_bs);
//             if is_empty_tag { report_closing_tag(TagName::Period); }
//         }
//         b"UtcTiming" => report_opening_tag(TagName::UtcTiming),
//         b"EventStream" => report_opening_tag(TagName::EventStream),
//         b"Accessibility" => report_opening_tag(TagName::Accessibility),
//         b"ContentComponent" => report_opening_tag(TagName::ContentComponent),
//         b"ContentProtection" => report_opening_tag(TagName::ContentProtection),
//         b"EssentialProperty" => report_opening_tag(TagName::EssentialProperty),
//         b"Role" => report_opening_tag(TagName::Role),
//         b"SupplementalProperty" => report_opening_tag(TagName::SupplementalProperty),
//         b"Initialization" => report_opening_tag(TagName::Initialization),
//         _ => (),
//     }
// }

// fn process_closing_tag(
//     tag_bs : &quick_xml::events::BytesEnd
// ) {
//     use TagName::*;
//     use super::report::report_closing_tag;
//     match tag_bs.name() {
//         b"MPD" => report_closing_tag(MPD),
//         b"AdaptationSet" => report_closing_tag(AdaptationSet),
//         b"SegmentTemplate" => report_closing_tag(SegmentTemplate),
//         b"SegmentBase" => report_closing_tag(SegmentBase),
//         b"SegmentList" => report_closing_tag(SegmentList),
//         b"Location" => report_closing_tag(Location),
//         b"Period" => report_closing_tag(Period),
//         b"UtcTiming" => report_closing_tag(UtcTiming),
//         b"EventStream" => report_closing_tag(EventStream),
//         b"Representation" => report_closing_tag(Representation),
//         b"Accessibility" => report_closing_tag(Accessibility),
//         b"ContentComponent" => report_closing_tag(ContentComponent),
//         b"ContentProtection" => report_closing_tag(ContentProtection),
//         b"EssentialProperty" => report_closing_tag(EssentialProperty),
//         b"Role" => report_closing_tag(Role),
//         b"SupplementalProperty" => report_closing_tag(SupplementalProperty),
//         b"Initialization" => report_closing_tag(Initialization),
//         _ => (),
//     }
// }
