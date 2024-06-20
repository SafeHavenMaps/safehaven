<template>
  <div class="h-full w-full">
    <div class="grid grid-cols-12 gap-4">
      <div
        v-for="(card, index) in cards"
        :key="index"
        class="col-span-12 md:col-span-6 xl:col-span-2 relative"
      >
        <Card :class="card.class + ' relative'">
          <template #title>
            <div class="whitespace-nowrap text-sm flex">
              <AppIcon
                :icon-name="card.iconName"
                size="16px"
                style="display: inline"
              />&nbsp;
              {{ card.title }}
            </div>
          </template>
          <template #content>
            <div class="text-4xl font-bold z-10">
              {{ card.stat }}
            </div>
          </template>
        </Card>
      </div>
    </div>

    <div class="flex flex-col h-full w-full items-center ">
      <div class="text-xl font-bold mt-8">
        Activités des 30 derniers jours
      </div>

      <Chart
        type="bar"
        :data="chartData"
        :options="chartOptions"
        class="h-full w-full max-h-96 xl:mt-12 xl:h-[25rem] !mt-6"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import Chart from 'primevue/chart'
import type { InitAdminLayout } from '~/layouts/admin-ui.vue'
import type { HomePageStats } from '~/lib'
import state from '~/lib/admin-state'

definePageMeta({
  layout: 'admin-ui',
})

const initAdminLayout = inject<InitAdminLayout>('initAdminLayout')!
initAdminLayout(
  'Accueil',
  'home',
  [],
  [
    { label: 'Accueil', url: '/admin/home' },
  ],
)

const stats = ref<HomePageStats>({
  total_entities: 0,
  total_comments: 0,
  pending_entities: 0,
  pending_comments: 0,
  total_visits_30_days: 0,
  total_visits_7_days: 0,
  visits_30_days: {},
})

try {
  const statsResponse = await state.client.getStats()
  stats.value = statsResponse
}
catch {
  // Do nothing
}

const chartData = {
  labels: Object.keys(stats.value!.visits_30_days).map(date => new Date(date).toLocaleDateString()),
  datasets: [
    {
      label: 'Visites',
      data: Object.values(stats.value!.visits_30_days),
      fill: true,
      borderColor: '#e86ba7',
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      backgroundColor: (context: any) => {
        const chart = context.chart
        const { ctx, chartArea } = chart

        if (!chartArea) {
          return
        }
        return getGradient(ctx, chartArea)
      },
      tension: 0.3,
    },
  ],
}

const chartOptions = {
  responsive: true,
  plugins: {
    legend: {
      display: false,
    },
    tooltip: {
      enabled: true,
      mode: 'index',
      intersect: false,
      callbacks: {
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
        label: (context: any) => {
          let label = context.dataset.label || ''
          if (label) {
            label += ': '
          }
          if (context.parsed.y !== null) {
            label += context.parsed.y
          }
          return label
        },
      },
    },
  },
  maintainAspectRatio: false,
  scales: {
    x: {
      display: false,
    },
    y: {
      display: true,
    },
  },
}

function getGradient(ctx: CanvasRenderingContext2D, chartArea: { top: number, bottom: number }) {
  const gradient = ctx.createLinearGradient(0, chartArea.top, 0, chartArea.bottom)
  gradient.addColorStop(0, 'rgba(232, 107, 167, 0.9)')
  gradient.addColorStop(1, 'rgba(232, 107, 167, 0)')
  return gradient
}

const cards = [
  {
    class: 'card-green',
    iconName: 'entity',
    title: 'Entités',
    stat: stats.value.total_entities,
  },
  {
    class: 'card-green',
    iconName: 'comment',
    title: 'Commentaires',
    stat: stats.value.total_comments,
  },
  {
    class: 'card-orange',
    iconName: 'addEntity',
    title: 'Entités',
    stat: stats.value.pending_entities,
  },
  {
    class: 'card-orange',
    iconName: 'addComment',
    title: 'Commentaires',
    stat: stats.value.pending_comments,
  },
  {
    class: 'card-blue',
    iconName: 'clock',
    title: 'Visites (30j)',
    stat: stats.value.total_visits_30_days,
  },
  {
    class: 'card-blue',
    iconName: 'clock',
    title: 'Visites (7j)',
    stat: stats.value.total_visits_7_days,
  },
]
</script>

<style>
.card-green {
  background-color: #6fac72 !important;
  color: white !important;
  position: relative !important;
  overflow: hidden !important;
}

.card-orange {
  background-color: #ef9444 !important;
  color: white !important;
  position: relative !important;
  overflow: hidden !important;
}

.card-blue {
  background-color: #E86BA7 !important;
  color: white !important;
  position: relative !important;
  overflow: hidden !important;
}

.p-card-body {
  padding: 0.75rem !important;
}

.p-card-content {
  margin-top: 0.25rem !important;
}
</style>
