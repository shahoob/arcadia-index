import api from './api.ts'

export const getTitleGroup = async (id: string | Number) => {
  try {
    const response = await api.get('/title-group?id=' + id)
    return response.data
  } catch (error) {
    console.error('API Error:', error)
    throw error
  }
}
