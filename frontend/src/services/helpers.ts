import type { ContentType, EditionGroupInfoLite } from './api/torrentService'

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

  attributes.push(`${dateRange}${itemRange} - ${editionGroup.name}`)
  if (editionGroup.additional_information?.format) {
    attributes.push(editionGroup.additional_information.format)
  }

  if (editionGroup.additional_information?.label) {
    attributes.push(editionGroup.additional_information.label)
  }
  if (editionGroup.additional_information?.catalogue_number) {
    attributes.push(editionGroup.additional_information.catalogue_number)
  }
  if (editionGroup.source) {
    attributes.push(editionGroup.source)
  }
  if (editionGroup.distributor) {
    attributes.push(editionGroup.distributor)
  }

  return attributes.join(' / ')
}
export const getFeatures = (contentType: string): string[] => {
  if (contentType == 'book' || contentType == 'music') {
    return ['Cue', 'Booklet']
  } else if (contentType == 'tv_show' || contentType == 'movie') {
    return ['HDR', 'HDR 10', 'HDR 10+', 'DV', 'Commentary', 'Remux', '3D']
  } else {
    return []
  }
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
export const getArtistRoles = () => {
  return ['main', 'producer', 'guest', 'composer', 'conductor', 'dj_compiler', 'remixer', 'arranger', 'director', 'cinematographer', 'actor', 'author']
}
export const isValidUrl = (url: string) => {
  try {
    new URL(url)
    return true
  } catch {
    return false
  }
}
