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
export const bytesToReadable = (bytes: Number) => {
  const units = ['B', 'KiB', 'MiB', 'GiB', 'TiB']
  let size = bytes
  let unitIndex = 0

  while (size >= 1024 && unitIndex < units.length - 1) {
    size /= 1024
    unitIndex++
  }

  return `${size.toFixed(2)} ${units[unitIndex]}`
}
export const getEditionGroupSlug = (editionGroup) => {
  let slug = ''

  if (editionGroup.additional_information?.date_from) {
    slug +=
      new Date(editionGroup.additional_information.date_from).toISOString().split('T')[0] + ' to '
  }
  slug += new Date(editionGroup.release_date).toISOString().split('T')[0]

  if (editionGroup.additional_information?.first_item) {
    slug +=
      ' (' +
      editionGroup.additional_information.first_item +
      ' to ' +
      editionGroup.additional_information.last_item +
      ')'
  }
  slug += ' - ' + editionGroup.name
  slug += ' / ' + editionGroup.source
  slug += editionGroup.distributor ? ' / ' + editionGroup.distributor : ''
  return slug
}
export const getTorrentSlug = (torrent) => {
  let slug = torrent.container

  slug += torrent.video_codec ? ' / ' + torrent.video_codec : ''
  slug += torrent.video_resolution ? ' / ' + torrent.video_resolution : ''
  slug += torrent.audio_codec ? ' / ' + torrent.audio_codec : ''
  slug += torrent.audio_bitrate_sampling ? ' / ' + torrent.audio_bitrate_sampling : ''
  slug += torrent.language && torrent.language != 'English' ? ' / ' + torrent.language : ''
  // slug += torrent.audio_bitrate ? ' / ' + torrent.audio_bitrate : ''
  torrent.features.forEach((feature) => {
    slug += ' / ' + feature
  })
  slug += torrent.release_group ? ' / ' + torrent.release_group : ''

  return slug
}
export const getFeatures = (contentType) => {
  if (contentType == 'Book' || contentType == 'Music') {
    return ['Cue', 'Booklet']
  } else {
    return ['HDR', 'DV', 'Commentary', 'Remux', '3D']
  }
}
