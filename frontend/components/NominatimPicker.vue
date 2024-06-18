<template>
  <div>
    <form @submit.prevent="searchNominatim">
      <InputGroup class="mb-6 mt-2">
        <InputText
          v-model="locationInput"
          placeholder="Rechercher une adresse"
        />
        <Button
          v-tooltip.bottom="'Lancer la recherche'"
          severity="primary"
          type="submit"
        >
          <template #icon>
            <AppIcon icon-name="search" />
          </template>
        </Button>
      </InputGroup>
    </form>

    <div v-if="!results.length && resultsSearched">
      <Message severity="error">
        Aucun résultat trouvé
      </Message>
    </div>
    <div
      v-else-if="locationSelected && resultsSearched"
      class="h-80"
    >
      <SingleEntityMap
        :coordinates="transformedCoordinates"
        :locked="false"
        fill-color="#9999FF"
        border-color="#222222"
        :icon-hash="null"
        :zoom="10"
      />
      <small class="text-secondary">Addresse : {{ results[0].display_name }}</small>
    </div>
  </div>
</template>

<script setup lang="ts">
import { transform } from 'ol/proj'
import type { UnprocessedLocation } from '~/lib'
import { freeFormSearch, type Result } from '~/lib/nominatim'

const props = defineProps<{ modelValue: UnprocessedLocation | null }>()
const emits = defineEmits(['select'])

const locationInput = ref(props.modelValue?.plain_text || '')
const locationSelected = ref(props.modelValue)
const results = ref<Result[]>([])
const resultsSearched = ref(false)

const transformedCoordinates = computed(() => {
  if (!props.modelValue) return [0, 0]

  return transform(
    [props.modelValue!.long, props.modelValue!.lat], 'EPSG:4326', 'EPSG:3857')
})

async function searchNominatim() {
  results.value = await freeFormSearch(locationInput.value)
  resultsSearched.value = true
  if (results.value.length !== 0) {
    locationSelected.value = {
      lat: results.value[0].lat,
      long: results.value[0].lon,
      plain_text: locationInput.value,
    }
    emits('select', locationSelected.value)
  }
}
</script>
