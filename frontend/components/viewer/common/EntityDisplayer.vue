<template>
  <div>
    <TabView>
      <TabPanel
        v-if="hasChildren()"
        header="Rattachés"
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
              class="flex justify-content-end align-items-end"
            >
              <Button
                severity="secondary"
                @click="newEntitySelected(child.id)"
              >
                Voir
              </Button>
            </div>
          </template>
        </Card>
      </TabPanel>

      <TabPanel
        header="Informations"
      >
        <div
          v-if="(props.entity?.tags.length ?? 0) > 0"
          class="mt-1"
        >
          <Divider type="dotted" />
          <Tag
            v-for="tag in props.entity?.tags"
            :key="tag.id"
            class="mr-1 mb-1"
          >
            {{ tag.title }}
          </Tag>
          <Divider type="dotted" />
        </div>

        <div>
          <h3>
            {{ props.entity?.entity.locations.length > 1 ? 'Adresses' : 'Adresse' }}
          </h3>
          <ul>
            <li
              v-for="location in props.entity?.entity.locations"
              :key="location.plain_text"
            >
              {{ location.plain_text }}
            </li>
          </ul>
        </div>

        <!--
          We display the DiscreteScore fields first, as they give context on the entity.
        -->
        <Fieldset
          v-if="hasScores()"
          legend="Notes"
        >
          <div
            v-for="score in discreteScoreAveragesOnComments().filter(score => !isNaN(score.average))"
            :key="score.key"
            class="m-0"
          >
            <h4 class="m-0">
              {{ score.display_name }}
            </h4>
            <ViewerCommonScoreJauge :score="score.average" />
          </div>
        </Fieldset>

        <ViewerCommonFormFields
          :fields="props.entity?.family.entity_form.fields"
          :data="props.entity?.entity.data"
        />
      </TabPanel>

      <TabPanel
        v-if="hasParent()"
        header="Rattachements"
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
              class="flex justify-content-end align-items-end"
            >
              <Button
                severity="secondary"
                @click="newEntitySelected(parent.id)"
              >
                Voir
              </Button>
            </div>
          </template>
        </Card>
      </TabPanel>

      <TabPanel
        v-if="hasComments()"
        header="Commentaires"
      >
        <Accordion :active-index="0">
          <AccordionTab
            v-for="comment in sortedComments()"
            :key="comment.id"
            :header="commentDisplayTitle(comment)"
          >
            <!-- eslint-disable vue/no-v-html -->
            <p
              style="white-space: pre-wrap; word-break: break-word; overflow-wrap: break-word;"
              v-html="commentWithBreaks(comment.text)"
            />

            <ViewerCommonFormFields
              :fields="props.entity?.family.comment_form.fields"
              :data="comment.data"
            />
          </AccordionTab>
        </Accordion>
      </TabPanel>
    </TabView>
  </div>
</template>

<script setup lang="ts">
import DOMPurify from 'dompurify'
import type { Category, PublicComment, ResolvedFetchedEntity } from '~/lib'

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

function commentDisplayTitle(comment: PublicComment) {
  return `${comment.author} - ${new Date(comment.created_at).toLocaleDateString()}`
}

function commentWithBreaks(txt: string) {
  return DOMPurify.sanitize(txt).replaceAll('\n', '<br />')
}

function discreteScoreAveragesOnComments() {
  return props.entity.family.comment_form.fields
    .filter(f => f.field_type === 'DiscreteScore')
    .sort((a, b) => a.display_weight - b.display_weight)
    .map(field => ({
      key: field.key,
      display_name: field.display_name,
      average: props.entity.comments
        .map(c => (c.data as Record<string, number>)[field.key])
        .reduce((acc, score) => acc + score, 0) / props.entity.comments.length,
    }))
}

function hasScores() {
  const scores = discreteScoreAveragesOnComments()

  return scores.length > 0 && scores.some(score => !isNaN(score.average))
}

function hasComments() {
  return props.entity.comments.length > 0
}

function hasChildren() {
  return props.entity.children.length > 0
}

function hasParent() {
  return props.entity.parents.length > 0
}

function sortedComments() {
  return props.entity.comments.slice().sort((a, b) => new Date(b.created_at).getTime() - new Date(a.created_at).getTime())
}

function newEntitySelected(id: string) {
  emits('entitySelected', id)
}
</script>
