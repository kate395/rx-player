use crate::{ onTagClose, onTagOpen };

#[derive(Clone, Copy)]
#[repr(C)]
pub enum CustomEventType {
    Log = 0,
    Error = 1,
}

#[derive(PartialEq, Clone, Copy)]
#[repr(C)]
pub enum TagName {
    MPD = 1,

    // MPD
    Period = 2,
    UtcTiming = 3,

    // Period
    AdaptationSet = 4,
    EventStream = 5,
    EventStreamElt = 6,

    // AdaptationSet
    Representation = 7,
    Accessibility = 8,
    ContentComponent = 9,
    ContentProtection = 10,
    EssentialProperty = 11,
    Role = 12,
    SupplementalProperty = 13,
    Initialization = 14,

    // Various
    BaseURL = 15,
    SegmentTemplate = 16,
    SegmentBase = 17,
    SegmentList = 18,
}

#[derive(PartialEq, Clone, Copy)]
#[repr(C)]
#[no_mangle]
pub enum AttributeName {
    // MPD + Period + AdaptationSet + Representation + EventStream + ContentComponent
    Id = 0, // String

    // MPD + Period + SegmentTemplate + EventStream
    Duration = 1, // f64

    Profiles = 2,

    // AdaptationSet + Representation
    AudioSamplingRate = 3,
    Codecs = 4, // String
    CodingDependency = 5,
    FrameRate = 6,
    Height = 7, // f64
    Width = 8, // f64
    MaxPlayoutRate = 9,
    MaximumSAPPeriod = 10,
    MimeType = 11, // f64
    SegmentProfiles = 12,

    // ContentProtection
    ContentProtectionValue = 13, // String
    ContentProtectionKeyId = 14, // ArrayBuffer
    ContentProtectionCencPSSH = 15, // ArrayBuffer

    // Various schemes (Accessibility) + EventStream + ContentProtection
    SchemeIdUri = 16, // String

    // Various schemes (Accessibility)
    SchemeValue = 17, // String

    // SegmentURL
    MediaRange = 18, // [f64, f64]

    // SegmentTimeline
    SegmentTimeline = 19, // Vec<SElement>

    // SegmentTemplate
    StartNumber = 20, // f64

    // SegmentBase
    SegmentBaseSegment = 21, // SegmentBaseSegment

    // SegmentTemplate + SegmentBase
    AvailabilityTimeComplete = 22, // u8 (bool)
    IndexRangeExact = 23, // u8 (bool)
    PresentationTimeOffset = 24, // f64

    // EventStream
    EventPresentationTime = 25, // f64

    // EventStreamElt
    Element = 26, // String (XML)

    // SegmentTemplate + SegmentBase + EventStream + EventStreamElt
    TimeScale = 27, // f64

    // SegmentURL + SegmentTemplate
    Index = 28, // String

    // Initialization
    InitializationRange = 29, // [f64, f64]

    // SegmentURL + SegmentTemplate + SegmentBase + Initialization
    Media = 30, // String
    IndexRange = 31, // [f64, f64]

    // Period + AdaptationSet + SegmentTemplate
    BitStreamSwitching = 32, // u8 (bool)


    // MPD
    Type = 33, // String
    AvailabilityStartTime = 34, // f64
    AvailabilityEndTime = 35, // f64
    PublishTime = 36, // f64
    MinimumUpdatePeriod = 37, // f64
    MinBufferTime = 38, // f64
    TimeShiftBufferDepth = 39, // f64
    SuggestedPresentationDelay = 40, // f64
    MaxSegmentDuration = 41, // f64
    MaxSubsegmentDuration = 42, // f64

    // BaseURL + SegmentTemplate
    AvailabilityTimeOffset = 43, // f64

    // BaseURL
    BaseUrlValue = 44, // String

    // Period
    Start = 45, // f64
    XLinkHref = 46, // String
    XLinkActuate = 47, // String

    // AdaptationSet
    Group = 48,
    MaxBitrate = 49, // f64
    MaxFrameRate = 50, // f64
    MaxHeight = 51, // f64
    MaxWidth = 52, // f64
    MinBitrate = 53, // f64
    MinFrameRate = 54, // f64
    MinHeight = 55, // f64
    MinWidth = 56, // f64
    SelectionPriority = 57,
    SegmentAlignment = 58,
    SubsegmentAlignment = 59,

    // AdaptationSet + ContentComponent
    Language = 60, // String
    ContentType = 61, // String
    Par = 62,

    // Representation
    Bitrate = 63, // f64

    Text = 64,
    QualityRanking = 65,
    Location = 66,
}

impl TagName {
    /// Signal a new tag opening to the application
    pub fn tag_open(self) {
        debug_assert!(self as u64 <= u8::MAX as u64);

        // UNSAFE: We're using FFI, but there's should be no risk at all here
        unsafe { onTagOpen(self as u8) };
    }

    /// Signal that a previously-open tag closed to the application
    pub fn tag_close(self) {
        debug_assert!(self as u64 <= u8::MAX as u64);

        // UNSAFE: We're using FFI, but there's should be no risk at all here
        unsafe { onTagClose(self as u8) };
    }
}

use crate::reportable::ReportableValue;
use crate::utils::*;

impl AttributeName {
    #[inline(always)]
    pub fn report<T: ReportableValue>(self, val: T) {
        val.report_as_attr(self)
    }

    pub fn try_report_as_f64(
        self,
        attr : &quick_xml::events::attributes::Attribute
    ) {
        match parse_f64(&attr.value) {
            Ok(val) => self.report(val),
            Err(error) => error.report_err(),
        }
    }

    pub fn try_report_as_iso_8601_duration(
        self,
        attr : &quick_xml::events::attributes::Attribute
    ) {
        match parse_iso_8601_duration(&attr.value) {
            Ok(val) => self.report(val),
            Err(error) => error.report_err(),
        }
    }

    pub fn try_report_as_u64(
        self,
        attr : &quick_xml::events::attributes::Attribute
    ) {
        match parse_u64(&attr.value) {
            Ok(val) => self.report(val as f64),
            Err(error) => error.report_err(),
        }
    }

    pub fn try_report_as_u64_or_bool(
        self,
        attr : &quick_xml::events::attributes::Attribute
    ) {
        match parse_u64_or_bool(&attr.value) {
            Ok(val) => self.report(val),
            Err(error) => error.report_err(),
        }
    }

    pub fn try_report_as_bool(
        self,
        attr : &quick_xml::events::attributes::Attribute
    ) {
        match parse_bool(&attr.value) {
            Ok(val) => self.report(val),
            Err(error) => error.report_err(),
        }
    }
}
