import type { components } from '@/api-schema/schema'
import api from './api.ts'

export type UserCreatedUserApplication = components['schemas']['UserCreatedUserApplication']

export const postUserApplication = async (application: UserCreatedUserApplication) => {
  return (await api.post('/user-application', application)).data
}
