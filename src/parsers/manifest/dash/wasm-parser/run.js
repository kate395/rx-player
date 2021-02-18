const tags = [
  "MPD",
  "BaseURL",
  "SegmentTemplate",
  "SegmentBase",
  "SegmentList",
  "Location",
  "Period",
  "UtcTiming",
  "AdaptationSet",
  "EventStream",
  "EventStreamElt",
  "Representation",
  "Accessibility",
  "ContentComponent",
  "ContentProtection",
  "EssentialProperty",
  "Role",
  "SupplementalProperty",
  "Initialization",
  "SegmentTemplate",
];

const ATTRIBUTES = {
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

  // SegmentTemplate
  SElement: 19, // SElement

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

  Text: 64
};
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

function runProgram(url) {
  parsedS.length = 0;
  let linearMemory;
  let globalMpd;
  let currentPos = 0;
  const parsersStack = [];
  const MPD = {
    periods: [],
    segmentTemplates: []
  };
  window.MPD = MPD;

  const parserState = new ParserState();
  let lastPeriod = null;
  let lastAdaptationSet = null;

  function onTagOpen(tag) {
    switch (tags[tag]) {
      case "Representation":
        if (lastAdaptationSet === null) {
          // TODO warning
        } else {
          const representationObj = {};
          lastAdaptationSet.representations.push(representationObj);
          // currentObj = periodObj;
          // parserState.setNewParser(generateAdaptationSetAttrParser(adaptationObj,
          //                                                          linearMemory));
        }
        break;
      case "AdaptationSet":
        if (lastPeriod === null) {
          // TODO warning
        } else {
          const adaptationObj = { representations: [] };
          lastPeriod.adaptations.push(adaptationObj);
          currentObj = adaptationObj;
          lastAdaptationSet = adaptationObj;
          parserState.setNewParser(generateAdaptationSetAttrParser(adaptationObj,
                                                                   linearMemory));
        }
        break;
      case "Period":
        const periodObj = { adaptations: [] };
        currentObj = periodObj;
        lastPeriod = periodObj;
        MPD.periods.push(periodObj);
        parserState.setNewParser(generatePeriodAttrParser(periodObj, linearMemory));
        break;
      case "MPD":
        currentObj = MPD;
        parserState.setNewParser(generateMPDAttrParser(MPD, linearMemory));
        break;
      case "SegmentTemplate":
        const segmentTemplateObj = {};
        window.MPD.segmentTemplates.push(segmentTemplateObj);
        parserState.setNewParser(generateSegmentTemplateAttrParser(segmentTemplateObj,
                                                                   linearMemory));
        break;
    }
  }

  function onTagClose(tag) {
    switch (tags[tag]) {
      case "MPD":
        parsersStack.pop();
        currentAttributeParser = parsersStack[parsersStack.length - 1];
        break;
      case "Period":
        parsersStack.pop();
        currentAttributeParser = parsersStack[parsersStack.length - 1];
        break;
      case "SegmentTemplate":
        parsersStack.pop();
        currentAttributeParser = parsersStack[parsersStack.length - 1];
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

  return fetch("./output_max.wasm")
  // return fetch("./a.wasm")
    .then(res => res.arrayBuffer())
    .then(resAb => WebAssembly.instantiate(resAb, imports))
  // return WebAssembly.instantiateStreaming(fetch("./mpd_node_parser.wasm", imports))
    .then(resWasm => {
      return fetch(url)
        .then(res => res.arrayBuffer())
        .then(mpd => {
          linearMemory = resWasm.instance.exports.memory;
          dataView = new DataView(linearMemory.buffer);
          globalMpd = mpd;
          let now = performance.now();
          resWasm.instance.exports.parse();
          console.log("TIME: ", performance.now() - now);
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
window.run = (url) => runProgram(url).catch(e => console.log(e));
window.parsedS = parsedS;

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
  return function onMPDAttribute(attr, ptr, len) {
    switch (attr) {
      case ATTRIBUTES.Id:
        mpdObj.id = parseString(linearMemory.buffer, ptr, len);
        break;
      case ATTRIBUTES.Profiles:
        mpdObj.profiles = parseString(linearMemory.buffer, ptr, len);
        break;
      case ATTRIBUTES.Type:
        mpdObj.type = parseString(linearMemory.buffer, ptr, len);
        break;
      case ATTRIBUTES.AvailabilityStartTime:
        mpdObj.availabilityStartTime = parseString(linearMemory.buffer, ptr, len);
        break;
      case ATTRIBUTES.AvailabilityEndTime:
        mpdObj.availabilityEndTime = parseString(linearMemory.buffer, ptr, len);
        break;
      case ATTRIBUTES.PublishTime:
        mpdObj.publishTime = parseString(linearMemory.buffer, ptr, len);
        break;
      case ATTRIBUTES.Duration:
        mpdObj.duration = parseString(linearMemory.buffer, ptr, len);
      case ATTRIBUTES.MinimumUpdatePeriod:
        mpdObj.minimumUpdatePeriod = parseString(linearMemory.buffer, ptr, len);
        break;
      case ATTRIBUTES.MinBufferTime:
        mpdObj.minimumUpdatePeriod = parseString(linearMemory.buffer, ptr, len);
        break;
      case ATTRIBUTES.TimeShiftBufferDepth:
        mpdObj.timeShiftBufferDepth = parseString(linearMemory.buffer, ptr, len);
        break;
      case ATTRIBUTES.SuggestedPresentationDelay:
        mpdObj.suggestedPresentationDelay = parseString(linearMemory.buffer, ptr, len);
        break;
      case ATTRIBUTES.MaxSegmentDuration:
        mpdObj.maxSegmentDuration = parseString(linearMemory.buffer, ptr, len);
        break;
      case ATTRIBUTES.MaxSubsegmentDuration:
        mpdObj.maxSubsegmentDuration = parseString(linearMemory.buffer, ptr, len);
        break;
    }
  };
}

function generatePeriodAttrParser(periodObj, linearMemory) {
  return function onPeriodAttribute(attr, ptr, len) {
    switch (attr) {
      case ATTRIBUTES.Id:
        periodObj.id = parseString(linearMemory.buffer, ptr, len);
        break;
      case ATTRIBUTES.Start:
        periodObj.start = parseString(linearMemory.buffer, ptr, len);
        break;
      case ATTRIBUTES.Duration:
        periodObj.duration = parseString(linearMemory.buffer, ptr, len);
        break;
      case ATTRIBUTES.BitStreamSwitching:
        periodObj.bitstreamSwitching = parseString(linearMemory.buffer, ptr, len);
        break;
      case ATTRIBUTES.XLinkHref:
        periodObj.xlinkHref = parseString(linearMemory.buffer, ptr, len);
        break;
      case ATTRIBUTES.xlinkActuate:
        periodObj.xlinkActuate = parseString(linearMemory.buffer, ptr, len);
        break;
    }
  };
}

function generateAdaptationSetAttrParser(adaptationObj, linearMemory) {
  return function onAdaptationSetAttribute(attr, ptr, len) {
    const dataView = new DataView(linearMemory.buffer);
    switch (attr) {
      case ATTRIBUTES.Id:
        adaptationObj.id = parseString(linearMemory.buffer, ptr, len);
        break;
      case ATTRIBUTES.Group:
        adaptationObj.group = dataView.getFloat64(ptr, true);
        break;
      case ATTRIBUTES.Language:
        adaptationObj.language = parseString(linearMemory.buffer, ptr, len);
        break;
      case ATTRIBUTES.ContentType:
        adaptationObj.contentType = parseString(linearMemory.buffer, ptr, len);
        break;
      case ATTRIBUTES.Par:
        adaptationObj.par = parseString(linearMemory.buffer, ptr, len);
        break;
      case ATTRIBUTES.MinBitrate:
        adaptationObj.MinBitrate = dataView.getFloat64(ptr, true);
        break;
      case ATTRIBUTES.MaxBitrate:
        adaptationObj.MaxBitrate = dataView.getFloat64(ptr, true);
        break;
      case ATTRIBUTES.MinWidth:
        adaptationObj.MinWidth = dataView.getFloat64(ptr, true);
        break;
      case ATTRIBUTES.MaxWidth:
        adaptationObj.MaxWidth = dataView.getFloat64(ptr, true);
        break;
      case ATTRIBUTES.MinHeight:
        adaptationObj.MinHeight = dataView.getFloat64(ptr, true);
        break;
      case ATTRIBUTES.MaxHeight:
        adaptationObj.MaxHeight = dataView.getFloat64(ptr, true);
        break;
      case ATTRIBUTES.MinFrameRate:
        adaptationObj.minFrameRate = parseString(linearMemory.buffer, ptr, len);
        break;
      case ATTRIBUTES.MaxFrameRate:
        adaptationObj.maxFrameRate = parseString(linearMemory.buffer, ptr, len);
        break;
      case ATTRIBUTES.SelectionPriority:
        adaptationObj.selectionPriority = dataView.getFloat64(ptr, true);
        break;
      case ATTRIBUTES.SegmentAlignment:
        adaptationObj.segmentAlignment = parseFloatOrBool(dataView.getFloat64(ptr, true));
        break;
      case ATTRIBUTES.SubsegmentAlignment:
        adaptationObj.segmentAlignment = parseFloatOrBool(dataView.getFloat64(ptr, true));
        break;
      case ATTRIBUTES.BitstreamSwitching:
        adaptationObj.bitstreamSwitching = dataView.getFloat64(ptr, true) !== 0;
        break;
      case ATTRIBUTES.AudioSamplingRate:
        adaptationObj.audioSamplingRate = parseString(linearMemory.buffer, ptr, len);
        break;
      case ATTRIBUTES.Codecs:
        adaptationObj.codecs = parseString(linearMemory.buffer, ptr, len);
        break;
      case ATTRIBUTES.Profiles:
        adaptationObj.profiles = parseString(linearMemory.buffer, ptr, len);
        break;
      case ATTRIBUTES.SegmentProfiles:
        adaptationObj.segmentProfiles = parseString(linearMemory.buffer, ptr, len);
        break;
      case ATTRIBUTES.MimeType:
        adaptationObj.mimeType = parseString(linearMemory.buffer, ptr, len);
        break;
      case ATTRIBUTES.CodingDependency:
        adaptationObj.codingDependency = dataView.getFloat64(ptr, true) !== 0;
        break;
      case ATTRIBUTES.FrameRate:
        adaptationObj.frameRate = parseString(linearMemory.buffer, ptr, len);
        break;
      case ATTRIBUTES.Height:
        adaptationObj.height = dataView.getFloat64(ptr, true);
        break;
      case ATTRIBUTES.Width:
        adaptationObj.width = dataView.getFloat64(ptr, true);
        break;
      case ATTRIBUTES.MaxPlayoutRate:
        adaptationObj.maxPlayoutRate = dataView.getFloat64(ptr, true);
        break;
      case ATTRIBUTES.MaximumSAPPeriod:
        adaptationObj.maxSAPPeriod = dataView.getFloat64(ptr, true);
        break;
    }
  };
}

window.tot = 0;
function generateSegmentTemplateAttrParser(segmentTemplateObj, linearMemory) {
  return function onSegmentTemplateAttribute(attr, ptr, len) {
    console.log(attr);
    const dataView = new DataView(linearMemory.buffer);
    if (attr === 19) {
      const now = performance.now();
      console.time("TOTO");
      segmentTemplateObj.timeline = [];
      let base = ptr;
      for (let i = 0; i < len / 24; i++) {
        base += 24;
        segmentTemplateObj.timeline.push({
          start: dataView.getFloat64(base, true),
          duration:  dataView.getFloat64(base + 8, true),
          repeatCount:  dataView.getFloat64(base + 16, true),
        });
      }
      const time = performance.now() - now;
      console.error("!!!!", time);
      console.timeEnd("TOTO");
      window.tot += time;
    }
  };
}
