export default defineNuxtConfig({
  modules: [
    "nuxt-primevue",
    "nuxt-mdi",
  ],
  ssr: false,
  devtools: { enabled: true },
  typescript: {
    typeCheck: true,
  },
  runtimeConfig: {
    public: {
      apiUrl: process.env.API_URL || "/",
    },
  },
  css: [
    "~/assets/main.css",
    "primeflex/primeflex.css",
    "primevue/resources/themes/aura-light-pink/theme.css",
  ],
  mdi: {
    cache: false,
    componentName: 'MdiIcon',
    defaultSize: '1em'
  }
});
