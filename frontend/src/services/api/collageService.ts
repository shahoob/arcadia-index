import type { components } from '@/api-schema/schema'
import api from './api.ts'

export type CollageAndAssociatedData = components['schemas']['CollageAndAssociatedData']

export type Collage = components['schemas']['Collage']

export type CollageSearchResponse = components['schemas']['CollageSearchResponse']

export type SearchCollagesQuery = components['schemas']['SearchCollagesQuery']

export type CollageSearchResult = components['schemas']['CollageSearchResult']

export const getCollage = async (id: number): Promise<CollageAndAssociatedData> => {
  return (await api.get<CollageAndAssociatedData>('/collages?id=' + id)).data
}

export const searchCollages = async (form: SearchCollagesQuery): Promise<CollageSearchResponse> => {
  return (await api.get<CollageSearchResponse>('/search/collages', { params: form })).data
}
