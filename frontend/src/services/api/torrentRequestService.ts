import type { components } from '@/api-schema/schema'
import api from './api'

export type TorrentRequest = components['schemas']['TorrentRequest']

export type TorrentRequestHierarchyLite = components['schemas']['TorrentRequestHierarchyLite']

export type TorrentRequestWithTitleGroupLite = components['schemas']['TorrentRequestWithTitleGroupLite']

export type SearchTorrentRequestsQuery = components['schemas']['SearchTorrentRequestsQuery']

export const searchTorrentRequests = async (form: SearchTorrentRequestsQuery): Promise<TorrentRequestWithTitleGroupLite[]> => {
  return (await api.get<TorrentRequestWithTitleGroupLite[]>('/search/torrent-request', { params: form })).data
}
