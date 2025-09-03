import type { components } from '@/api-schema/schema'
import api from './api.ts'

export type StaffPm = components['schemas']['StaffPm']

export type StaffPmOverview = components['schemas']['StaffPmOverview']

export type StaffPmMessage = components['schemas']['StaffPmMessage']

export type UserCreatedStaffPm = components['schemas']['UserCreatedStaffPm']

export type UserCreatedStaffPmMessage = components['schemas']['UserCreatedStaffPmMessage']

export const listStaffPms = async (): Promise<StaffPmOverview[]> => {
  return (await api.get('/staff-pms')).data
}

export const postStaffPm = async (conversation: UserCreatedStaffPm): Promise<StaffPm> => {
  return (await api.post<StaffPm>('/staff-pms', conversation)).data
}

export const postStaffPmMessage = async (message: UserCreatedStaffPmMessage): Promise<StaffPmMessage> => {
  return (await api.post<StaffPmMessage>('/staff-pms/messages', message)).data
}

export const getStaffPm = async (id: number) => {
  return (await api.get(`/staff-pms/${id}`)).data
}

export const resolveStaffPm = async (id: number): Promise<StaffPm> => {
  return (await api.put<StaffPm>(`/staff-pms/${id}/resolve`)).data
}
