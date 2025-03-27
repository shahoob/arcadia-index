import api from './api.ts'

export const subscribeToItem = async (item_id: string | Number, item: string) => {
  try {
    const response = await api.post('/subscription?item=' + item + '&item_id=' + item_id)
    return response.data
  } catch (error) {
    console.error('API Error:', error)
    throw error
  }
}
export const unsubscribeToItem = async (item_id: string | Number, item: string) => {
  try {
    const response = await api.delete('/subscription?item=' + item + '&item_id=' + item_id)
    return response.data
  } catch (error) {
    console.error('API Error:', error)
    throw error
  }
}
