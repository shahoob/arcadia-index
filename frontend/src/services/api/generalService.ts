import api from './api.ts'

export const subscribeToItem = async (item_id: number, item: string) => {
  return (await api.post('/subscription?item=' + item + '&item_id=' + item_id)).data
}
export const unsubscribeToItem = async (item_id: number, item: string) => {
  return (await api.delete('/subscription?item=' + item + '&item_id=' + item_id)).data
}
