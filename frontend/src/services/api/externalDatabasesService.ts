import type { components } from '@/api-schema/schema.js'
import api from './api.ts'

export type ExternalDBData = components['schemas']['ExternalDBData']

export const getExternalDatabaseData = async (item_id: string | number, database: string): Promise<ExternalDBData> => {
  switch (database) {
    case 'isbn': {
      return (await api.get('external_db/isbn?isbn=' + item_id)).data
    }
    case 'musicbrainz': {
      return (await api.get('external_db/musicbrainz?url=' + item_id)).data
    }
    case 'tmdb': {
      return (await api.get('external_db/tmdb?url=' + item_id)).data
    }
    // case 'tmdb/tv': {
    //   return (await api.get('external_db/tmdb/tv?id=' + item_id)).data
    // }
    default:
      return {}
  }
}
