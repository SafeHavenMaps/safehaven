<template>
  <div>
    <div class="col-span-12">
      <div
        class="flex flex-col sm:flex-row sm:items-stretch p-6 gap-4"
      >
        <div
          class="md:w-40 flex flex-col"
        >
          <SingleEntityMap
            v-if="props.entity.locations.length > 0"
            :coordinates="locations"
            :fill-color="state.getCategory(props.entity.category_id).fill_color"
            :border-color="state.getCategory(props.entity.category_id).fill_color"
            :icon-hash="state.getCategory(props.entity.category_id).icon_hash"
            :zoom="13"
            :locked="true"
          />
          <div
            v-else
            class="no-geo"
          >
            Aucune position géographique
          </div>
        </div>

        <div class="flex flex-col md:flex-row justify-between md:items-center flex-1 gap-6">
          <div class="flex flex-row md:flex-col justify-between items-start gap-2">
            <div>
              <CategoryTag
                :category="state.getCategory(props.entity.category_id)"
                :size="2"
              />
              <div class="text-lg font-medium mt-2">
                {{ props.entity.display_name }}
              </div>
            </div>

            <div
              class="p-1"
              style="border-radius: 30px"
            >
              <ul>
                <li
                  v-for="(loc, idx) in props.entity.locations"
                  :key="idx"
                >
                  {{ loc.plain_text }}
                </li>
              </ul>
            </div>
          </div>

          <div class="flex flex-col md:items-end gap-8">
            <div class="flex flex-row-reverse md:flex-row gap-2">
              <Button
                label="Voir en détail"
                class="flex-auto md:flex-initial whitespace-nowrap"
                @click="changeActiveEntity(props.entity)"
              >
                <template #icon>
                  <AppIcon icon-name="eye" />
                </template>
              </Button>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { ViewerPaginatedCachedEntities } from '~/lib'
import state from '~/lib/viewer-state'

type ReceivedEntity = ViewerPaginatedCachedEntities['entities'][0]

const props = defineProps<{
  entity: ReceivedEntity
}>()

const locations = computed(() => props.entity.locations.map(loc => [loc.x, loc.y]))

function changeActiveEntity(entity: ReceivedEntity) {
  state.selectEntity(entity.entity_id)
}
</script>

<style>
.no-geo {
  text-align: center;
  background-color: #f0f0f0;
  height: 100%;
  width: 100%;
  display: flex;
  justify-content: center;
  align-content: center;
  flex-direction: column;
}
</style>
