import api from './api.ts'

export const getExternalDatabaseData = async (item_id: string | number, database: string) => {
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
