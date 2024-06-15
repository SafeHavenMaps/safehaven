<template>
  <div class="relative h-full w-full">
    <div class="grid">
      <div
        v-for="(card, index) in cards"
        :key="index"
        class="col-12 md:col-6 xl:col-2 relative"
      >
        <Card :class="card.class + ' relative'">
          <template #title>
            <div class="white-space-nowrap text-sm flex">
              <AppIcon
                :icon-name="card.iconName"
                size="16px"
                style="display: inline"
              />&nbsp;
              {{ card.title }}
            </div>
          </template>
          <template #content>
            <div class="text-4xl font-bold relative z-10">
              {{ card.stat }}
            </div>
          </template>
        </Card>
      </div>
    </div>

    <div class="relative flex h-full w-full justify-content-center mt-2">
      <div class="text-xl font-bold mb-3">
        Activités des 30 derniers jours
      </div>
      <Chart
        type="line"
        :data="chartData"
        :options="chartOptions"
        class="absolute top-0 left-0 h-full w-full mt-3 xl:h-25rem"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import Chart from 'primevue/chart'
import type { InitAdminLayout } from '~/layouts/admin-ui.vue'
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

const stats = await state.client.getStats()

const chartData = {
  labels: Object.keys(stats.visits_30_days).map(date => new Date(date).toLocaleDateString()),
  datasets: [
    {
      label: 'Visites',
      data: Object.values(stats.visits_30_days),
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
      display: false,
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
    stat: stats.total_entities,
  },
  {
    class: 'card-green',
    iconName: 'comment',
    title: 'Commentaires',
    stat: stats.total_comments,
  },
  {
    class: 'card-orange',
    iconName: 'addEntity',
    title: 'Entités',
    stat: stats.pending_entities,
  },
  {
    class: 'card-orange',
    iconName: 'addComment',
    title: 'Commentaires',
    stat: stats.pending_comments,
  },
  {
    class: 'card-blue',
    iconName: 'clock',
    title: 'Visites (30j)',
    stat: stats.total_visits_30_days,
  },
  {
    class: 'card-blue',
    iconName: 'clock',
    title: 'Visites (7j)',
    stat: stats.total_visits_7_days,
  },
]
</script>

<style>
.card-green {
  background-color: #6fac72;
  color: white;
  position: relative;
  overflow: hidden;
}

.card-orange {
  background-color: #ef9444;
  color: white;
  position: relative;
  overflow: hidden;
}

.card-blue {
  background-color: #E86BA7;
  color: white;
  position: relative;
  overflow: hidden;
}

.p-card-body {
  padding: 0.75rem;
}

.p-card-content {
  margin-top: 0.25rem;
}

.p-chart {
  border-left: 1px dashed #DDD;
  border-right: 1px dashed #DDD;
  border-bottom: 1px dashed #DDD;
}
</style>
