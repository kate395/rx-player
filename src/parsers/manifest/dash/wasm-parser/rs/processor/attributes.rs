use crate::errors::ParsingError;
use crate::events::AttributeName::*;

pub fn report_mpd_attrs(e : &quick_xml::events::BytesStart) {
    for res_attr in e.attributes() {
        match res_attr {
            Ok(attr) => match attr.key {
                b"id" => Id.report(attr.value),
                b"profiles" => Profiles.report(attr.value),
                b"type" => Type.report(attr.value),
                b"availabilityStartTime" => AvailabilityStartTime.report(attr.value),
                b"availabilityEndTime" => AvailabilityEndTime.report(attr.value),
                b"publishTime" => PublishTime.report(attr.value),
                b"mediaPresentationDuration" =>
                    Duration.try_report_as_iso_8601_duration(&attr),
                b"minimumUpdatePeriod" =>
                    MinimumUpdatePeriod.try_report_as_iso_8601_duration(&attr),
                b"minBufferTime" =>
                    MinBufferTime.try_report_as_iso_8601_duration(&attr),
                b"timeShiftBufferDepth" =>
                    TimeShiftBufferDepth.try_report_as_iso_8601_duration(&attr),
                b"suggestedPresentationDelay" =>
                    SuggestedPresentationDelay.try_report_as_iso_8601_duration(&attr),
                b"maxSegmentDuration" =>
                    MaxSegmentDuration.try_report_as_iso_8601_duration(&attr),
                b"maxSubsegmentDuration" =>
                    MaxSegmentDuration.try_report_as_iso_8601_duration(&attr),
                _ => {},
            },
            Err(err) => ParsingError::from(err).report_err(),
        };
    };
}

pub fn report_period_attrs(tag_bs : &quick_xml::events::BytesStart) {
    for res_attr in tag_bs.attributes() {
        match res_attr {
            Ok(attr) => match attr.key {
                b"id" => Id.report(attr.value),
                b"start" => Start.try_report_as_iso_8601_duration(&attr),
                b"duration" => Duration.try_report_as_iso_8601_duration(&attr),
                b"bitstreamSwitching" => BitStreamSwitching.try_report_as_bool(&attr),
                b"xlink:href" => XLinkHref.report(attr.value),
                b"xlink:actuate" => XLinkActuate.report(attr.value),
                _ => {},
            },
            Err(err) => ParsingError::from(err).report_err(),
        };
    };
}

pub fn report_adaptation_set_attrs(e : &quick_xml::events::BytesStart) {
    for res_attr in e.attributes() {
        match res_attr {
            Ok(attr) => match attr.key {
                b"id" => Id.report(attr.value),
                b"group" => Group.try_report_as_u64(&attr),
                b"lang" => Language.report(attr.value),
                b"contentType" => ContentType.report(attr.value),
                b"par" => Par.report(attr.value),
                b"minBandwidth" => MinBitrate.try_report_as_u64(&attr),
                b"maxBandwidth" => MaxBitrate.try_report_as_u64(&attr),
                b"minWidth" => MinWidth.try_report_as_u64(&attr),
                b"maxWidth" => MaxWidth.try_report_as_u64(&attr),
                b"minHeight" => MinHeight.try_report_as_u64(&attr),
                b"maxHeight" => MaxHeight.try_report_as_u64(&attr),
                b"minFrameRate" => MinFrameRate.report(attr.value),
                b"maxFrameRate" => MaxFrameRate.report(attr.value),
                b"selectionPriority" => SelectionPriority.try_report_as_u64(&attr),
                b"segmentAlignment" => SegmentAlignment.try_report_as_u64_or_bool(&attr),
                b"subsegmentAlignment" =>
                    SubsegmentAlignment.try_report_as_u64_or_bool(&attr),
                b"bitstreamSwitching" => BitStreamSwitching.try_report_as_bool(&attr),
                b"audioSamplingRate" => AudioSamplingRate.report(attr.value),
                b"codecs" => Codecs.report(attr.value),
                b"profiles" => Profiles.report(attr.value),
                b"segmentProfiles" => SegmentProfiles.report(attr.value),
                b"mimeType" => MimeType.report(attr.value),
                b"codingDependency" => CodingDependency.try_report_as_bool(&attr),
                b"frameRate" => FrameRate.report(attr.value),
                b"height" => Height.try_report_as_u64(&attr),
                b"width" => Width.try_report_as_u64(&attr),
                b"maxPlayoutRate" => MaxPlayoutRate.try_report_as_f64(&attr),
                b"maxSAPPeriod" => MaximumSAPPeriod.try_report_as_f64(&attr),
                _ => {},
            },
            Err(err) => ParsingError::from(err).report_err(),
        };
    };
}

pub fn report_representation_attrs(tag_bs : &quick_xml::events::BytesStart) {
    for res_attr in tag_bs.attributes() {
        match res_attr {
            Ok(attr) => match attr.key {
                b"id" => Id.report(attr.value),
                b"audioSamplingRate" => AudioSamplingRate.report(attr.value),
                b"bandwidth" => Bitrate.try_report_as_u64(&attr),
                b"codecs" => Codecs.report(attr.value),
                b"codingDependency" => CodingDependency.try_report_as_bool(&attr),
                b"frameRate" => FrameRate.report(attr.value),
                b"height" => Height.try_report_as_u64(&attr),
                b"width" => Width.try_report_as_u64(&attr),
                b"maxPlayoutRate" => MaxPlayoutRate.try_report_as_f64(&attr),
                b"maxSAPPeriod" => MaximumSAPPeriod.try_report_as_f64(&attr),
                b"mimeType" => MimeType.report(attr.value),
                b"profiles" => Profiles.report(attr.value),
                b"qualityRanking" => QualityRanking.try_report_as_u64(&attr),
                b"segmentProfiles" => SegmentProfiles.report(attr.value),
                _ => {},
            },
            Err(err) => ParsingError::from(err).report_err(),
        };
    };
}

pub fn report_base_url_attrs(tag_bs : &quick_xml::events::BytesStart) {
    for res_attr in tag_bs.attributes() {
        match res_attr {
            Ok(attr) => if let b"availabilityTimeOffset" = attr.key {
                match attr.value.as_ref() {
                    b"INF" => AvailabilityTimeOffset.report(f64::INFINITY),
                    _ => AvailabilityTimeOffset.try_report_as_u64(&attr),
                }
            },
            Err(err) => ParsingError::from(err).report_err(),
        };
    };
}

pub fn report_content_component_attrs(tag_bs : &quick_xml::events::BytesStart) {
    for res_attr in tag_bs.attributes() {
        match res_attr {
            Ok(attr) => match attr.key {
                b"id" => Id.report(attr.value),
                b"lang" => Language.report(attr.value),
                b"contentType" => ContentType.report(attr.value),
                b"par" => Par.report(attr.value),
                _ => {},
            },
            Err(err) => ParsingError::from(err).report_err(),
        };
    };
}

pub fn report_content_protection_attrs(tag_bs : &quick_xml::events::BytesStart) {
    for res_attr in tag_bs.attributes() {
        match res_attr {
            Ok(attr) => match attr.key {
                b"schemeIdUri" => Id.report(attr.value),
                b"value" => Language.report(attr.value),
                b"cenc:default_KID" => ContentType.report(attr.value),
                _ => {},
            },
            Err(err) => ParsingError::from(err).report_err(),
        };
    };
}
        // let hex = hex.as_ref();
        // if hex.len() % 2 != 0 {
        //     return Err(FromHexError::OddLength);
        // }

        // hex.chunks(2).enumerate().map(|(i, pair)| {
        //     Ok(val(pair[0], 2 * i)? << 4 | val(pair[1], 2 * i + 1)?)
        // }).collect()
