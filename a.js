// ## How to use it
//
// The DASH-WASM feature is excitingly fast so we encourage everyone to use it.
//
// However, it needs some preliminary steps:
//   - two external files will need to be loaded and shared with the RxPlayer
//   - this feature has to be added to the RxPlayer
//
// Don't worry, it is relatively straightforward.
// The current chapter will explain everything you need to do an everything you
// should know.
//
//
// ### Quick example code
//
// Let's begin by an heavily commented example of a code adding the DASH-WASM
// feature to the RxPlayer.
//
// It might be a lot to grasp now, we will focus on what has been done here step
// by step.
//
// ```js
// // Import the minimal RxPlayer
// import RxPlayer from "rx-player/minimal";
//
// // Import the `createDashWasmParser` function
// import { createDashWasmParser } from "rx-player/experimental/features";
//
// // Create DASH_WASM parser.
// // Note that this function will immediately request URL(s) that it is given.
// const dashWasmParser = createDashWasmParser({
//   // WebAssembly code. Can be either:
//   //   - The URL to the WebAssembly file. This is the easiest way.
//   //     Note: It is advised to serve that file with a `Content-Type` response
//   //     header set to `application/wasm` to allow potential future
//   //     optimizations.
//   //   - The compiled WebAssembly code as a `WebAssembly.Module` object (output
//   //     from either the `WebAssembly.compileStreaming` or `WebAssembly.compile`
//   //     functions).
//   //     You might want to go that way if you prefer to avoid doing the request
//   //     at this point, either because you would prefer to load it sooner in
//   //     your code or because you use a caching solution (e.g. IndexedDB) to
//   //     avoid the request and compilation step from happening each time.
//   //     Note that in that last scenario, you need to be careful to not share
//   //     a WebAssembly code from a previous RxPlayer version.
//   wasm: "https://example.com/dash-parser.wasm",
//
//   // Worker code. Can be either:
//   //   - The URL to the Worker file.
//   //   - An already-created Worker linked to that file.
//   // Note: Extra-care may have to be put into Content-Security-Policy headers
//   // depending on where this file is loaded from relatively to the current page.
//   worker: "https://example.com/dash-parser.js"
// });
//
// // Add the DASH-WASM parser to the RxPlayer.
// RxPlayer.addFeatures([ dashWasmParser ]);
// ```
//
//
// ### Obtaining DASH-WASM files
//
// The RxPlayer will need to two supplementary files to be able to run the
// DASH-WASM parser:
//
//   - `dash-parser.wasm`: The fast DASH Manifest parser written in WebAssembly.
//
//   - dash-parser.worker.js: The "Worker".
//     This is the JavaScript file which will run in another thread sending and
//     receiving data from the WebAssembly code.
//
// You can find those two files at any of the following places:
//
//   - They are provided at the end of every release note (you should only use
//     the files linked to the RxPlayer's version you're using).
//
//   - They are also available in the `dist/` directory of the project.
//     Those are also published, which mean they might already be loaded in
//     your project, for example in the node_modules directory (most probably in
//     `node_modules/rx-player/dist/` depending on your project)
//
// Once you've retrieved the files linked to your version, you will need to
// either load them yourself to then share them with the RxPlayer or just give
// their new URLs to the RxPlayer which will load them itself.
//
// ---
//
// :warning: __Important note__: Workers and Content-Security-Policy
//
// Under the wrong conditions, the browser could decide to block the worker
// script from being loaded, due to the Content-Security-Policy mechanisms.
//
// To avoid this, it is very important to properly set your application page's
// Content-Security-Policy header as to authorize workers loaded from the URL
// you serve it from.
//
// If you don't know what the Content-Security-Policy header is, you can think
// of it like another CORS-style mechanism that blocks workers from being loaded
// based on their source URLs. A complete documentation is available here: XXX
// TODO
// For example
//
// If the worker script is blocked, the DASH-WASM parser won't be used by the
// RxPlayer.
//
// --
//
// ### Adding the DASH-WASM feature to the RxPlayer
//
// The DASH-WASM feature is an "experimental" feature.
// This is because although the feature is considered as stable, its API may
// still change at any new RxPlayer version (if this happens, changes on its API
// will be explained on our CHANGELOG and this documentation will be updated).
//
// As any experimental features
//
// ```js
// import { createDashWasmParser } from "rx-player/experimental/features";
// ```
//
// For example, if you store them at those two URLs:
//   - https://example.com/dash-parser.wasm
//   - https://example.com/dash-parser.worker.js
//
// You should instantiate the DASH-WASM feature the following way:
// ```js
// import { createDashWasmParser } from "rx-player/experimental/features";
//
// // Create DASH_WASM parser by giving it the URL to the wasm code
// // Note that this operation does not perform any request.
// // Note that this operation will immediately request both the webassembly
// // file and the worker file.
// const dashWasmParser = createDashWasmParser({
//   wasm: "https://example.com/dash-parser.wasm",
//   worker: "https://example.com/dash-parser.js"
// });
// For example, if you store them at those two URLs:
//   - https://example.com/dash-parser.wasm
//   - https://example.com/dash-parser.worker.js
//
// You should instantiate the DASH-WASM feature the following way:
// ```js
// import { createDashWasmParser } from "rx-player/experimental/features";
//
// // Create DASH_WASM parser by giving it the URL to the wasm code
// // Note that this operation does not perform any request.
// // Note that this operation will immediately request both the webassembly
// // file and the worker file.
// const dashWasmParser = createDashWasmParser({
//   wasm: "https://example.com/dash-parser.wasm",
//   worker: "https://example.com/dash-parser.js"
// });

//     _(note that there might be supplementary actions that should be taken
//      depending on where the worker file is stored relatively to your
//      application page. See the important note below for more information.)_
// Import the RxPlayer
//
// ### Example (and summary)
//
// To summarize, let's have an heavily commented example:
// ```js
// // Import the minimal RxPlayer
// import RxPlayer from "rx-player/minimal";
//
// // Import the `createDashWasmParser` function
// import { createDashWasmParser } from "rx-player/experimental/features";
//
// // Create DASH_WASM parser by giving it the related URLs.
// // Note that this operation will immediately request both the webassembly
// // and the worker file.
// const dashWasmParser = createDashWasmParser({
//   // Either the URL to the WebAssembly file or the loaded WebAssembly code
//   // itself as an ArrayBuffer.
//   // Note: It is advised to serve that file with a content-type response
//   // header set to "application/wasm" to allow potential future optimizations
//   wasm: "https://example.com/dash-parser.wasm",
//
//   // Either URL to the Worker file or a Worker linked to that file.
//   // Note: Extra-care may have to be put into Content-Security-Policy headers
//   // depending on where this file is stored relatively to the current page.
//   worker: "https://example.com/dash-parser.js"
// });
//
// // Add the DASH-WASM parser to the RxPlayer.
// //
// // Note that you'll most likely want other features.
// // Those can be added in any order you wish: either in the same `addFeatures`
// // call, or in another one (after or before).
// RxPlayer.addFeatures([ dashWasmParser ]);
//```

// Import the minimal RxPlayer
import RxPlayer from "rx-player/minimal";

// Import the function allowing to create the DASH-WASM parser
import { createDashWasmParser } from "rx-player/experimental/features";

// Create DASH-WASM parser.
// Note that this function will immediately request any URL it is given.
const dashWasmParser = createDashWasmParser({
  // WebAssembly code. Can be either:
  //   - The URL to the WebAssembly file.
  //     This is the easiest way.
  //     Note: It is advised to serve that file with a `Content-Type` response
  //     header set to `application/wasm` to unlock some optimizations.
  //   - The compiled WebAssembly code as a `WebAssembly.Module` object
  //     (outputed from either the `WebAssembly.compileStreaming` or the
  //     `WebAssembly.compile` functions).
  //     You might want to go that way if you prefer to avoid a request here.
  wasm: "https://example.com/dash-parser.wasm",

  // Worker code. Can be either:
  //   - The URL to the Worker file.
  //   - An already-created Worker linked to that file.
  // Note: Extra-care may have to be put into Content-Security-Policy headers
  // depending on where this file is loaded from relatively to the current page.
  worker: "https://example.com/dash-parser.js"
});

// Add the DASH-WASM parser to the RxPlayer.
RxPlayer.addFeatures([ dashWasmParser ]);
