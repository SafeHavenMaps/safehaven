<template>
  <Sidebar
    v-model:visible="state.hasActiveEntity"
    :modal="false"
    position="left"
    class="w-full md:w-20rem lg:w-30rem"
  >
    <div v-if="state.hasActiveEntity">
      <h2 class="m-0">
        {{ state.activeEntity?.entity.display_name }}
      </h2>
      <Tag
        :style="{
          backgroundColor: state.activeEntity?.category.fill_color,
          borderColor: state.activeEntity?.category.border_color,
          color: 'white',
        }"
      >
        {{ state.activeEntity?.category.title }}
      </Tag>
      <div v-if="state.activeEntity?.entity.locations.length > 1">
        <h3>Lieux de consultations</h3>
        <ul>
          <li
            v-for="location in state.activeEntity?.entity.locations"
            :key="location.plain_text"
          >
            {{ location.plain_text }}
          </li>
        </ul>
      </div>
      <div v-if="(state.activeEntity?.tags.length ?? 0) > 0">
        <h3>Tags</h3>
        <Tag
          v-for="tag in state.activeEntity?.tags"
          :key="tag.id"
          class="m-1"
        >
          {{ tag.title }}
        </Tag>
      </div>
      <div
        v-for="field in state.activeEntity?.family.entity_form.fields.sort((a, b) => a.display_weight - b.display_weight)"
        :key="field.key"
      >
        <div v-if="(field.field_type == 'SingleLineText' || field.field_type == 'MultiLineText') && hasKey(field.key)">
          <h3>{{ field.display_name }}</h3>

          <div v-if="isUrlField(field.key)">
            <a :href="getDataAsEscapedString(field.key)" target="_blank" g >
              {{ getDataAsEscapedString(field.key) }}
            </a>
          </div>

          <p
            v-else
            v-html="getDataAsEscapedString(field.key)"
          />
        </div>

        <div v-else-if="field.field_type == 'Date' && hasKey(field.key)" />

        <div v-else-if="field.field_type == 'Number' && hasKey(field.key)" />

        <div v-else-if="field.field_type == 'Boolean' && hasKey(field.key)" />

        <div v-else-if="field.field_type == 'DiscreteScore' && hasKey(field.key)" />

        <div v-else-if="field.field_type == 'EnumSingleOption' && hasKey(field.key)">
          <h3>{{ field.display_name }}</h3>
          <p>
            {{ getValueForEnum(field.key, getKeyValue(field.key)) }}
          </p>
        </div>

        <div v-else-if="field.field_type == 'EnumMultiOption' && hasKey(field.key)">
          <h3>{{ field.display_name }}</h3>
          <Tag v-for="value in getKeyValue(field.key)" :key="value">
            {{ getValueForEnum(field.key, value) }}
          </Tag>
        </div>

        <div v-else-if="field.field_type == 'EventList' && hasKey(field.key)" />
      </div>
    </div>
  </Sidebar>
</template>

<script setup>
import DOMPurify from 'dompurify'
import state from '~/lib/viewer-state'

function isUrlField(key) {
  const field = state.activeEntity.family.entity_form.fields.find(f => f.key === key)
  return field.field_type_metadata?.format === 'url'
}

function getDataAsEscapedString(key) {
  const txt = state.activeEntity.entity.data[key] ?? ''
  return DOMPurify.sanitize(txt).replaceAll('\n', '<br />')
}

function hasKey(key) {
  return state.activeEntity.entity.data[key] !== undefined && state.activeEntity.entity.data[key] !== null
}

function getKeyValue(key) {
  return state.activeEntity.entity.data[key]
}

function getValueForEnum(key, value) {
  const field = state.activeEntity.family.entity_form.fields.find(f => f.key === key)
  return field.field_type_metadata.options.find(o => o.value === value)?.label
}
</script>
