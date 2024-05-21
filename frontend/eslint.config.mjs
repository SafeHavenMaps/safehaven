// @ts-check
import stylistic from '@stylistic/eslint-plugin'
import withNuxt from './.nuxt/eslint.config.mjs'

export default withNuxt(
  stylistic.configs['recommended-flat'],
  {
    rules: {
      'vue/no-v-html': 'off',
    },
  },
)
