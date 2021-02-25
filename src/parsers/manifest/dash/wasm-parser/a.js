let i = 0;
self.onmessage = function (url) {
  runProgram(url.data);
};

const TagName = {
    MPD: 1,

    // MPD
    Period: 2,
    UtcTiming: 3,

    // Period
    AdaptationSet: 4,
    EventStream: 5,
    EventStreamElt: 6,

    // AdaptationSet
    Representation: 7,
    Accessibility: 8,
    ContentComponent: 9,
    ContentProtection: 10,
    EssentialProperty: 11,
    Role: 12,
    SupplementalProperty: 13,
    Initialization: 14,

    // Various
    BaseURL: 15,
    SegmentTemplate: 16,
    SegmentBase: 17,
    SegmentList: 18,
}

const AttributeName = {
    // MPD + Period + AdaptationSet + Representation + EventStream + ContentComponent
    Id: 0, // String

    // MPD + Period + SegmentTemplate + EventStream
    Duration: 1, // f64

    Profiles: 2,

    // AdaptationSet + Representation
    AudioSamplingRate: 3,
    Codecs: 4, // String
    CodingDependency: 5,
    FrameRate: 6,
    Height: 7, // f64
    Width: 8, // f64
    MaxPlayoutRate: 9,
    MaximumSAPPeriod: 10,
    MimeType: 11, // f64
    SegmentProfiles: 12,

    // ContentProtection
    ContentProtectionValue: 13, // String
    ContentProtectionKeyId: 14, // ArrayBuffer
    ContentProtectionCencPSSH: 15, // ArrayBuffer

    // Various schemes (Accessibility) + EventStream + ContentProtection
    SchemeIdUri: 16, // String

    // Various schemes (Accessibility)
    SchemeValue: 17, // String

    // SegmentURL
    MediaRange: 18, // [f64, f64]

    // SegmentTimeline
    SegmentTimeline: 19, // Vec<SElement>

    // SegmentTemplate
    StartNumber: 20, // f64

    // SegmentBase
    SegmentBaseSegment: 21, // SegmentBaseSegment

    // SegmentTemplate + SegmentBase
    AvailabilityTimeComplete: 22, // u8 (bool)
    IndexRangeExact: 23, // u8 (bool)
    PresentationTimeOffset: 24, // f64

    // EventStream
    EventPresentationTime: 25, // f64

    // EventStreamElt
    Element: 26, // String (XML)

    // SegmentTemplate + SegmentBase + EventStream + EventStreamElt
    TimeScale: 27, // f64

    // SegmentURL + SegmentTemplate
    Index: 28, // String

    // Initialization
    InitializationRange: 29, // [f64, f64]

    // SegmentURL + SegmentTemplate + SegmentBase + Initialization
    Media: 30, // String
    IndexRange: 31, // [f64, f64]

    // Period + AdaptationSet + SegmentTemplate
    BitStreamSwitching: 32, // u8 (bool)


    // MPD
    Type: 33, // String
    AvailabilityStartTime: 34, // f64
    AvailabilityEndTime: 35, // f64
    PublishTime: 36, // f64
    MinimumUpdatePeriod: 37, // f64
    MinBufferTime: 38, // f64
    TimeShiftBufferDepth: 39, // f64
    SuggestedPresentationDelay: 40, // f64
    MaxSegmentDuration: 41, // f64
    MaxSubsegmentDuration: 42, // f64

    // BaseURL + SegmentTemplate
    AvailabilityTimeOffset: 43, // f64

    // BaseURL
    BaseUrlValue: 44, // String

    // Period
    Start: 45, // f64
    XLinkHref: 46, // String
    XLinkActuate: 47, // String

    // AdaptationSet
    Group: 48,
    MaxBitrate: 49, // f64
    MaxFrameRate: 50, // f64
    MaxHeight: 51, // f64
    MaxWidth: 52, // f64
    MinBitrate: 53, // f64
    MinFrameRate: 54, // f64
    MinHeight: 55, // f64
    MinWidth: 56, // f64
    SelectionPriority: 57,
    SegmentAlignment: 58,
    SubsegmentAlignment: 59,

    // AdaptationSet + ContentComponent
    Language: 60, // String
    ContentType: 61, // String
    Par: 62,

    // Representation
    Bitrate: 63, // f64

    Text: 64,
    QualityRanking: 65,
    Location: 66,
}

const parsedS = [];
const textDecoder = new TextDecoder();

function noop() {}

class ParserState {
  constructor() {
    this._currentAttributeParser = noop;
    this._currentNode = null;
    this._attributeParsersStack = [];
  }

  setNewParser(parserFn) {
    this._currentAttributeParser = parserFn;
    this._attributeParsersStack.push(parserFn);
  }

  popParser() {
    this._attributeParsersStack.pop();
    this._currentAttributeParser =
      this._attributeParsersStack[this._attributeParsersStack.length - 1];
  }

  getCurrentParser() {
    return this._currentAttributeParser;
  }
}

let MPD = null;
function runProgram(url) {
  parsedS.length = 0;
  let linearMemory;
  let globalMpd;
  let currentPos = 0;
  MPD = {
    periods: [],
    segmentTemplates: []
  };

  const parserState = new ParserState();
  let lastPeriod = null;
  let lastAdaptationSet = null;

  function onTagOpen(tag) {
    switch (tag) {
      case TagName.Representation:
        if (lastAdaptationSet === null) {
          console.warn("WASH-DASH: A Representation is not in an AdaptationSet");
        } else {
          const representationObj = {};
          lastAdaptationSet.representations.push(representationObj);
          currentObj = representationObj;
          lastRepresentation = representationObj;
          parserState.setNewParser(generateRepresentationAttrParser(representationObj,
                                                                    linearMemory));
        }
        break;
      case TagName.AdaptationSet:
        if (lastPeriod === null) {
          console.warn("WASH-DASH: An AdaptationSet is not in a Period");
        } else {
          const adaptationObj = { representations: [] };
          lastPeriod.adaptations.push(adaptationObj);
          currentObj = adaptationObj;
          lastAdaptationSet = adaptationObj;
          parserState.setNewParser(generateAdaptationSetAttrParser(adaptationObj,
                                                                   linearMemory));
        }
        break;
      case TagName.Period:
        const periodObj = { adaptations: [] };
        currentObj = periodObj;
        lastPeriod = periodObj;
        MPD.periods.push(periodObj);
        parserState.setNewParser(generatePeriodAttrParser(periodObj, linearMemory));
        break;
      case TagName.MPD:
        currentObj = MPD;
        parserState.setNewParser(generateMPDAttrParser(MPD, linearMemory));
        break;
      case TagName.SegmentTemplate:
        const segmentTemplateObj = {};
        MPD.segmentTemplates.push(segmentTemplateObj);
        parserState.setNewParser(generateSegmentTemplateAttrParser(segmentTemplateObj,
                                                                   linearMemory));
        break;
    }
  }

  function onTagClose(tag) {
    switch (tag) {
      case TagName.MPD:
      case TagName.Period:
      case TagName.AdaptationSet:
      case TagName.Representation:
      case TagName.SegmentTemplate:
        parserState.popParser();
        break;
    }
  }

  function onCustomEvent(evt, ptr, len) {
    if (evt === 0) {
      const arr = new Uint8Array(linearMemory.buffer, ptr, len);
      console.log("log:", textDecoder.decode(arr));
    } if (evt === 1) {
      const arr = new Uint8Array(linearMemory.buffer, ptr, len);
      console.error("log:", textDecoder.decode(arr));
      const dec = textDecoder.decode(arr);
      if (dec === "BEF") {
        console.time("RS");
      } else if (dec === "AF") {
        console.timeEnd("RS");
      }
    }
  }

  function onAttribute(attr, ptr, len) {
    return parserState.getCurrentParser()(attr, ptr, len);
  }

  const MAX_READ_SIZE = 15e3;

  function read_next(ptr, size) {
    const sizeToRead = Math.min(size, MAX_READ_SIZE, globalMpd.byteLength - currentPos);
    const arr = new Uint8Array(linearMemory.buffer, ptr, sizeToRead);
    arr.set(new Uint8Array(globalMpd, currentPos, sizeToRead));
    currentPos += sizeToRead;
    return sizeToRead;
  }

  const imports = {
    env: {
      memoryBase: 0,
      tableBase: 0,
      memory: new WebAssembly.Memory({ initial: 100, maximum: 500, shared: true }),
      table: new WebAssembly.Table({initial: 4, element: 'anyfunc'}),
      onTagOpen,
      onCustomEvent,
      onAttribute,
      read_next,
      onTagClose,
    }
  };

  return fetch("./mpd_node_parser.wasm")
  // return fetch("./a.wasm")
    .then(res => res.arrayBuffer())
    .then(resAb => {
      console.time("COMPILE");
      const res = WebAssembly.instantiate(resAb, imports);
      console.timeEnd("COMPILE");
      return res;
    })
  // return WebAssembly.instantiateStreaming(fetch("./mpd_node_parser.wasm", imports))
    .then(resWasm => {
      return fetch(url)
        .then(res => res.arrayBuffer())
        .then(mpd => {
          linearMemory = resWasm.instance.exports.memory;
          dataView = new DataView(linearMemory.buffer);
          globalMpd = mpd;
          let now = performance.now();
          self.postMessage("s");
          resWasm.instance.exports.parse();

          console.log("TIME: ", performance.now() - now, MPD.segmentTemplates.map(t => t.timeline));
          self.postMessage(MPD, MPD.segmentTemplates.map(t => t.timeline));

          // delete MPD.segmentTemplates;
          // self.postMessage(MPD);

          console.log("SENT");
          self.postMessage("e");
          console.log(MPD);
          // if (mpd.byteLength > linearMemory.buffer.byteLength) {
          //   debugger;
          //   const nbPages = Math.ceil(
          //     (mpd.byteLength - linearMemory.buffer.byteLength) / 65536);
          //   linearMemory.grow(nbPages);
          // }
          // const buffer = new Uint8Array(linearMemory.buffer);
          // buffer.set(new Uint8Array(mpd));
          // let now = performance.now();
          // resWasm.instance.exports.parse(0, mpd.byteLength);
          // console.log("TIME: ", performance.now() - now);
        })
    });
  // return fetch("./mpd_node_parser.wasm")
  //   .then(res => res.arrayBuffer())
  //   .then(resAb => WebAssembly.instantiate(resAb, imports))
  //   .then((resWasm) => {
  //     return fetch("./m6-hd.mpd")
  //       .then(async (response) => {
  //         if (!response.ok) {
  //           throw new Error("B");
  //         }
  //         // Get the reader to stream the document to sax-wasm
  //         const reader = response.body.getReader();
  //         while(true) {
  //           const chunk = await reader.read();
  //           linearMemory = resWasm.instance.exports.memory;
  //             // .then(chunk => {
  //               if (chunk.done) {
  //                 resWasm.instance.exports.parse();
  //                 return;
  //               }
  //               const buffer = new Uint8Array(linearMemory.buffer);
  //               buffer.set(chunk.value);
  //               // let now = performance.now();
  //               resWasm.instance.exports.add(0, buffer.length);
  //               // console.log("TIME: ", performance.now() - now);
  //             // });
  //         }
  //       });
  //   });
}
function parseString(buffer, ptr, len) {
  const arr = new Uint8Array(buffer, ptr, len);
  return textDecoder.decode(arr);
}

function parseFloatOrBool(val) {
  return val === Infinity  ? true :
         val === -Infinity ? false :
                             val;
}

function generateMPDAttrParser(mpdObj, linearMemory) {
  let dataView;
  return function onMPDAttribute(attr, ptr, len) {
    switch (attr) {
      case AttributeName.Id:
        mpdObj.id = parseString(linearMemory.buffer, ptr, len);
        break;
      case AttributeName.Profiles:
        mpdObj.profiles = parseString(linearMemory.buffer, ptr, len);
        break;
      case AttributeName.Type:
        mpdObj.type = parseString(linearMemory.buffer, ptr, len);
        break;
      case AttributeName.AvailabilityStartTime:
        mpdObj.availabilityStartTime =
          new Date(parseString(linearMemory.buffer, ptr, len)).getTime() / 1000;
        break;
      case AttributeName.AvailabilityEndTime:
        mpdObj.availabilityEndTime =
          new Date(parseString(linearMemory.buffer, ptr, len)).getTime() / 1000;
        break;
      case AttributeName.PublishTime:
        mpdObj.publishTime =
          new Date(parseString(linearMemory.buffer, ptr, len)).getTime() / 1000;
        break;
      case AttributeName.Duration:
        dataView = new DataView(linearMemory.buffer);
        mpdObj.duration = dataView.getFloat64(ptr, true);
      case AttributeName.MinimumUpdatePeriod:
        dataView = new DataView(linearMemory.buffer);
        mpdObj.minimumUpdatePeriod = dataView.getFloat64(ptr, true);
        break;
      case AttributeName.MinBufferTime:
        dataView = new DataView(linearMemory.buffer);
        mpdObj.minBufferTime = dataView.getFloat64(ptr, true);
        break;
      case AttributeName.TimeShiftBufferDepth:
        dataView = new DataView(linearMemory.buffer);
        mpdObj.timeShiftBufferDepth = dataView.getFloat64(ptr, true);
        break;
      case AttributeName.SuggestedPresentationDelay:
        dataView = new DataView(linearMemory.buffer);
        mpdObj.suggestedPresentationDelay = dataView.getFloat64(ptr, true);
        break;
      case AttributeName.MaxSegmentDuration:
        dataView = new DataView(linearMemory.buffer);
        mpdObj.maxSegmentDuration = dataView.getFloat64(ptr, true);
        break;
      case AttributeName.MaxSubsegmentDuration:
        dataView = new DataView(linearMemory.buffer);
        mpdObj.maxSubsegmentDuration = dataView.getFloat64(ptr, true);
        break;
    }
  };
}

function generatePeriodAttrParser(periodObj, linearMemory) {
  return function onPeriodAttribute(attr, ptr, len) {
    switch (attr) {
      case AttributeName.Id:
        periodObj.id = parseString(linearMemory.buffer, ptr, len);
        break;
      case AttributeName.Start:
        periodObj.start = new DataView(linearMemory.buffer).getFloat64(ptr, true);
        break;
      case AttributeName.Duration:
        periodObj.duration = new DataView(linearMemory.buffer).getFloat64(ptr, true);
        break;
      case AttributeName.BitStreamSwitching:
        periodObj.bitstreamSwitching =
          new DataView(linearMemory.buffer).getUint8(0) === 0;
        break;
      case AttributeName.XLinkHref:
        periodObj.xlinkHref = parseString(linearMemory.buffer, ptr, len);
        break;
      case AttributeName.xlinkActuate:
        periodObj.xlinkActuate = parseString(linearMemory.buffer, ptr, len);
        break;
    }
  };
}

function generateAdaptationSetAttrParser(adaptationObj, linearMemory) {
  return function onAdaptationSetAttribute(attr, ptr, len) {
    const dataView = new DataView(linearMemory.buffer);
    switch (attr) {
      case AttributeName.Id:
        adaptationObj.id = parseString(linearMemory.buffer, ptr, len);
        break;
      case AttributeName.Group:
        adaptationObj.group = dataView.getFloat64(ptr, true);
        break;
      case AttributeName.Language:
        adaptationObj.language = parseString(linearMemory.buffer, ptr, len);
        break;
      case AttributeName.ContentType:
        adaptationObj.contentType = parseString(linearMemory.buffer, ptr, len);
        break;
      case AttributeName.Par:
        adaptationObj.par = parseString(linearMemory.buffer, ptr, len);
        break;
      case AttributeName.MinBitrate:
        adaptationObj.MinBitrate = dataView.getFloat64(ptr, true);
        break;
      case AttributeName.MaxBitrate:
        adaptationObj.MaxBitrate = dataView.getFloat64(ptr, true);
        break;
      case AttributeName.MinWidth:
        adaptationObj.MinWidth = dataView.getFloat64(ptr, true);
        break;
      case AttributeName.MaxWidth:
        adaptationObj.MaxWidth = dataView.getFloat64(ptr, true);
        break;
      case AttributeName.MinHeight:
        adaptationObj.MinHeight = dataView.getFloat64(ptr, true);
        break;
      case AttributeName.MaxHeight:
        adaptationObj.MaxHeight = dataView.getFloat64(ptr, true);
        break;
      case AttributeName.MinFrameRate:
        adaptationObj.minFrameRate = parseString(linearMemory.buffer, ptr, len);
        break;
      case AttributeName.MaxFrameRate:
        adaptationObj.maxFrameRate = parseString(linearMemory.buffer, ptr, len);
        break;
      case AttributeName.SelectionPriority:
        adaptationObj.selectionPriority = dataView.getFloat64(ptr, true);
        break;
      case AttributeName.SegmentAlignment:
        adaptationObj.segmentAlignment = parseFloatOrBool(dataView.getFloat64(ptr, true));
        break;
      case AttributeName.SubsegmentAlignment:
        adaptationObj.segmentAlignment = parseFloatOrBool(dataView.getFloat64(ptr, true));
        break;
      case AttributeName.BitstreamSwitching:
        adaptationObj.bitstreamSwitching = dataView.getFloat64(ptr, true) !== 0;
        break;
      case AttributeName.AudioSamplingRate:
        adaptationObj.audioSamplingRate = parseString(linearMemory.buffer, ptr, len);
        break;
      case AttributeName.Codecs:
        adaptationObj.codecs = parseString(linearMemory.buffer, ptr, len);
        break;
      case AttributeName.Profiles:
        adaptationObj.profiles = parseString(linearMemory.buffer, ptr, len);
        break;
      case AttributeName.SegmentProfiles:
        adaptationObj.segmentProfiles = parseString(linearMemory.buffer, ptr, len);
        break;
      case AttributeName.MimeType:
        adaptationObj.mimeType = parseString(linearMemory.buffer, ptr, len);
        break;
      case AttributeName.CodingDependency:
        adaptationObj.codingDependency = dataView.getFloat64(ptr, true) !== 0;
        break;
      case AttributeName.FrameRate:
        adaptationObj.frameRate = parseString(linearMemory.buffer, ptr, len);
        break;
      case AttributeName.Height:
        adaptationObj.height = dataView.getFloat64(ptr, true);
        break;
      case AttributeName.Width:
        adaptationObj.width = dataView.getFloat64(ptr, true);
        break;
      case AttributeName.MaxPlayoutRate:
        adaptationObj.maxPlayoutRate = dataView.getFloat64(ptr, true);
        break;
      case AttributeName.MaximumSAPPeriod:
        adaptationObj.maxSAPPeriod = dataView.getFloat64(ptr, true);
        break;
    }
  };
}

function generateRepresentationAttrParser(representationObj, linearMemory) {
  return function onRepresentationAttribute(attr, ptr, len) {
    const dataView = new DataView(linearMemory.buffer);
    switch (attr) {
      case AttributeName.Id:
        representationObj.id = parseString(linearMemory.buffer, ptr, len);
        break;
      case AttributeName.AudioSamplingRate:
        representationObj.audioSamplingRate = parseString(linearMemory.buffer, ptr, len);
        break;
      case AttributeName.Bitrate:
        representationObj.bitrate = dataView.getFloat64(ptr, true);
        break;
      case AttributeName.Codecs:
        representationObj.codecs = parseString(linearMemory.buffer, ptr, len);
        break;
      case AttributeName.CodingDependency:
        periodObj.codingDependency =
          new DataView(linearMemory.buffer).getUint8(0) === 0;
      case AttributeName.FrameRate:
        representationObj.frameRate = parseString(linearMemory.buffer, ptr, len);
        break;
      case AttributeName.Height:
        representationObj.height = dataView.getFloat64(ptr, true);
        break;
      case AttributeName.Width:
        representationObj.width = dataView.getFloat64(ptr, true);
        break;
      case AttributeName.MaxPlayoutRate:
        representationObj.maxPlayoutRate = dataView.getFloat64(ptr, true);
        break;
      case AttributeName.MaximumSAPPeriod:
        representationObj.maxSAPPeriod = dataView.getFloat64(ptr, true);
        break;
      case AttributeName.MimeType:
        representationObj.mimeType = parseString(linearMemory.buffer, ptr, len);
        break;
      case AttributeName.Profiles:
        representationObj.profiles = parseString(linearMemory.buffer, ptr, len);
        break;
      case AttributeName.QualityRanking:
        representationObj.qualityRanking = dataView.getFloat64(ptr, true);
        break;
      case AttributeName.SegmentProfiles:
        representationObj.segmentProfiles =
          parseString(linearMemory.buffer, ptr, len);
        break;
    }
  };
}

function generateSegmentTemplateAttrParser(segmentTemplateObj, linearMemory) {
  return function onSegmentTemplateAttribute(attr, ptr, len) {
    if (attr === 19) {
      // segmentTemplateObj.timeline = [];byteLength
      // console.warn(ptr, len);
      // console.time("HIH");
      const buff = linearMemory.buffer.slice(ptr, ptr + len);
      segmentTemplateObj.timeline = buff;
      // console.timeEnd("HIH");
      // console.time("HIH2");
      // const buff2 = new Float64Array(linearMemory.buffer, ptr, len / 8);
      // segmentTemplateObj.timeline = buff2;
      // console.timeEnd("HIH2");
      // console.time("HIH3");
      // segmentTemplateObj.timeline = buff2.buffer;
      // console.timeEnd("HIH3");
      // console.log("RES:", linearMemory.buffer.byteLength, buff2.length, buff2.buffer.byteLength);
      // console.log(ptr, ptr + len, buff.byteLength);
      // const dataView = new DataView(linearMemory.buffer);
      // segmentTemplateObj.timeline = [];
      // let base = ptr;
      // for (let i = 0; i < len / 24; i++) {
      //   base += 24;
      //   segmentTemplateObj.timeline.push({
      //     start: dataView.getFloat64(base, true),
      //     duration:  dataView.getFloat64(base + 8, true),
      //     repeatCount:  dataView.getFloat64(base + 16, true),
      //   });
      // }
    }
  };
}
