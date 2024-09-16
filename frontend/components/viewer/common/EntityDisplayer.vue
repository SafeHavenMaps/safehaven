<template>
  <div>
    <Tabs v-model:value="tabValue">
      <TabList>
        <Tab
          v-if="hasChildren"
          value="0"
        >
          {{ $t('cmp.viewer.common.entityDisplayer.attached') }}
        </Tab>
        <Tab value="1">
          {{ $t('cmp.viewer.common.entityDisplayer.information') }}
        </Tab>
        <Tab
          v-if="hasParent"
          value="2"
        >
          {{ $t('cmp.viewer.common.entityDisplayer.attachments') }}
        </Tab>
        <Tab
          v-if="hasComments"
          value="3"
        >
          {{ $t('cmp.viewer.common.entityDisplayer.comments') }}
        </Tab>
      </TabList>

      <TabPanels>
        <TabPanel
          v-if="hasChildren"
          value="0"
        >
          <Card
            v-for="child in props.entity?.children"
            :key="child.id"
            class="mb-1"
          >
            <template #title>
              {{ child.display_name }}
            </template>

            <template #content>
              <p class="m-0">
                <CategoryTag :category="getCategory(child.category_id)" />
              </p>
            </template>

            <template #footer>
              <div
                class="flex justify-end items-end"
              >
                <Button
                  severity="secondary"
                  @click="newEntitySelected(child.id)"
                >
                  {{ $t('cmp.viewer.common.entityDisplayer.view') }}
                </Button>
              </div>
            </template>
          </Card>
        </TabPanel>

        <TabPanel
          value="1"
        >
          <div
            v-if="(props.entity?.tags.length ?? 0) > 0"
            class="flex flex-wrap gap-1"
          >
            <DisplayedTag
              v-for="tag in props.entity?.tags"
              :key="tag.id"
              :tag="tag"
            />
          </div>

          <Fieldset
            v-if="props.entity?.entity.locations.length > 0"
            :legend="props.entity?.entity.locations.length > 1 ? $t('cmp.viewer.common.entityDisplayer.addresses') : $t('cmp.viewer.common.entityDisplayer.address')"
          >
            <ul class="list-disc list-inside">
              <li
                v-for="location in props.entity?.entity.locations"
                :key="location.plain_text"
              >
                {{ location.plain_text }}
              </li>
            </ul>
          </Fieldset>

          <!--
            We display the DiscreteScore fields first, as they give context on the entity.
          -->
          <Fieldset
            v-if="hasScores"
            :legend="$t('cmp.viewer.common.entityDisplayer.notes')"
          >
            <div
              v-for="score in discreteScoreAveragesOnComments"
              :key="`${score.key}-${props.entity!.entity.id}`"
              class="m-0"
            >
              <h4 class="m-0 font-medium">
                {{ score.display_name }}
              </h4>
              <ViewerCommonScoreJauge :score="score.average" />
            </div>
          </Fieldset>

          <ViewerCommonFormFields
            :fields="props.entity.family.entity_form.fields
              .filter(field => field.categories == null || field.categories.includes(props.entity.category.id))"
            :data="props.entity?.entity.data"
          />
        </TabPanel>

        <TabPanel
          v-if="hasParent"
          value="2"
        >
          <Card
            v-for="parent in props.entity?.parents"
            :key="parent.id"
            class="mb-1"
          >
            <template #title>
              {{ parent.display_name }}
            </template>

            <template #content>
              <p class="m-0">
                <CategoryTag :category="getCategory(parent.category_id)" />
              </p>
            </template>

            <template #footer>
              <div
                class="flex justify-end items-end"
              >
                <Button
                  severity="secondary"
                  @click="newEntitySelected(parent.id)"
                >
                  {{ $t('cmp.viewer.common.entityDisplayer.view') }}
                </Button>
              </div>
            </template>
          </Card>
        </TabPanel>

        <TabPanel
          v-if="hasComments"
          value="3"
        >
          <CommentsDisplayer
            public
            :comments="props.entity.comments"
            :comment-form-fields="props.entity!.family.comment_form.fields"
            :entity-category-id="props.entity.category.id"
          />
        </TabPanel>
      </TabPanels>
    </Tabs>
  </div>
</template>

<script setup lang="ts">
import Tabs from 'primevue/tabs'
import type { Category, ResolvedFetchedEntity } from '~/lib'

const props = defineProps<{
  entity: ResolvedFetchedEntity
  categories: Category[]
}>()

const emits = defineEmits<{
  entitySelected: [string]
}>()

function getCategory(id: string) {
  return props.categories.find(c => c.id === id)!
}

const discreteScoreAveragesOnComments = ref<{
  key: string
  display_name: string
  average: number
}[]>([])

function refreshDiscreteScoreAverages() {
  discreteScoreAveragesOnComments.value = props.entity.family.comment_form.fields
    .filter(f => f.field_type === 'DiscreteScore')
    .sort((a, b) => a.display_weight - b.display_weight)
    .map(field => ({
      key: field.key,
      display_name: field.display_name,
      average: props.entity.comments
        .map(c => (c.data as Record<string, number>)[field.key] ?? 0)
        .reduce((acc, score) => acc + score, 0) / props.entity.comments.filter(c => (c.data as Record<string, number>)[field.key] != null).length,
    }))
}

refreshDiscreteScoreAverages()

const hasScores = computed(() => discreteScoreAveragesOnComments.value.length > 0
  && discreteScoreAveragesOnComments.value.some(score => !isNaN(score.average)))

const hasComments = computed(() => props.entity.comments.length > 0)

const hasChildren = computed(() => props.entity.children.length > 0)

const hasParent = computed(() => props.entity.parents.length > 0)

function newEntitySelected(id: string) {
  emits('entitySelected', id)
}

const tabValue = ref(hasChildren.value ? '0' : '1')

watch(
  () => props.entity,
  (newEntity, oldValue) => {
    refreshDiscreteScoreAverages()
    if (newEntity.entity.id !== oldValue.entity.id) {
      tabValue.value = hasChildren.value ? '0' : '1'
    }
  },
)
</script>
