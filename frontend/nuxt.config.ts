export default defineNuxtConfig({
  modules: ['nuxt-primevue', '@nuxt/eslint'],
  ssr: false,
  devtools: { enabled: true },
  typescript: {
    typeCheck: true,
    strict: true,
  },
  css: [
    '~/assets/main.css',
    'primeflex/primeflex.css',
    'primevue/resources/themes/aura-light-pink/theme.css',
  ],
  eslint: {
    config: {
      stylistic: true,
    },
  },
  app: {
    head: {
      title: 'Safehaven',
    },
  },
})
