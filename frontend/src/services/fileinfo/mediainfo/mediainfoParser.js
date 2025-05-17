"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
var debug_1 = require("debug");
var utils_js_1 = require("../utils.js");
var debug = (0, debug_1.default)('mediainfo');
/*
 * new Mediainfo.parse(rawText)
 * #-> { general: { KEY: VALUE }, video: [..], .. }
 */
var MediainfoParser = /** @class */ (function () {
    function MediainfoParser() {
    }
    MediainfoParser.prototype.parse = function (text) {
        var _a, _b;
        var result = {
            general: {},
            video: [],
            audio: [],
            text: [],
            menu: {},
        };
        var sections = (0, utils_js_1.splitIntoSections)(text);
        debug('SECTIONS', sections);
        for (var _i = 0, sections_1 = sections; _i < sections_1.length; _i++) {
            var section = sections_1[_i];
            if (!section.match(/([a-zA-Z]+).*\n([\s\S]+)/)) {
                continue;
            }
            var _c = (_b = (_a = section.match(/([a-zA-Z]+).*\n([\s\S]+)/)) === null || _a === void 0 ? void 0 : _a.slice(1)) !== null && _b !== void 0 ? _b : [], rawSectionName = _c[0], sectionBody = _c[1];
            var sectionName = rawSectionName.toLowerCase();
            var fields = this.extractFields({ sectionName: sectionName, sectionBody: sectionBody });
            if (Array.isArray(result[sectionName])) {
                result[sectionName].push(fields);
            }
            else {
                // @ts-expect-error TODO: type isn't narrowing - why?
                result[sectionName] = fields;
            }
        }
        return result;
    };
    MediainfoParser.prototype.extractFields = function (_a) {
        var _sectionName = _a.sectionName, sectionBody = _a.sectionBody;
        var result = {};
        var lines = (0, utils_js_1.splitIntoLines)(sectionBody);
        for (var _i = 0, lines_1 = lines; _i < lines_1.length; _i++) {
            var line = lines_1[_i];
            var found = line.match(/^(.*?):(.*)$/);
            if (!found) {
                continue;
            }
            var _b = found.slice(1).map(function (v) { return v.trim(); }), key = _b[0], value = _b[1];
            result[key.toLowerCase()] = value;
        }
        return result;
    };
    return MediainfoParser;
}());
exports.default = MediainfoParser;
