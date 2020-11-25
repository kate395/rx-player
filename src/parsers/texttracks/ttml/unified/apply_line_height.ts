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

import log from "../../../../../log";
import { REGXP_LENGTH } from "../../regexps";
import { IIntermediateParagraphStyle } from "./types";

/**
 * @param {HTMLElement} element
 * @param {string} lineHeight
 */
export default function applyLineHeight(
  paragraphStyle : IIntermediateParagraphStyle,
  lineHeight : string
) : void {
  const trimmedLineHeight = lineHeight.trim();
  if (trimmedLineHeight === "auto") {
    return;
  }
  const firstLineHeight = REGXP_LENGTH.exec(trimmedLineHeight[0]);
  if (firstLineHeight === null) {
    return;
  }
  if (firstLineHeight[2] === "px" ||
      firstLineHeight[2] === "%" ||
      firstLineHeight[2] === "em")
  {
    paragraphStyle.lineHeight = {
      isProportional: false,
      value: firstLineHeight[1] + firstLineHeight[2],
    };
  } else if (firstLineHeight[2] === "c") {
    paragraphStyle.lineHeight = {
      isProportional: true,
      value: firstLineHeight[1],
    }
  } else {
    log.warn("TTML Parser: unhandled lineHeight unit:", firstLineHeight[2]);
  }
}
