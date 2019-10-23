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

import isNonEmptyString from "../../../utils/is_non_empty_string";

export interface ITTParameters { frameRate : number;
                                 subFrameRate : number;
                                 tickRate : number;
                                 spaceStyle: "default" | "preserve"; }

/**
 * Returns global parameters from a TTML Document
 * @param {Element} tt - <tt> node
 * @throws Error - Throws if the spacing style is invalid.
 * @returns {Object}
 */
export default function getParameters(tt : Element) : ITTParameters {
  const parsedFrameRate = tt.getAttribute("ttp:frameRate");
  const parsedSubFrameRate = tt.getAttribute("ttp:subFramRate");
  const parsedTickRate = tt.getAttribute("ttp:tickRate");
  const parsedFrameRateMultiplier = tt.getAttribute("ttp:frameRateMultiplier");
  const parsedSpaceStyle = tt.getAttribute("xml:space");

  if (isNonEmptyString(parsedSpaceStyle) &&
      parsedSpaceStyle !== "default" &&
      parsedSpaceStyle !== "preserve")
  {
    throw new Error("Invalid spacing style");
  }

  let nbFrameRate = Number(parsedFrameRate);
  if (isNaN(nbFrameRate)) {
    nbFrameRate = 30;
  }
  let nbSubFrameRate = Number(parsedSubFrameRate);
  if (isNaN(nbSubFrameRate)) {
    nbSubFrameRate = 1;
  }
  let nbTickRate = Number(parsedTickRate);
  if (isNaN(nbTickRate)) {
    nbTickRate = 0;
  }

  let tickRate = nbTickRate;
  let frameRate = nbFrameRate;
  const subFrameRate = nbSubFrameRate != null ? nbSubFrameRate :
                                                1;

  const spaceStyle = parsedSpaceStyle !== null ? parsedSpaceStyle :
                                                 "default";

  if (nbTickRate === 0) {
    tickRate = isNaN(nbFrameRate) || nbFrameRate === 0 ?
      1 :
      nbFrameRate * nbSubFrameRate;
  }

  if (parsedFrameRateMultiplier  !== null) {
    const multiplierResults = /^(\d+) (\d+)$/g.exec(parsedFrameRateMultiplier);
    if (multiplierResults !== null) {
      const numerator = Number(multiplierResults[1]);
      const denominator = Number(multiplierResults[2]);
      const multiplierNum = numerator / denominator;
      frameRate = nbFrameRate * multiplierNum;
    }
  }

  return { tickRate,
           frameRate,
           subFrameRate,
           spaceStyle };
}
