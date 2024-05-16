// This file re-exports types from the API schema, and other convienient types
// so that they can be easily imported in other files.

import type { Coordinate } from 'ol/coordinate'
import type * as api from '~/lib/api'

export type Status = api.components['schemas']['StatusResponse']

export type Entity = api.components['schemas']['Entity']
export type Cluster = api.components['schemas']['Cluster']
export type CachedEntity = api.components['schemas']['CachedEntity']
export type Family = api.components['schemas']['Family']
export type Category = api.components['schemas']['Category']
export type Tag = api.components['schemas']['Tag']
export type CartographyInitConfig =
  api.components['schemas']['CartographyInitConfig']
export type FetchedEntity = api.components['schemas']['FetchedEntity']

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

export type DisplayableCachedEntity = CachedEntity &
  HasCoordinates &
  HasFamily &
  HasCategory &
  CanBeHighlighted

export type DisplayableCluster = Cluster & HasCoordinates
