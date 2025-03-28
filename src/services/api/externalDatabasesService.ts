import api from './api.ts'

export const getExternalDatabaseData = async (item_id: string | Number, database: string) => {
  try {
    switch (database) {
      case 'openlibrary': {
        const response = await api.get('external_db/open_library?id=' + item_id)
        return response.data
      }
      default:
        return {}
    }
  } catch (error) {
    console.error('API Error:', error)
    throw error
  }
}
