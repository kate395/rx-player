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
import { parseMPDInteger, ValueParser, } from "./utils";
/**
 * Parse an BaseURL element into an BaseURL intermediate
 * representation.
 * @param {Element} adaptationSetElement - The BaseURL root element.
 * @returns {Array.<Object|undefined>}
 */
export default function parseBaseURL(root) {
    var attributes = {};
    var value = root.textContent;
    var warnings = [];
    var parseValue = ValueParser(attributes, warnings);
    if (value === null || value.length === 0) {
        return [undefined, warnings];
    }
    for (var i = 0; i < root.attributes.length; i++) {
        var attribute = root.attributes[i];
        switch (attribute.name) {
            case "availabilityTimeOffset":
                if (attribute.value === "INF") {
                    attributes.availabilityTimeOffset = Infinity;
                }
                else {
                    parseValue(attribute.value, { asKey: "availabilityTimeOffset",
                        parser: parseMPDInteger,
                        dashName: "availabilityTimeOffset" });
                }
                break;
        }
    }
    return [{ value: value, attributes: attributes }, warnings];
}