<template>
  <div>
    <div class="col-12">
      <div
        class="flex flex-column sm:flex-row sm:align-items-stretch p-4 gap-3"
      >
        <div
          class="md:w-10rem flex flex-column"
        >
          <SingleEntityMap
            v-if="props.entity.web_mercator_x && props.entity.web_mercator_y"
            :entity="{
              id: props.entity.id,
              coordinates: [props.entity.web_mercator_x, props.entity.web_mercator_y],
              category: {
                fill_color: state.getCategory(props.entity.category_id).fill_color,
                border_color: state.getCategory(props.entity.category_id).border_color,
                icon_hash: state.getCategory(props.entity.category_id).icon_hash,
              },
            }"
          />
          <div
            v-else
            class="no-geo"
          >
            Aucune position géographique
          </div>
        </div>

        <div class="flex flex-column md:flex-row justify-content-between md:align-items-center flex-1 gap-4">
          <div class="flex flex-row md:flex-column justify-content-between align-items-start gap-2">
            <div>
              <CategoryTag
                :category="state.getCategory(props.entity.category_id)"
                :size="2"
              />
              <div class="text-lg font-medium text-900 mt-2">
                {{ props.entity.display_name }}
              </div>
            </div>
            <div
              class="surface-100 p-1"
              style="border-radius: 30px"
            >
              {{ props.entity.plain_text_location }}
            </div>
          </div>

          <div class="flex flex-column md:align-items-end gap-5">
            <div class="flex flex-row-reverse md:flex-row gap-2">
              <Button
                label="Voir en détail"
                class="flex-auto md:flex-initial white-space-nowrap"
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
import type { PaginatedCachedEntities } from '~/lib'
import state from '~/lib/viewer-state'

type ReceivedEntity = PaginatedCachedEntities['entities'][0]

const props = defineProps<{
  entity: ReceivedEntity
}>()

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
