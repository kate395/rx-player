/**
 * Copyright 2015 CANAL+ Group
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

import pinkie from "pinkie";
import {
  EMPTY,
  merge as observableMerge,
  race as observableRace,
  Subject,
} from "rxjs";
import {
  catchError,
  filter,
  finalize,
  ignoreElements,
  map,
  mergeMap,
  take,
} from "rxjs/operators";
import Player from "../../../core/api";
import createSegmentLoader from "../../../core/fetchers/segment/create_segment_loader";
import log from "../../../log";
import { ISegment } from "../../../manifest";
import getContentInfos from "./get_content_infos";
import {
  disposeSourceBuffer,
  initSourceBuffer$,
} from "./init_source_buffer";
import removeBufferAroundTime$ from "./remove_buffer_around_time";
import {
  IContentInfos,
  ILoaders,
} from "./types";
import VideoThumbnailLoaderError from "./video_thumbnail_loader_error";

const PPromise = typeof Promise === "function" ? Promise :
                                                 pinkie;

interface IJob { contentInfos: IContentInfos;
                 segment: ISegment;
                 stop: () => void;
                 jobPromise: Promise<unknown>; }

/**
 * This tool, as a supplement to the RxPlayer, intent to help creating thumbnails
 * from a video source.
 *
 * The tools will extract a "thumbnail track" either from a video track (whose light
 * chunks are adapted from such use case) or direclty from the media content.
 */
export default class VideoThumbnailLoader {
  private readonly _videoElement: HTMLVideoElement;

  private _player: Player;
  private _currentJob?: IJob;
  private _loaders: ILoaders = {};

  constructor(videoElement: HTMLVideoElement,
              player: Player) {
    this._videoElement = videoElement;
    this._player = player;
  }

  /**
   * Add imported loader to thumbnail loader loader object.
   * It allows to use it when setting time.
   * @param {function} loaderFunc
   */
  addLoader(loaderFunc: (features: ILoaders) => void): void {
    loaderFunc(this._loaders);
  }

  /**
   * Set time of thumbnail video media element :
   * - Remove buffer when too much buffered data
   * - Search for thumbnail track element to display
   * - Load data
   * - Append data
   * Resolves when time is set.
   * @param {number} time
   * @returns {Promise}
   */
  setTime(time: number): Promise<unknown> {
    for (let i = 0; i < this._videoElement.buffered.length; i++) {
      if (this._videoElement.buffered.start(i) <= time &&
          this._videoElement.buffered.end(i) >= time) {
        this._videoElement.currentTime = time;
        log.debug("VTL: Thumbnail already loaded.", time);
        return PPromise.resolve(time);
      }
    }

    const manifest = this._player.getManifest();
    if (manifest === null) {
      return PPromise.reject(
        new VideoThumbnailLoaderError("NO_MANIFEST",
                                      "No manifest available."));
    }
    const contentInfos = getContentInfos(time, manifest);
    if (contentInfos === null) {
      return PPromise.reject(
        new VideoThumbnailLoaderError("NO_TRACK",
                                      "Couldn't find track for this time."));
    }
    const segment = contentInfos.representation.index.getSegments(time, 10)[0];
    if (segment === undefined) {
      return PPromise.reject(
        new VideoThumbnailLoaderError("NO_THUMBNAIL",
                                      "Couldn't find thumbnail."));
    }

    log.debug("VTL: Found thumbnail for time", time, segment);

    if (this._currentJob !== undefined &&
        this._currentJob.contentInfos.representation.id ===
          contentInfos.representation.id &&
        this._currentJob.contentInfos.adaptation.id ===
          contentInfos.adaptation.id &&
        this._currentJob.contentInfos.period.id ===
          contentInfos.period.id &&
        this._currentJob.contentInfos.manifest.id ===
          contentInfos.manifest.id &&
        this._currentJob.segment.id === segment.id) {
      // The current job is already handling the loading for the wanted time
      // (same thumbnail).
      return this._currentJob.jobPromise;
    }
    this._currentJob?.stop();
    return this.startJob(contentInfos, time, segment);
  }

  /**
   * Dispose thumbnail loader.
   * @returns {void}
   */
  dispose(): void {
    this._currentJob?.stop();
    disposeSourceBuffer(this._videoElement);
  }

  private startJob(contentInfos: IContentInfos,
                   time: number,
                   segment: ISegment
  ): Promise<unknown> {
    const loader = this._loaders[contentInfos.manifest.transport];
    if (loader === undefined) {
      const error =
        new VideoThumbnailLoaderError("NO_LOADER",
                                      "VideoThumbnailLoaderError: No " +
                                      "imported loader for this transport type: " +
                                      contentInfos.manifest.transport);
      return PPromise.reject(error);
    }
    const killJob$ = new Subject();
    const abortError$ = killJob$.pipe(
      map(() => {
        throw new VideoThumbnailLoaderError("ABORTED",
                                            "VideoThumbnailLoaderError: Aborted job.");
      })
    );

    const segmentLoader = createSegmentLoader(
      loader.video.loader,
      undefined,
      { baseDelay: 0,
        maxDelay: 0,
        maxRetryOffline: 0,
        maxRetryRegular: 0 }
    );
    const { parser: segmentParser } = loader.video;


    const jobPromise = observableRace(
      abortError$,
      initSourceBuffer$(contentInfos,
                        this._videoElement,
                        { segmentLoader,
                          segmentParser }).pipe(
        mergeMap((videoSourceBuffer) => {
          const bufferCleaning$ =
            removeBufferAroundTime$(this._videoElement, videoSourceBuffer, time);
          log.debug("VTL: Removed buffer before appending segments.", time);

          const segmentLoading$ = segmentLoader({
            manifest: contentInfos.manifest,
            period: contentInfos.period,
            adaptation: contentInfos.adaptation,
            representation: contentInfos.representation,
            segment,
          });

          return observableMerge(
            bufferCleaning$.pipe(ignoreElements()),
            segmentLoading$
          ).pipe(
            filter((evt): evt is { type: "data";
                                   value: { responseData: Uint8Array }; } =>
              evt.type === "data"),
            mergeMap((evt) => {
              const inventoryInfos = { manifest: contentInfos.manifest,
                                       period: contentInfos.period,
                                       adaptation: contentInfos.adaptation,
                                       representation: contentInfos.representation,
                                       segment,
                                       start: segment.time / segment.timescale,
                                       end: (segment.time + segment.duration) /
                                         segment.timescale };
              return segmentParser({
                response: {
                  data: evt.value.responseData,
                  isChunked: false,
                },
                content: inventoryInfos,
              }).pipe(
                mergeMap((parserEvt) => {
                  if (parserEvt.type !== "parsed-segment") {
                    return EMPTY;
                  }
                  const { chunkData, appendWindow } = parserEvt.value;
                  const segmentData = chunkData instanceof ArrayBuffer ?
                    new Uint8Array(chunkData) : chunkData;
                  return videoSourceBuffer
                    .pushChunk({ data: { chunk: segmentData,
                                         timestampOffset: 0,
                                         appendWindow,
                                         initSegment: null,
                                         codec: contentInfos
                                           .representation.getMimeTypeString() },
                                 inventoryInfos })
                    .pipe(map(() => {
                      log.debug("VTL: Appended segment.", evt.value.responseData);
                      this._videoElement.currentTime = time;
                      return time;
                    }));
                })
              );
            })
          );
        }),
        catchError((err: unknown) => {
          let message = "Unknown error.";
          if ((err as { message?: string; toString(): string }).message !== undefined ||
            /* eslint-disable @typescript-eslint/no-unsafe-assignment */
            /* eslint-disable @typescript-eslint/no-unsafe-member-access */
            /* eslint-disable @typescript-eslint/no-unsafe-call */
              typeof (err as { message?: string;
                               toString(): string; }).toString === "function") {
            message = (err as any).message ?? (err as any).toString();
            /* eslint-enable @typescript-eslint/no-unsafe-assignment */
            /* eslint-enable @typescript-eslint/no-unsafe-member-access */
            /* eslint-enable @typescript-eslint/no-unsafe-call */
          }
          throw new VideoThumbnailLoaderError("LOADING_ERROR", message);
        })
      )
    ).pipe(
      take(1),
      finalize(() => {
        this._currentJob = undefined;
        killJob$.complete();
      })
    ).toPromise(PPromise);

    this._currentJob = {
      contentInfos,
      segment,
      stop: () => {
        killJob$.next();
        killJob$.complete();
      },
      jobPromise,
    };

    return jobPromise;
  }
}

export { default as DASH_LOADER } from "./features/dash";
export { default as MPL_LOADER } from "./features/metaplaylist";
