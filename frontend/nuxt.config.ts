export default defineNuxtConfig({
  modules: ["nuxt-primevue"],
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
});
