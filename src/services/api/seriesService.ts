import api from './api.ts'

export const getSeries = async (id: string | Number) => {
  try {
    const response = await api.get('/series?id=' + id)
    return response.data
  } catch (error) {
    console.error('API Error:', error)
    throw error
  }
}
