import OpenLayersMap from 'vue3-openlayers'
import { defineNuxtPlugin } from '#app'
import 'vue3-openlayers/dist/vue3-openlayers.css'

export default defineNuxtPlugin((nuxtApp) => {
  nuxtApp.vueApp.use(OpenLayersMap)
})
