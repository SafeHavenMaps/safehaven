// eslint-disable-next-line @typescript-eslint/ban-ts-comment
// @ts-nocheck
import theme from './theme.mjs'

export default defineNuxtConfig({
  modules: [
    '@primevue/nuxt-module',
    '@nuxt/eslint',
    '@nuxtjs/tailwindcss',
  ],
  ssr: false,
  devtools: { enabled: true },
  typescript: {
    typeCheck: true,
    strict: true,
  },
  primevue: {
    options: {
      theme: {
        preset: theme,
        options: {
          darkModeSelector: 'none',
        },
      },
    },
  },
  css: [
    '~/assets/main.css',
    '~/assets/richtext.css',
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
