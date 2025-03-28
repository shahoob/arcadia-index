import api from './api.ts'

export const getExternalDatabaseData = async (item_id: string | Number, database: string) => {
  try {
    switch (database) {
      case 'openlibrary': {
        return (await api.get('external_db/open_library?id=' + item_id)).data
      }
      default:
        return {}
    }
  } catch (error) {
    console.error('API Error:', error)
    throw error
  }
}
