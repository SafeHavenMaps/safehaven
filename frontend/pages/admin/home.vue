<template>
  <div class="grid">
    <div class="col-12 md:col-6 xl:col-2">
      <Card class="card-green">
        <template #title>
          <div class="white-space-nowrap text-sm flex">
            <AppIcon
              icon-name="entity"
              size="16px"
              style="display: inline"
            />&nbsp;
            Entités
          </div>
        </template>
        <template #content>
          <div class="text-4xl font-bold">
            {{ stats.total_entities }}
          </div>
        </template>
      </Card>
    </div>

    <div class="col-12 md:col-6 xl:col-2">
      <Card class="card-green">
        <template #title>
          <div class="white-space-nowrap text-sm flex">
            <AppIcon
              icon-name="comment"
              size="16px"
              style="display: inline"
            />&nbsp;
            Commentaires
          </div>
        </template>
        <template #content>
          <div class="text-4xl font-bold">
            {{ stats.total_comments }}
          </div>
        </template>
      </Card>
    </div>

    <div class="col-12 md:col-6 xl:col-2">
      <Card class="card-orange">
        <template #title>
          <div class="white-space-nowrap text-sm flex">
            <AppIcon
              icon-name="addEntity"
              size="16px"
              style="display: inline"
            />&nbsp;
            Entités
          </div>
        </template>
        <template #content>
          <div class="text-4xl font-bold">
            {{ stats.pending_entities }}
          </div>
        </template>
      </card>
    </div>

    <div class="col-12 md:col-6 xl:col-2">
      <Card class="card-orange">
        <template #title>
          <div class="white-space-nowrap text-sm flex">
            <AppIcon
              icon-name="addComment"
              size="16px"
              style="display: inline"
            />&nbsp;
            Commentaires
          </div>
        </template>
        <template #content>
          <div class="text-4xl font-bold">
            {{ stats.pending_comments }}
          </div>
        </template>
      </card>
    </div>

    <div class="col-12 md:col-6 xl:col-2">
      <Card class="card-blue">
        <template #title>
          <div class="white-space-nowrap text-sm flex">
            <AppIcon
              icon-name="clock"
              size="16px"
              style="display: inline"
            />&nbsp;
            Visites (30j)
          </div>
        </template>
        <template #content>
          <div class="text-4xl font-bold">
            {{ stats.total_visits_30_days }}
          </div>
        </template>
      </card>
    </div>

    <div class="col-12 md:col-6 xl:col-2">
      <Card class="card-blue">
        <template #title>
          <div class="white-space-nowrap text-sm flex">
            <AppIcon
              icon-name="clock"
              size="16px"
              style="display: inline"
            />&nbsp;
            Visites (7j)
          </div>
        </template>
        <template #content>
          <div class="text-4xl font-bold">
            {{ stats.total_visits_7_days }}
          </div>
        </template>
      </card>
    </div>

    <div
      class="col-12 p-4"
    >
      <!-- <Chart
        type="line"
        :data="chartData"
        :options="chartOptions"
      /> -->
    </div>
  </div>
</template>

<script setup lang="ts">
// import Chart from 'primevue/chart'
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
      fill: false,
      borderColor: '#E86BA7',
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
  },
  preserveAspectRatio: false,
  scales: {
    x: {
      display: true,
      title: {
        text: 'Date',
      },
    },
    y: {
      display: true,
    },
  },
}
</script>

<style>
.card-green {
  background-color: #6fac72;
  color: white;
}

.card-orange {
  background-color: #ef9444;
  color: white;
}

.card-blue {
  background-color: #E86BA7;
  color: white;
}

.p-card-body {
  padding: 0.75rem;
}

.p-card-content {
  margin-top: 0.25rem;
}
</style>
