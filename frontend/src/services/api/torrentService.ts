import type { components } from '@/api-schema/schema'
import api from './api.ts'
import type { TitleGroupHierarchyLite } from './artistService.ts'

export type TitleGroup = components['schemas']['TitleGroup']

export type EditedTitleGroup = components['schemas']['EditedTitleGroup']

export type TitleGroupHierarchy = components['schemas']['TitleGroupHierarchy']

export type TitleGroupCategory = components['schemas']['TitleGroupCategory']

export type TitleGroupLite = components['schemas']['TitleGroupLite']

export type TitleGroupAndAssociatedData = components['schemas']['TitleGroupAndAssociatedData']

export type ContentType = components['schemas']['ContentType']

export type PublicRating = components['schemas']['PublicRating']

export type EmbeddedLinks = {
  [key: string]: {
    [key: string]: string
  }
}

export const getTitleGroup = async (id: number): Promise<TitleGroupAndAssociatedData> => {
  return (await api.get<TitleGroupAndAssociatedData>('/title-group?id=' + id)).data
}

export const getTitleGroupLite = async (id: number): Promise<TitleGroupLite> => {
  return (await api.get<TitleGroupLite>('/title-group/lite?id=' + id)).data
}

export const searchTitleGroupLite = async (name: string, contentType: ContentType | null): Promise<TitleGroupHierarchyLite[]> => {
  return (await api.get<TitleGroupHierarchyLite[]>('/search/title-group/lite?name=' + name + (contentType ? `&content_type=${contentType}` : ''))).data
}

export type UserCreatedTitleGroup = components['schemas']['UserCreatedTitleGroup']

export const createTitleGroup = async (titleGroup: UserCreatedTitleGroup) => {
  return (await api.post<TitleGroup>('/title-group', titleGroup)).data
}

export const editTitleGroup = async (title_group: EditedTitleGroup): Promise<TitleGroup> => {
  return (await api.put<TitleGroup>('/title-group', title_group)).data
}

export type UserCreatedEditionGroup = components['schemas']['UserCreatedEditionGroup']

export type EditionGroup = components['schemas']['EditionGroup']

export type EditionGroupInfoLite = components['schemas']['EditionGroupInfoLite']

export type EditionGroupHierarchyLite = components['schemas']['EditionGroupHierarchyLite']

export type EditionGroupHierarchy = components['schemas']['EditionGroupHierarchy']

export type Source = components['schemas']['Source']

export const createEditionGroup = async (editionGroup: UserCreatedEditionGroup) => {
  editionGroup.additional_information = editionGroup.additional_information
    ? Object.fromEntries(Object.entries(editionGroup.additional_information).filter(([, value]) => value !== null && value !== ''))
    : {}
  editionGroup.covers = editionGroup.covers.filter((cover) => cover.trim() !== '')
  editionGroup.external_links = editionGroup.external_links.filter((link) => link.trim() !== '')
  editionGroup.distributor = editionGroup.distributor == '' ? null : editionGroup.distributor
  return (await api.post<EditionGroup>('/edition-group', editionGroup)).data
}

export type UploadedTorrent = components['schemas']['UploadedTorrent']

export type Torrent = components['schemas']['Torrent']

export type EditedTorrent = components['schemas']['EditedTorrent']

export type TorrentHierarchyLite = components['schemas']['TorrentHierarchyLite']

export type TorrentHierarchy = components['schemas']['TorrentHierarchy']

export const uploadTorrent = async (torrentForm: object) => {
  const formData = new FormData()
  for (const [key, value] of Object.entries(torrentForm)) {
    if (value != null) {
      formData.append(key, value)
    }
  }
  return (
    await api.post<Torrent>('/torrent', formData, {
      headers: {
        'Content-Type': 'multipart/form-data',
      },
    })
  ).data
}

export const downloadTorrent = async (torrent: Torrent, titleGroupName: string) => {
  const response = await api.get('/torrent?id=' + torrent.id, {
    responseType: 'blob',
  })

  const blob = response.data
  const url = window.URL.createObjectURL(blob)
  const a = document.createElement('a')
  a.href = url
  a.download = `[${import.meta.env.VITE_SITE_NAME}] - ${titleGroupName} (${torrent.id}).torrent`
  document.body.appendChild(a)
  a.click()
  window.URL.revokeObjectURL(url)
  document.body.removeChild(a)
}

export type TorrentSearch = components['schemas']['TorrentSearch']

export type TorrentSearchResults = components['schemas']['TorrentSearchResults']

export const searchTorrentsLite = async (searchOptions: TorrentSearch) => {
  return (await api.post<TorrentSearchResults>('/search/torrent/lite', searchOptions)).data
}

export type UserCreatedTorrentReport = components['schemas']['UserCreatedTorrentReport']

export type TorrentReport = components['schemas']['TorrentReport']

export type Features = components['schemas']['Features']

export type Extras = components['schemas']['Extras']

export const reportTorrent = async (torrentReport: UserCreatedTorrentReport) => {
  return (await api.post<TorrentReport>('/report/torrent', torrentReport)).data
}

export const editTorrent = async (editedTorrent: EditedTorrent) => {
  return (await api.put<Torrent>('/torrent', editedTorrent)).data
}

export type UploadInformation = components['schemas']['UploadInformation']

export const getUploadInformation = async () => {
  return (await api.get<UploadInformation>('/upload')).data
}

export type TorrentToDelete = components['schemas']['TorrentToDelete']

export const deleteTorrent = async (form: TorrentToDelete) => {
  return (await api.delete('/torrent', { data: form })).data
}
