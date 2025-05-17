'use strict'
Object.defineProperty(exports, '__esModule', { value: true })
exports.DISK_SIZE = exports.VIDEO_OPTION = exports.AUDIO_OPTION = void 0
exports.GB = GB
exports.splitIntoBDInfoSections = splitIntoBDInfoSections
exports.splitIntoSections = splitIntoSections
exports.splitIntoLines = splitIntoLines
exports.extractBBCode = extractBBCode
exports.removeMediainfoTag = removeMediainfoTag
exports.calcDiskType = calcDiskType
var lodash_es_1 = require('lodash-es')
function GB(sizeInBytes) {
  return sizeInBytes * 1024 * 1024 * 1024
}
exports.AUDIO_OPTION = {
  LOSSLESS: 'audio_lossless',
  CHANNEL51: 'audio_51',
  CHANNEL71: 'audio_71',
  DTSX: 'dtsx',
  DOLBYATMOS: 'dolby_atmos',
}
exports.VIDEO_OPTION = {
  BIT10: 'color_10bit',
  HDR10: 'hdr10',
  HDR10PLUS: 'hdr10_plus',
  DOLBYVISION: 'dolby_vision',
}
exports.DISK_SIZE = {
  BD25: GB(23.28),
  BD50: GB(46.57),
  BD66: GB(61.47),
  BD100: GB(93.13),
}
function splitIntoBDInfoSections(text) {
  return (0, lodash_es_1.compact)(
    text
      .trim()
      .replace(/\n\r/, '\n')
      .split(/([a-zA-Z ]*:\n)/)
      .map(function (v) {
        return v.trim()
      }),
  )
}
function splitIntoSections(text) {
  return (0, lodash_es_1.compact)(
    text
      .trim()
      .replace(/\n\r/, '\n')
      .split(/\n\s*\n/)
      .map(function (v) {
        return v.trim()
      }),
  )
}
function splitIntoLines(text) {
  return (0, lodash_es_1.compact)(
    text
      .trim()
      .replace(/\n\r/, '\n')
      .split(/\n/)
      .map(function (v) {
        return v.trim()
      }),
  )
}
function extractBBCode(bbcode) {
  var found = bbcode.match(/\[mediainfo\]([\s\S]*?)\[\/mediainfo\]/i)
  if (!found) {
    return
  }
  return found[1].trim()
}
function removeMediainfoTag(bbcode) {
  return bbcode.replace('[mediainfo]', '').replace('[/mediainfo]', '').trim()
}
function calcDiskType(size) {
  if (size <= exports.DISK_SIZE.BD25) {
    return 'BD25'
  } else if (size <= exports.DISK_SIZE.BD50) {
    return 'BD50'
  } else if (size <= exports.DISK_SIZE.BD66) {
    return 'BD66'
  } else if (size <= exports.DISK_SIZE.BD100) {
    return 'BD100'
  }
}
