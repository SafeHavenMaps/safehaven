// This file re-exports types from the API schema, and other convenient types
// so that they can be easily imported in other files.

import type { Coordinate } from 'ol/coordinate'
import type * as api from '~/lib/api'

export type AppError = {
  error_code: string
  details?: string
}

export type InitConfig = api.components['schemas']['StatusResponse']
export type SafeHavenVersion = api.components['schemas']['SafeHavenVersionResponse']

export type SafeHavenOptions = api.components['schemas']['SafeHavenOptions']
export type ConfigurationOption = api.components['schemas']['ConfigurationOption']

export type Category = api.components['schemas']['Category']
export type Tag = api.components['schemas']['Tag']

export type PublicEntity = api.components['schemas']['PublicEntity']
export type PublicListedEntity = api.components['schemas']['PublicListedEntity']
export type PublicNewEntity = api.components['schemas']['PublicNewEntity']
export type PublicNewEntityRequest = api.components['schemas']['PublicNewEntityRequest']
export type PublicNewEntityResponse = api.components['schemas']['PublicNewEntityResponse']
export type Cluster = api.components['schemas']['Cluster']

export type ViewerCachedEntity = api.components['schemas']['ViewerCachedEntity']
export type ViewerSearchedCachedEntity = api.components['schemas']['ViewerSearchedCachedEntity']
export type ViewerPaginatedCachedEntities = PaginatedVec<ViewerSearchedCachedEntity>

export type FetchedEntity = api.components['schemas']['FetchedEntity']
export type ResolvedFetchedEntity = FetchedEntity & {
  family: Family
  category: Category
  tags: Tag[]
}

export type AdminEntity = api.components['schemas']['AdminEntity']
export type AdminListedEntity = api.components['schemas']['AdminListedEntity']
export type AdminEntityWithRelations = api.components['schemas']['AdminEntityWithRelations']
export type AdminNewOrUpdateEntity = api.components['schemas']['AdminNewOrUpdateEntity']
export type AdminSearchRequestBody = api.components['schemas']['AdminSearchRequest']
export type AdminCachedEntity = api.components['schemas']['AdminCachedEntity']
export type AdminPaginatedCachedEntities = PaginatedVec<AdminCachedEntity>

export type PublicComment = api.components['schemas']['PublicComment']
export type PublicNewComment = api.components['schemas']['PublicNewComment']
export type PublicNewCommentRequest = api.components['schemas']['NewCommentRequest']
export type AdminComment = api.components['schemas']['AdminComment']
export type AdminListedComment = api.components['schemas']['AdminListedComment']
export type AdminNewOrUpdateComment = api.components['schemas']['AdminNewOrUpdateComment']

export type ErrorResponse = api.components['schemas']['ErrorResponse']

export type NewOrUpdateCategory = api.components['schemas']['NewOrUpdateCategory']
export type NewOrUpdateTag = api.components['schemas']['NewOrUpdateTag']
export type PermissionPolicy = api.components['schemas']['PermissionPolicy']
export type Permissions = api.components['schemas']['Permissions']
export type NewOrUpdateAccessToken = api.components['schemas']['NewOrUpdateAccessToken']
export type AccessToken = api.components['schemas']['AccessToken']
export type AccessTokenStats = api.components['schemas']['AccessTokenStats']
export type BootstrapPermissions = api.components['schemas']['BootstrapPermissions']

export type User = api.components['schemas']['User']
export type NewOrUpdatedUser = api.components['schemas']['NewOrUpdatedUser']
export type AdminUserTokenClaims = api.components['schemas']['AdminUserTokenClaims']

export type UnprocessedLocation = api.components['schemas']['UnprocessedLocation']

export interface CanBeHighlighted {
  highlighted: boolean
}

export interface HasCoordinates {
  coordinates: Coordinate
}

export interface HasFamily {
  family: Family
}

export interface HasCategory {
  category: Category
}

export type DisplayableCachedEntity = ViewerCachedEntity &
  HasCoordinates &
  HasFamily &
  HasCategory &
  CanBeHighlighted

export type DisplayableCluster = Cluster & HasCoordinates

export type StringFieldTypeMetadata = {
  format: string
}

export type OptionsFieldTypeMetadata = {
  options: {
    value: string
    label: string
    hidden?: boolean
  }[]
}

export type EventsFieldTypeMetadata = {
  event_types: {
    value: string
    label: string
    color: string
  }[]
}

export type FieldTypeMetadataEnum = {
  field_type: 'SingleLineText'
  field_type_metadata: StringFieldTypeMetadata
} | {
  field_type: 'EnumSingleOption'
  field_type_metadata: OptionsFieldTypeMetadata
} | {
  field_type: 'EnumMultiOption'
  field_type_metadata: OptionsFieldTypeMetadata
} | {
  field_type: 'EventList'
  field_type_metadata: EventsFieldTypeMetadata
} | {
  field_type_metadata?: null
} | {
  field_type: 'MultiLineText' | 'RichText'
} | {
  field_type: 'Number'
} | {
  field_type: 'Boolean'
} | {
  field_type: 'DiscreteScore'
} | {
  field_type: 'Date'
}

export type NewOrUpdateFamily = api.components['schemas']['NewOrUpdateFamily']

export type FormField = NewOrUpdateFamily['entity_form']['fields'][number]

export type Family = api.components['schemas']['Family']

export type EntityOrCommentEvent = {
  date: Date | undefined
  type: string | undefined
  details: string | undefined
}

export type FieldContentMap = {
  SingleLineText: string
  MultiLineText: string
  RichText: string
  Number: number
  Boolean: boolean
  DiscreteScore: number
  Date: Date
  EnumSingleOption: string
  EnumMultiOption: string[]
  EventList: EntityOrCommentEvent[]
}

export type EntityOrCommentData = Record<string, FieldContentMap[FormField['field_type']]>

export type FamilyRecord = Record<string, Family>
export type CategoryRecord = Record<string, Category>
export type TagRecord = Record<string, Tag>

export type HomePageStats = api.components['schemas']['HomePageStats']

export type EnumFilter = {
  key: string
  title: string
  values: {
    label: string
    value: string
  }[]
  active: string[]
}
