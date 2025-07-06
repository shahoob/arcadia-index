import type { ContentType, EditionGroupInfoLite, Features, Source } from './api/torrentService'

export const timeAgo = (date: string) => {
  const diff = (Date.now() - new Date(date).getTime()) / 1000
  return diff < 60
    ? `${Math.floor(diff)}s ago`
    : diff < 3600
      ? `${Math.floor(diff / 60)}m ago`
      : diff < 86400
        ? `${Math.floor(diff / 3600)}h ago`
        : `${Math.floor(diff / 86400)}d ago`
}
export const formatDate = (dateString: string) => {
  const date = new Date(dateString)
  const time = date.toLocaleTimeString('en-US', {
    hour12: false,
    hour: '2-digit',
    minute: '2-digit',
  })
  return `${date.getDate()} ${date.toLocaleString('default', { month: 'long' })} ${date.getFullYear()}, ${time}`
}
export const bytesToReadable = (bytes: number): string => {
  const units = ['B', 'KiB', 'MiB', 'GiB', 'TiB']
  let size = bytes
  let unitIndex = 0

  while (size >= 1024 && unitIndex < units.length - 1) {
    size /= 1024
    unitIndex++
  }

  return `${size.toFixed(unitIndex === 0 ? 0 : 2)} ${units[unitIndex]}`
}
export const getEditionGroupSlug = (editionGroup: EditionGroupInfoLite): string => {
  const attributes: string[] = []

  let dateRange = ''
  if (editionGroup.additional_information?.date_from) {
    dateRange += new Date(editionGroup.additional_information.date_from).toISOString().split('T')[0] + ' to '
  }
  dateRange += new Date(editionGroup.release_date).toISOString().split('T')[0]

  let itemRange = ''
  if (editionGroup.additional_information?.first_item) {
    itemRange = ` (${editionGroup.additional_information.first_item} to ${editionGroup.additional_information.last_item})`
  }

  if (editionGroup.name) {
    attributes.push(`${dateRange}${itemRange} - ${editionGroup.name}`)
  } else {
    attributes.push(`${dateRange}${itemRange}`)
  }
  if (editionGroup.additional_information?.format) {
    attributes.push(editionGroup.additional_information.format)
  }

  if (editionGroup.additional_information?.label) {
    attributes.push(editionGroup.additional_information.label)
  }
  if (editionGroup.additional_information?.catalogue_number) {
    attributes.push(editionGroup.additional_information.catalogue_number)
  }
  if (editionGroup.additional_information?.isbn_13) {
    attributes.push(editionGroup.additional_information.isbn_13)
  }
  if (editionGroup.source) {
    attributes.push(editionGroup.source)
  }
  if (editionGroup.distributor) {
    attributes.push(editionGroup.distributor)
  }

  return attributes.join(' / ')
}
export const getFeatures = (contentType: string, format: string = '', source: Source | null = null): Features[] => {
  let features: Features[] = []
  if (source === 'Physical Book') {
    features = features.concat(['OCR'])
  }
  if ((contentType == 'book' && format === 'audiobook') || contentType == 'music') {
    features = features.concat(['Cue', 'Booklet'])
  } else if (contentType == 'tv_show' || contentType == 'movie') {
    features.concat(['HDR', 'HDR 10', 'HDR 10+', 'DV', 'Commentary', 'Remux', '3D'])
  }
  return features
}
export const getLanguages = () => {
  return ['English', 'French', 'German', 'Italian', 'Spanish', 'Swedish']
}
export const getPlatforms = () => {
  return ['Linux', 'MacOS', 'Windows']
}
export const getSources = (contentType: ContentType) => {
  const sources = ['Web']
  switch (contentType) {
    case 'book': {
      sources.push('Physical Book', 'CD')
      break
    }
    case 'music': {
      sources.push('Vinyl', 'Blu-Ray', 'CD', 'Soundboard', 'SACD', 'DAT', 'Cassette')
      break
    }
    case 'movie': {
      sources.push('Blu-Ray', 'DVD9', 'DVD5', 'HD-DVD', 'HD-TV', 'PDTV', 'VHS', 'TV', 'LaserDisc')
      break
    }
    case 'tv_show': {
      sources.push('Blu-Ray', 'DVD9', 'DVD5', 'HD-DVD', 'HD-TV', 'PDTV', 'VHS', 'TV', 'LaserDisc')
      break
    }
    case 'collection': {
      sources.push(
        'Blu-Ray',
        'DVD9',
        'DVD5',
        'HD-DVD',
        'HD-TV',
        'PDTV',
        'VHS',
        'TV',
        'LaserDisc',
        'Physical Book',
        'Vinyl',
        'CD',
        'Soundboard',
        'SACD',
        'DAT',
        'Cassette',
      )
      break
    }
  }
  sources.push('Mixed')
  return sources
}
export const getArtistRoles = (contentType: ContentType) => {
  const commonRoles = ['main', 'guest']

  switch (contentType) {
    case 'movie':
    case 'tv_show':
      return [...commonRoles, 'producer', 'director', 'cinematographer', 'actor', 'writer', 'composer']
    case 'video':
      return [...commonRoles, 'producer', 'director', 'cinematographer', 'actor', 'writer', 'composer', 'remixer']
    case 'music':
      return [...commonRoles, 'producer', 'composer', 'conductor', 'dj_compiler', 'remixer', 'arranger', 'writer']
    case 'podcast':
      return [...commonRoles, 'producer', 'writer', 'host']
    case 'book':
      return [...commonRoles, 'author', 'writer', 'illustrator', 'editor']
    case 'software':
      return [...commonRoles, 'developer', 'designer', 'producer', 'writer']
    case 'collection':
      return [...commonRoles, 'producer', 'director', 'composer', 'author', 'writer', 'editor']
    default:
      return commonRoles
  }
}
export const isValidUrl = (url: string) => {
  try {
    new URL(url)
    return true
  } catch {
    return false
  }
}
