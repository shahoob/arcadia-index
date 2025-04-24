import type { components } from '@/api-schema/schema'
import api from './api.ts'

export type TitleGroupHierarchy = components['schemas']['TitleGroupHierarchy']

export type TitleGroupLite = components['schemas']['TitleGroupLite']

export type TitleGroupAndAssociatedData = components['schemas']['TitleGroupAndAssociatedData']

export const getTitleGroup = async (id: number): Promise<TitleGroupAndAssociatedData> => {
  try {
    return (await api.get<TitleGroupAndAssociatedData>('/title-group?id=' + id)).data
  } catch (error) {
    console.error('API Error:', error)
    throw error
  }
}

export type TitleGroupInfoLite = components['schemas']['TitleGroupInfoLite']

export const getTitleGroupLite = async (id: number): Promise<TitleGroupInfoLite> => {
  try {
    return (await api.get<TitleGroupInfoLite>('/title-group/lite?id=' + id)).data
  } catch (error) {
    console.error('API Error:', error)
    throw error
  }
}

export type UserCreatedTitleGroup = components['schemas']['UserCreatedTitleGroup']

export const createTitleGroup = async (titleGroup: UserCreatedTitleGroup) => {
  try {
    titleGroup.screenshots = titleGroup.screenshots.filter((screenshot) => screenshot.trim() !== '')
    return (await api.post('/title-group', titleGroup)).data
  } catch (error) {
    console.error('API Error:', error)
    throw error
  }
}

export type UserCreatedEditionGroup = components['schemas']['UserCreatedEditionGroup']

export const createEditionGroup = async (editionGroup: UserCreatedEditionGroup) => {
  try {
    editionGroup.additional_information = Object.fromEntries(
      Object.entries(editionGroup.additional_information).filter(
        ([, value]) => value !== null && value !== '',
      ),
    )
    editionGroup.covers = editionGroup.covers.filter((cover) => cover.trim() !== '')
    editionGroup.external_links = editionGroup.external_links.filter((link) => link.trim() !== '')
    editionGroup.distributor = editionGroup.distributor == '' ? null : editionGroup.distributor
    return (await api.post('/edition-group', editionGroup)).data
  } catch (error) {
    console.error('API Error:', error)
    throw error
  }
}

// TODO: type uploadTorrent.

export const uploadTorrent = async (torrentForm: object) => {
  try {
    const formData = new FormData()
    for (const [key, value] of Object.entries(torrentForm)) {
      if (value != null) {
        formData.append(key, value)
      }
    }
    return (
      await api.post('/torrent', formData, {
        headers: {
          'Content-Type': 'multipart/form-data',
        },
      })
    ).data
  } catch (error) {
    console.error('API Error:', error)
    throw error
  }
}

export const downloadTorrent = async (torrentId: number) => {
  try {
    const response = await api.get('/torrent?id=' + torrentId, {
      responseType: 'blob',
    })

    const blob = response.data
    const url = window.URL.createObjectURL(blob)
    const a = document.createElement('a')
    a.href = url
    a.download = `${torrentId}.torrent`
    document.body.appendChild(a)
    a.click()
    window.URL.revokeObjectURL(url)
    document.body.removeChild(a)
  } catch (error) {
    console.error('API Error:', error)
    throw error
  }
}

export type TorrentSearch = components['schemas']['TorrentSearch']

export const searchTorrents = async (searchOptions: TorrentSearch) => {
  try {
    return (await api.post('/search/torrent', searchOptions)).data
  } catch (error) {
    console.error('API Error:', error)
    throw error
  }
}

export type UserCreatedTorrentReport = components['schemas']['UserCreatedTorrentReport']

export const reportTorrent = async (torrentReport: UserCreatedTorrentReport) => {
  try {
    return (await api.post('/report/torrent', torrentReport)).data
  } catch (error) {
    console.error('API Error:', error)
    throw error
  }
}
