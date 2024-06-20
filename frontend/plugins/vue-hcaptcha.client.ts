import VueHcaptcha from '@hcaptcha/vue3-hcaptcha'
import { defineNuxtPlugin } from '#app'

export default defineNuxtPlugin((nuxtApp) => {
  nuxtApp.vueApp.component('VueHcaptcha', VueHcaptcha)
})
