import type { ComponentCustomProperties } from 'vue'

declare module '@vue/runtime-core' {
  export interface ComponentCustomProperties {
    $timeAgo: (date: string) => string
    $bytesToReadable: (bytes: number) => string
    $getEditionGroupSlug: (editionGroup: unknown) => string
    $getFeatures: (contentType: string) => string[]
    $getLanguages: () => string[]
    $getPlatforms: () => string[]
    $getSources: (contentType: string) => string[]
    $isValidUrl: (url: string) => boolean
  }
}
