use std::io::BufReader;
use quick_xml::Reader;
use quick_xml::events::Event;

use super::event_enums::*;
use super::error::ParsingError;
use super::mpd_reader::MPDReader;
use super::utils::{
    parse_u64_to_js_float,
    parse_u64_or_bool_to_js_float,
    parse_bool,
    parse_f64,
};
use super::report::ReportableValue;

pub fn process_mpd(reader : &mut BufReader<MPDReader>) {
    let mut reader = Reader::from_reader(reader);
    let mut buf = Vec::new();
    reader.trim_text(true);
    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(e)) => process_new_tag(&mut reader, &e, false),
            Ok(Event::Empty(e)) => process_new_tag(&mut reader, &e, true),
            Ok(Event::Text(e)) => if e.len() > 0 {
                match e.unescaped() {
                    Ok(unescaped) => unescaped.report_as_attr(AttributeName::Text),
                    Err(err) => ParsingError::from(err).report_err(),
                }
            },
            Ok(Event::End(e)) => process_closing_tag(&e),
            Ok(Event::Eof) => break, // exits the loop when reaching end of file
            Err(e) => ParsingError::from(e).report_err(),
            _ => (),
        }
        // report_error(format!("LOOPED {}", buf.len()));
        buf.clear();
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
    fn from_xml(
        e : &quick_xml::events::BytesStart,
        previous_s_base : f64
    ) -> Result<SElement, ParsingError> {
        let mut parsed_s = SElement::default();
        let mut has_t = false;

        for res_attr in e.attributes() {
            match res_attr {
                Ok(attr) => {
                    let key = attr.key;
                    match key {
                        b"t" => {
                            parsed_s.start = parse_u64_to_js_float(&attr.value)?;
                            has_t = true;
                        },
                        b"d" => {
                            parsed_s.duration = parse_u64_to_js_float(&attr.value)?;
                        },
                        b"r" => {
                            parsed_s.repeat_count = parse_u64_to_js_float(&attr.value)?;
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

fn process_mpd_attrs(e : &quick_xml::events::BytesStart) {
    use AttributeName::*;
    for res_attr in e.attributes() {
        match res_attr {
            Ok(attr) => {
                let key = attr.key;
                match key {
                    b"id" => attr.value.report_as_attr(Id),
                    b"profiles" => attr.value.report_as_attr(Profiles),
                    b"type" => attr.value.report_as_attr(Type),
                    b"availabilityStartTime" =>
                        attr.value.report_as_attr(AvailabilityStartTime),
                    b"availabilityEndTime" =>
                        attr.value.report_as_attr(AvailabilityEndTime),
                    b"publishTime" => attr.value.report_as_attr(PublishTime),
                    b"mediaPresentationDuration" => attr.value.report_as_attr(Duration),
                    b"minimumUpdatePeriod" =>
                        attr.value.report_as_attr(MinimumUpdatePeriod),
                    b"minBufferTime" => attr.value.report_as_attr(MinBufferTime),
                    b"timeShiftBufferDepth" =>
                        attr.value.report_as_attr(TimeShiftBufferDepth),
                    b"suggestedPresentationDelay" =>
                        attr.value.report_as_attr(SuggestedPresentationDelay),
                    b"maxSegmentDuration" =>
                        attr.value.report_as_attr(MaxSegmentDuration),
                    b"maxSubsegmentDuration" =>
                        attr.value.report_as_attr(MaxSegmentDuration),
                    _ => {},
                }
            },
            Err(err) => ParsingError::from(err).report_err(),
        };
    };
}

fn parse_and_report_u64(
    attr_name : AttributeName,
    attr : &quick_xml::events::attributes::Attribute
) {
    match parse_u64_to_js_float(&attr.value) {
        Ok(val) => val.report_as_attr(attr_name),
        Err(error) => error.report_err(),
    }
}

fn parse_and_report_f64(
    attr_name : AttributeName,
    attr : &quick_xml::events::attributes::Attribute
) {
    match parse_f64(&attr.value) {
        Ok(val) => val.report_as_attr(attr_name),
        Err(error) => error.report_err(),
    }
}

fn parse_and_report_u64_or_bool(
    attr_name : AttributeName,
    attr : &quick_xml::events::attributes::Attribute
) {
    match parse_u64_or_bool_to_js_float(&attr.value) {
        Ok(val) => val.report_as_attr(attr_name),
        Err(error) => error.report_err(),
    }
}

fn parse_and_report_bool(
    attr_name : AttributeName,
    attr : &quick_xml::events::attributes::Attribute
) {
    match parse_bool(&attr.value) {
        Ok(val) => val.report_as_attr(attr_name),
        Err(error) => ParsingError::from(error).report_err(),
    }
}

fn report_adaptation_set_attrs(e : &quick_xml::events::BytesStart) {
    use AttributeName::*;
    for res_attr in e.attributes() {
        match res_attr {
            Ok(attr) => {
                let key = attr.key;
                match key {
                    b"id" => attr.value.report_as_attr(Id),
                    b"group" => parse_and_report_u64(Group, &attr),
                    b"lang" => attr.value.report_as_attr(Language),
                    b"contentType" => attr.value.report_as_attr(ContentType),
                    b"par" => attr.value.report_as_attr(Par),
                    b"minBandwidth" => parse_and_report_u64(MinBitrate, &attr),
                    b"maxBandwidth" => parse_and_report_u64(MaxBitrate, &attr),
                    b"minWidth" => parse_and_report_u64(MinWidth, &attr),
                    b"maxWidth" => parse_and_report_u64(MaxWidth, &attr),
                    b"minHeight" => parse_and_report_u64(MinHeight, &attr),
                    b"maxHeight" => parse_and_report_u64(MaxHeight, &attr),
                    b"minFrameRate" => attr.value.report_as_attr(MinFrameRate),
                    b"maxFrameRate" => attr.value.report_as_attr(MaxFrameRate),
                    b"selectionPriority" =>
                        parse_and_report_u64(SelectionPriority, &attr),
                    b"segmentAlignment" =>
                        parse_and_report_u64_or_bool(SegmentAlignment, &attr),
                    b"subsegmentAlignment" =>
                        parse_and_report_u64_or_bool(SubsegmentAlignment, &attr),
                    b"bitstreamSwitching" =>
                        parse_and_report_bool(BitStreamSwitching, &attr),
                    b"audioSamplingRate" => attr.value.report_as_attr(AudioSamplingRate),
                    b"codecs" => attr.value.report_as_attr(Codecs),
                    b"profiles" => attr.value.report_as_attr(Profiles),
                    b"segmentProfiles" => attr.value.report_as_attr(SegmentProfiles),
                    b"mimeType" => attr.value.report_as_attr(MimeType),
                    b"codingDependency" => parse_and_report_bool(CodingDependency, &attr),
                    b"frameRate" => attr.value.report_as_attr(FrameRate),
                    b"height" =>
                        parse_and_report_u64(Height, &attr),
                    b"width" =>
                        parse_and_report_u64(Height, &attr),
                    b"maxPlayoutRate" =>
                        parse_and_report_f64(MaxPlayoutRate, &attr),
                    b"maxSAPPeriod" =>
                        parse_and_report_f64(MaximumSAPPeriod, &attr),
                    _ => {},
                }
            },
            Err(err) => ParsingError::from(err).report_err(),
        };
    };
}

fn report_representation_attrs(e : &quick_xml::events::BytesStart) {
    use AttributeName::*;
    for res_attr in e.attributes() {
        match res_attr {
            Ok(attr) => {
                let key = attr.key;
                match key {
                    b"id" => attr.value.report_as_attr(Id),
                    b"audioSamplingRate" => attr.value.report_as_attr(AudioSamplingRate),
                    b"bandwidth" => parse_and_report_u64(Bitrate, &attr),
                    b"codecs" => attr.value.report_as_attr(Codecs),
                    b"codingDependency" => parse_and_report_bool(CodingDependency, &attr),
                    b"frameRate" => attr.value.report_as_attr(FrameRate),
                    b"height" => parse_and_report_u64(Height, &attr),
                    b"width" => parse_and_report_u64(Height, &attr),
                    b"maxPlayoutRate" => parse_and_report_f64(MaxPlayoutRate, &attr),
                    b"maxSAPPeriod" => parse_and_report_f64(MaximumSAPPeriod, &attr),
                    b"mimeType" => attr.value.report_as_attr(MimeType),
                    b"profiles" => attr.value.report_as_attr(Profiles),
                    b"qualityRanking" => parse_and_report_u64(QualityRanking, &attr),
                    b"segmentProfiles" => attr.value.report_as_attr(SegmentProfiles),
                    _ => {},
                }
            },
            Err(err) => ParsingError::from(err).report_err(),
        };
    };
}

fn report_period_attrs(e : &quick_xml::events::BytesStart) {
    for res_attr in e.attributes() {
        match res_attr {
            Ok(attr) => {
                let key = attr.key;
                match key {
                    b"id" =>
                        attr.value.report_as_attr(AttributeName::Id),
                    b"start" =>
                        attr.value.report_as_attr(AttributeName::Start),
                    b"duration" =>
                        attr.value.report_as_attr(AttributeName::Duration),
                    b"bitstreamSwitching" =>
                        attr.value.report_as_attr(AttributeName::BitStreamSwitching),
                    b"xlink:href" =>
                        attr.value.report_as_attr(AttributeName::XLinkHref),
                    b"xlink:actuate" =>
                        attr.value.report_as_attr(AttributeName::XLinkActuate),
                    _ => {},
                }
            },
            Err(err) => ParsingError::from(err).report_err(),
        };
    };
}

fn process_segment_timeline(
    reader : &mut quick_xml::Reader<&mut BufReader<MPDReader>>,
    ss : &mut Vec<SElement>
) {
    // let start = std::time::Instant::now();
    let mut buf = Vec::new();
    ParsingError("BEF".to_owned()).report_err();

    // Will store the ending timestamp of the previous <S> element, starting
    // at `0`.
    // Most subsequent <S> elements won't explicitly indicate a starting
    // timestamp which indicates that they start at the end of the previous
    // <S> element (its starting timestamp + its duration).
    let mut previous_s_base : f64 = 0.;

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(e)) | Ok(Event::Empty(e)) if e.name() == b"S" => {
                match SElement::from_xml(&e, previous_s_base) {
                    Ok(s_element) => {
                        previous_s_base = s_element.start + s_element.duration;
                        ss.push(s_element);
                    },
                    Err(err) => err.report_err(),
                }
            },
            Ok(Event::End(e)) if e.name() == b"SegmentTimeline" => {
                // let msg = "toto BEF";
                // unsafe {
                //     super::onCustomEvent(
                //         CustomEventType::Error,
                //         std::mem::transmute((msg).as_ptr()),
                //         msg.len());
                // }
                // let time_taken = std::time::Instant::now().duration_since(start).as_millis() as u64;
                // ParsingError(format!("AF").to_owned());
                // ParsingError(format!("Time Taken: {}", time_taken).to_owned())
                //  .report_err();
                // ParsingError(format!("AF2").to_owned());
                ParsingError("AF".to_owned()).report_err();
                ss.report_as_attr(AttributeName::SegmentTimeline);
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
        buf.clear();
    }
    ss.clear();
}

fn process_new_tag(
    reader : &mut quick_xml::Reader<&mut BufReader<MPDReader>>,
    e : &quick_xml::events::BytesStart,
    is_empty_tag : bool,
) {
    use super::report::{report_opening_tag, report_closing_tag};

    // Create a common Vec for storing parsed <S> elements - from a
    // possible SegmentTimeline, as they can be very numerous and should be in
    // roughly similar numbers between SegmentTimelines, at least for the same
    // Period.
    // Re-using the same Vec (cleared between SegmentTimelines) allows us to
    // profit from the maximum capacity between them without needing to
    // re-allocating each time.
    let mut ss_vec = Vec::new();

    match e.name() {
        // b"S" => parse_s_tag(&e, previous_s_base),
        b"Representation" => {
            report_opening_tag(TagName::Representation);
            report_representation_attrs(&e);
            if is_empty_tag { report_closing_tag(TagName::Representation); }
        },
        b"AdaptationSet" => {
            report_opening_tag(TagName::AdaptationSet);
            report_adaptation_set_attrs(&e);
            if is_empty_tag { report_closing_tag(TagName::AdaptationSet); }
        },
        b"MPD" => {
            report_opening_tag(TagName::MPD);
            process_mpd_attrs(&e);
            if is_empty_tag { report_closing_tag(TagName::MPD); }
        },
        b"SegmentTemplate" => {
            report_opening_tag(TagName::SegmentTemplate);

            if is_empty_tag { report_closing_tag(TagName::SegmentTemplate); }
        },
        b"SegmentBase" => {
            report_opening_tag(TagName::SegmentBase);

            if is_empty_tag { report_closing_tag(TagName::SegmentBase); }
        },
        b"SegmentTimeline" => process_segment_timeline(reader, &mut ss_vec),
        b"SegmentList" => {
            report_opening_tag(TagName::SegmentList);

            if is_empty_tag { report_closing_tag(TagName::SegmentList); }
        },
        b"Location" => report_opening_tag(TagName::Location),
        b"Period" => {
            report_opening_tag(TagName::Period);
            report_period_attrs(&e);
            if is_empty_tag { report_closing_tag(TagName::Period); }
        }
        b"UtcTiming" => report_opening_tag(TagName::UtcTiming),
        b"EventStream" => report_opening_tag(TagName::EventStream),
        b"Accessibility" => report_opening_tag(TagName::Accessibility),
        b"ContentComponent" => report_opening_tag(TagName::ContentComponent),
        b"ContentProtection" => report_opening_tag(TagName::ContentProtection),
        b"EssentialProperty" => report_opening_tag(TagName::EssentialProperty),
        b"Role" => report_opening_tag(TagName::Role),
        b"SupplementalProperty" => report_opening_tag(TagName::SupplementalProperty),
        b"Initialization" => report_opening_tag(TagName::Initialization),
        _ => (),
    }
}

fn process_closing_tag(
    e : &quick_xml::events::BytesEnd
) {
    use TagName::*;
    use super::report::report_closing_tag;
    match e.name() {
        b"MPD" => report_closing_tag(MPD),
        b"AdaptationSet" => report_closing_tag(AdaptationSet),
        b"SegmentTemplate" => report_closing_tag(SegmentTemplate),
        b"SegmentBase" => report_closing_tag(SegmentBase),
        b"SegmentList" => report_closing_tag(SegmentList),
        b"Location" => report_closing_tag(Location),
        b"Period" => report_closing_tag(Period),
        b"UtcTiming" => report_closing_tag(UtcTiming),
        b"EventStream" => report_closing_tag(EventStream),
        b"Representation" => report_closing_tag(Representation),
        b"Accessibility" => report_closing_tag(Accessibility),
        b"ContentComponent" => report_closing_tag(ContentComponent),
        b"ContentProtection" => report_closing_tag(ContentProtection),
        b"EssentialProperty" => report_closing_tag(EssentialProperty),
        b"Role" => report_closing_tag(Role),
        b"SupplementalProperty" => report_closing_tag(SupplementalProperty),
        b"Initialization" => report_closing_tag(Initialization),
        _ => (),
    }
}
