<template>
  <div class="w-full">
    <h3 class="mt-0">
      Origines des liens
    </h3>

    <DataTable :value="origins">
      <Column
        field="referrer"
        header="Origine"
        sortable
      />

      <Column
        field="count"
        header="Nombre de visites"
        sortable
      />
    </DataTable>

    <div class="mt-3">
      <h3 class="mt-0">
        Activités des 30 derniers jours
      </h3>

      <div class="relative flex h-full w-full justify-content-center ">
        <Chart
          type="bar"
          :data="chartData"
          :options="chartOptions"
          class="absolute top-0 left-0 h-full w-full h-13rem xl:h-16rem"
        />
      </div>
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

const accessTokenId = useRoute().params.id as string

const token = await state.client.getAccessToken(accessTokenId)
const stats = await state.client.getAccessTokenStats(accessTokenId)

const origins = Object.entries(stats.origins).map(([referrer, count]) => ({ referrer, count }))

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

const initAdminLayout = inject<InitAdminLayout>('initAdminLayout')!
initAdminLayout(
    `Détails du jeton ${token.title}`,
    'accessToken',
    [],
    [
      { label: 'Jetons d\'accès', url: '/admin/access-tokens' },
      { label: `Détails du jeton ${token.title}`, url: `/admin/access-tokens/${accessTokenId}` },
    ],
)
</script>
