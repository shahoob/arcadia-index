import type { components } from '@/api-schema/schema.js'
import api from './api.ts'

export type ExternalDBData = components['schemas']['ExternalDBData']

export const getExternalDatabaseData = async (
  item_id: string | number,
  database: string,
): Promise<ExternalDBData> => {
  switch (database) {
    case 'openlibrary': {
      return (await api.get('external_db/open_library?id=' + item_id)).data
    }
    case 'tmdb/movie': {
      return (await api.get('external_db/tmdb/movie?id=' + item_id)).data
    }
    case 'tmdb/tv': {
      return (await api.get('external_db/tmdb/tv?id=' + item_id)).data
    }
    default:
      return {}
  }
}
