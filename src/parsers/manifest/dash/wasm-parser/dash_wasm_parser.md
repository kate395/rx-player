# Fast and non UI-blocking DASH MPD parsing with the DASH-WASM parser

The RxPlayer provides two types of parsers for DASH MPD files (its Manifest):
  1. A JavaScript parser. Provided in the default build or through the `DASH`
     feature in the minimal build.
  2. A generally-faster WebAssembly parser running in parallel of the user
     interface, thus avoiding to block the UI while doing so. Only provided
     through the `createDashWasmParser` experimental feature in the minimal
     build.

This page will document everything you need to know about that second parser:
  - In which case would you need it.
  - How you can use it.
  - How it works.
  - How this feature interacts with the other RxPlayer features.


## Do I need this?

When playing DASH contents, parsing its MPD file is often the most expensive
operation in terms of performance.


## How to use it?

The DASH-WASM feature is excitingly fast so we encourage everyone to use it.

However, it needs some preliminary steps:
  - two external files will need to be loaded and shared with the RxPlayer
  - this feature has to be added to the RxPlayer

Don't worry, it is relatively straightforward.
The current chapter will explain everything you need to do an everything you
should know.


### Quick code example

Let's begin by an heavily commented example of a code adding the DASH-WASM
feature to the RxPlayer.

It might be a lot to grasp now, we will focus on what has been done here step
by step in the next chapters.

```js
// Import the minimal RxPlayer
import RxPlayer from "rx-player/minimal";

// Import the function allowing to create the DASH-WASM parser
import { createDashWasmParser } from "rx-player/experimental/features";

// Create DASH-WASM parser.
// Note that this function will immediately request any URL it is given.
const dashWasmParser = createDashWasmParser({
  // WebAssembly code. Can be either:
  //   - The URL to the WebAssembly file.
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
```


### Step 1: Obtaining the DASH-WASM files

The RxPlayer will need two supplementary files to be able to run the DASH-WASM
parser:

  - `dash-parser.wasm`: The fast DASH Manifest parser written in WebAssembly.

  - `dash-parser.worker.js`: The "Worker".
    This is the JavaScript file which will run in another thread sending and
    receiving data from the WebAssembly code.

You can find those two files at any of the following places:

  - They are provided at the end of every release note (you should only use
    the files linked to the RxPlayer's version you're using).

  - They are also available in the `dist/` directory of the project.
    Those are also published, which mean they might already be loaded in
    your project, for example in the node_modules directory (most probably in
    `node_modules/rx-player/dist/` depending on your project)
    `

Once you've retrieved the files linked to your version, you will need to
either give their new URLs to the RxPlayer so it can load them or even load them
yourself to then share them with the player.


### Step 2: importing the `createDashWasmParser` feature

The DASH-WASM feature is an "experimental" feature.
This is because although the feature is considered stable, its API may still
change at any new RxPlayer version (if this happens, changes on its API will be
explained on our CHANGELOG and this documentation will be updated).

As any experimental features, it needs to be imported through the
`rx-player/experimental/features` path:
```js
import { createDashWasmParser } from "rx-player/experimental/features";
```

### Step 3: Giving to `createDashWasmParser` the DASH-WASM files

We will now need to call `createDashWasmParser` with the two files obtained in
step 1 as arguments.

They are two ways to do that for each file. The most simple way is to just give
it the URLs of these two files.

For example, if you store them at those two URLs:
  - https://example.com/dash-parser.wasm
  - https://example.com/dash-parser.worker.js

You should instantiate the DASH-WASM feature the following way:
```js
import { createDashWasmParser } from "rx-player/experimental/features";

const dashWasmParser = createDashWasmParser({
  wasm: "https://example.com/dash-parser.wasm",
  worker: "https://example.com/dash-parser.worker.js"
});
```

That's it!

An important thing to consider is that `createDashWasmParser` will immediately
request both files. This may or may not be something you want, which is why
there are other ways to share each of those two files.

First, for the WebAssembly file (`dash-parser.wasm`),

// ...


### Step 4: Adding the feature to the RxPlayer

The last step is also the easiest, you now just need to add the Rx

```js
RxPlayer.addFeatures([ dashWasmParser ]);
```


## How does it work?

This chapter will dive into the inner working of the DASH-WASM parser. It is not
required to know about this, this chapter is just for information purposes for
the more curious readers.

Let's begin by a simple schema representing broadly the different steps when
the DASH-WASM parser is used:

```
                                             ( CDN )
 The internet                                   ^
- - - - - - - - - - - - - - - - - - - - - - - - | - - - - - - - - - - - - - - -
 RxPlayer                                       |
              (2) The Worker requests the MPD > |
                                                |
                            +---------------+--------+
 (3) The Worker gives the > |  WebAssembly  |        |
     MPD to the WebAssembly +---------------+        |
     which parses it.       |          Worker        |
                            +------------------------+
                                  ^         |
                                  |         |  < (4) The worker translates the
                                  |         |        result into a JavaScript
     (1) The RxPlayer asks that > |         |        Object and sends it to the
         the MPD should be loaded |         |        RxPlayer.
         or refreshed             |         |
                                  |         V
                    +--------------------------------------+
                    |                                      |
                    |                                      |
                    |          main RxPlayer Code          |
                    |                                      |
                    |                                      |
                    +--------------------------------------+
```


## What happens if both the `DASH` and DASH-WASM features are added?
