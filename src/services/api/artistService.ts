import api from './api.ts'

export const getArtist = async (id: string | Number) => {
  try {
    return (await api.get('/artist?id=' + id)).data
  } catch (error) {
    console.error('API Error:', error)
    throw error
  }
}
