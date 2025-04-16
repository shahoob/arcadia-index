import MediainfoParser from './mediainfo/mediainfoParser'
import MediainfoConverter from './mediainfo/mediainfoConverter'
import { removeMediainfoTag } from './utils'

function preProcess(text) {
  const replaces = [
    { from: '\u2002', to: ' ' },
    { from: '\u200d', to: '' },
  ]
  for (const { from, to } of replaces) {
    text = text.replaceAll(from, to)
  }
  return text
}

function getType(text) {
  text = preProcess(text)
  return text.match(/Disc (Title|Label)\s*:/i)
    ? 'bdinfo'
    : text.match(/Complete name\s*:/i)
      ? 'mediainfo'
      : null
}

export const getFileInfo = (text) => {
  text = preProcess(text)
  text = removeMediainfoTag(text)
  const type = getType(text)
  switch (type) {
    case 'mediainfo': {
      const info = new MediainfoParser().parse(text)
      if (!info) {
        return
      }
      const fields = new MediainfoConverter().convert(info)
      return fields
    }
    case 'bdinfo': {
      const info = new BdinfoParser().parse(text)
      if (!info) {
        return
      }
      return new BdinfoConverter().convert(info)
    }
    default:
      console.error('mediainfo unknown type, no Disc Title/Label or Complete name')
      return null
  }
}
