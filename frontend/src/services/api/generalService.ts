import api from './api.ts'

export const subscribeToItem = async (item_id: string | Number, item: string) => {
  try {
    return (await api.post('/subscription?item=' + item + '&item_id=' + item_id)).data
  } catch (error) {
    console.error('API Error:', error)
    throw error
  }
}
export const unsubscribeToItem = async (item_id: string | Number, item: string) => {
  try {
    return (await api.delete('/subscription?item=' + item + '&item_id=' + item_id)).data
  } catch (error) {
    console.error('API Error:', error)
    throw error
  }
}
