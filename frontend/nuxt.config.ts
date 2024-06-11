// eslint-disable-next-line @typescript-eslint/ban-ts-comment
// @ts-nocheck
import theme from './theme.mjs'

export default defineNuxtConfig({
  modules: [
    '@primevue/nuxt-module',
    '@nuxt/eslint',
    '@nuxtjs/tailwindcss',
    '@vueuse/nuxt',
  ],
  ssr: false,
  devtools: { enabled: true },
  typescript: {
    typeCheck: true,
    strict: true,
  },
  primevue: {
    options: {
      ripple: true,
      theme: {
        preset: theme,
        options: {
          darkModeSelector: '.sh-dark',
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
  vite: {
    resolve: {
      alias: {
        ace: 'ace-builds/src-noconflict',
      },
    },
  },
})
