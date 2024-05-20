<template>
  <!-- eslint-disable vue/no-v-html -->
  <!--
    In this file we use v-html to render the content of the fields.
    This is safe because we use DOMPurify to sanitize the content when required.
    All "v-html" usages are unsafe UNLESS they are sanitized.

    Developers and reviewers should be aware of the risks associated with using v-html when reading this file.
  -->

  <Sidebar
    v-model:visible="state.hasActiveEntity"
    :modal="false"
    position="left"
    class="w-full md:w-20rem lg:w-30rem"
  >
    <template #header>
      <div class="flex align-items-center gap-2">
        <Tag
          :style="{
            backgroundColor: state.activeEntity?.category.fill_color,
            borderColor: state.activeEntity?.category.border_color,
            color: 'white',
          }"
        >
          {{ state.activeEntity?.category.title }}
        </Tag>
        <h3 class="m-0">
          {{ state.activeEntity?.entity.display_name }}
        </h3>
      </div>
    </template>

    <div v-if="state.hasActiveEntity">
      <TabView>
        <TabPanel
          v-if="hasChildren()"
          header="Rattachés"
        >
          <Card
            v-for="child in state.activeEntity?.children"
            :key="child.id"
            class="mb-1"
          >
            <template #title>
              {{ child.display_name }}
            </template>

            <template #content>
              <p class="m-0">
                <Tag
                  :style="{
                    backgroundColor: getCategory(child.category_id)?.fill_color,
                    borderColor: getCategory(child.category_id)?.border_color,
                    color: 'white',
                  }"
                >
                  {{ getCategory(child.category_id)?.title }}
                </Tag>
              </p>
            </template>

            <template #footer>
              <div
                class="flex justify-content-end align-items-end"
              >
                <Button
                  severity="secondary"
                  @click="state.selectEntity(child.id)"
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
            v-if="(state.activeEntity?.tags.length ?? 0) > 0"
            class="mt-1"
          >
            <Divider type="dotted" />
            <Tag
              v-for="tag in state.activeEntity?.tags"
              :key="tag.id"
              class="mr-1 mb-1"
            >
              {{ tag.title }}
            </Tag>
            <Divider type="dotted" />
          </div>

          <div>
            <h3>
              {{ state.activeEntity?.entity.locations.length > 1 ? 'Adresses' : 'Adresse' }}
            </h3>
            <ul>
              <li
                v-for="location in state.activeEntity?.entity.locations"
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
              <ViewerSidebarScoreJauge :score="score.average" />
            </div>
          </Fieldset>

          <ViewerSidebarFormFields
            :fields="state.activeEntity?.family.entity_form.fields"
            :data="state.activeEntity?.entity.data"
          />
        </TabPanel>

        <TabPanel
          v-if="hasParent()"
          header="Rattachements"
        >
          <Card
            v-for="parent in state.activeEntity?.parents"
            :key="parent.id"
            class="mb-1"
          >
            <template #title>
              {{ parent.display_name }}
            </template>

            <template #content>
              <p class="m-0">
                <Tag
                  :style="{
                    backgroundColor: getCategory(parent.category_id)?.fill_color,
                    borderColor: getCategory(parent.category_id)?.border_color,
                    color: 'white',
                  }"
                >
                  {{ getCategory(parent.category_id)?.title }}
                </Tag>
              </p>
            </template>

            <template #footer>
              <div
                class="flex justify-content-end align-items-end"
              >
                <Button
                  severity="secondary"
                  @click="state.selectEntity(parent.id)"
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
              <p
                style="white-space: pre-wrap; word-break: break-word; overflow-wrap: break-word;"
                v-html="commentWithBreaks(comment.text)"
              />

              <ViewerSidebarFormFields
                :fields="state.activeEntity?.family.comment_form.fields"
                :data="comment.data"
              />
            </AccordionTab>
          </Accordion>
        </TabPanel>
      </TabView>
    </div>
  </Sidebar>
</template>

<script setup>
import DOMPurify from 'dompurify'
import state from '~/lib/viewer-state'

function commentDisplayTitle(comment) {
  return `${comment.author} - ${new Date(comment.created_at).toLocaleDateString()}`
}

function commentWithBreaks(txt) {
  return DOMPurify.sanitize(txt).replaceAll('\n', '<br />')
}

function discreteScoreAveragesOnComments() {
  return state.activeEntity.family.comment_form.fields
    .filter(f => f.field_type === 'DiscreteScore')
    .sort((a, b) => a.display_weight - b.display_weight)
    .map(field => ({
      key: field.key,
      display_name: field.display_name,
      average: state.activeEntity.comments
        .map(c => c.data[field.key])
        .reduce((acc, score) => acc + score, 0) / state.activeEntity.comments.length,
    }))
}

function hasScores() {
  const scores = discreteScoreAveragesOnComments()

  return scores.length > 0 && scores.some(score => !isNaN(score.average))
}

function hasComments() {
  return state.activeEntity.comments.length > 0
}

function hasChildren() {
  return state.activeEntity.children.length > 0
}

function hasParent() {
  return state.activeEntity.parents.length > 0
}

function getCategory(category_id) {
  return state.categories.find(c => c.id === category_id)
}

function sortedComments() {
  return state.activeEntity.comments
    .sort((a, b) => new Date(b.created_at) - new Date(a.created_at))
}
</script>
