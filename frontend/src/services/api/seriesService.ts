import api from './api.ts'

export const getSeries = async (id: string | number) => {
  try {
    return (await api.get('/series?id=' + id)).data
  } catch (error) {
    console.error('API Error:', error)
    throw error
  }
}
