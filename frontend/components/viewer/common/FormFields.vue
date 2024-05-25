<template>
  <Fieldset
    v-for="field in fieldsToDisplay()"
    :key="field.key"
    :legend="field.display_name"
  >
    <div v-if="(field.field_type == 'SingleLineText' || field.field_type == 'MultiLineText')">
      <div v-if="isUrlField(field.key)">
        <a
          :href="getKeyValue(field.key)"
          target="_blank"
        >
          {{ extractHostnameFromUrl(getKeyValue(field.key)) }}
        </a>
      </div>

      <p
        v-else
        v-html="getDataAsEscapedString(field.key)"
      />
    </div>

    <div v-else-if="field.field_type == 'Date'">
      <p>
        {{ new Date(getKeyValue(field.key)).toLocaleDateString() }}
      </p>
    </div>

    <div v-else-if="field.field_type == 'Number'">
      <p>
        {{ getKeyValue(field.key) }}
      </p>
    </div>

    <div v-else-if="field.field_type == 'Boolean'">
      <p>
        {{ getKeyValue(field.key) ? '✅ Oui' : '❌ Non' }}
      </p>
    </div>

    <div v-else-if="field.field_type == 'EnumSingleOption'">
      <p>
        {{ getValueForEnum(field.key, getKeyValue(field.key)) }}
      </p>
    </div>

    <div v-else-if="field.field_type == 'EnumMultiOption'">
      <Tag
        v-for="value in getKeyValue(field.key)"
        :key="value"
        class="mr-1 mb-1"
      >
        {{ getValueForEnum(field.key, value) }}
      </Tag>
    </div>

    <div v-else-if="field.field_type == 'DiscreteScore'">
      <ViewerCommonScoreJauge
        :score="getKeyValue(field.key)"
      />
    </div>

    <div v-else-if="field.field_type == 'EventList'">
      <Accordion :active-index="0">
        <AccordionTab
          v-for="event in getSortedEventList(field.key)"
          :key="event"
        >
          <template #header>
            <Tag
              :severity="event.severity"
              :value="event.title"
            />
          </template>

          <p>
            <strong>Date :</strong> {{ event.date.toLocaleDateString() }}
          </p>

          <p v-if="event.comment && event.comment.length > 0">
            <strong>Commentaire :</strong>
            <br>
            {{ event.comment }}
          </p>
        </AccordionTab>
      </Accordion>
    </div>
  </Fieldset>
</template>

<script setup>
import DOMPurify from 'dompurify'

// eslint-disable-next-line vue/require-prop-types
const props = defineProps(['fields', 'data'])

function getSortedEventList(fieldKey) {
  return getKeyValue(fieldKey).map(e => eventWithMetadata(e, fieldKey)).sort((a, b) => a.date - b.date)
}

function eventWithMetadata(event, fieldKey) {
  const field = props.fields.find(f => f.key === fieldKey)
  const field_type_metadata = field.field_type_metadata
  const current_metadata = field_type_metadata?.event_types?.find(m => m.value === event.type)

  return {
    ...event,
    date: new Date(event.date),
    title: current_metadata.label,
    severity: eventColorToSeverity(current_metadata.color),
  }
}

function eventColorToSeverity(color) {
  switch (color) {
    case 'neutral':
      return 'secondary'
    case 'success':
      return 'success'
    case 'fail':
      return 'danger'
    case 'warning':
      return 'warning'
    default:
      return 'secondary'
  }
}

function isUrlField(key) {
  const field = props.fields.find(f => f.key === key)
  return field.field_type_metadata?.format === 'url'
}

function getDataAsEscapedString(key) {
  const txt = props.data[key] ?? ''
  return DOMPurify.sanitize(txt).replaceAll('\n', '<br />')
}

function extractHostnameFromUrl(url) {
  try {
    const u = new URL(url)
    const hostname = u.hostname
    return hostname.startsWith('www.') ? hostname.substring(4) : hostname
  }
  catch (e) {
    return url
  }
}

function hasRealValue(value) {
  const result = value !== undefined && value !== null && value !== ''
  if (typeof value === 'string') {
    return result && value.trim() !== ''
  }
  return value
}

function fieldsToDisplay() {
  const fields = props.fields.slice()
    .sort((a, b) => a.display_weight - b.display_weight)
    .filter(f => hasKey(f.key))
    .filter(f => hasRealValue(getKeyValue(f.key)))
    .filter(f => !(
      f.field_type === 'EnumSingleOption'
      && f.field_type_metadata?.options.find(o => o.value == getKeyValue(f.key))?.hidden),
    )

  return fields
}

function getKeyValue(key) {
  return props.data[key]
}

function getValueForEnum(key, value) {
  const field = props.fields.find(f => f.key === key)
  return field.field_type_metadata.options.find(o => o.value === value)?.label
}

function hasKey(key) {
  return props.data[key] !== undefined
    && props.data[key] !== null
}
</script>
