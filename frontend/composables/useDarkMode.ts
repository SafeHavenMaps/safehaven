const localStorageKey = 'sh-dark-mode-enabled'

// Create the localStorage value if it doesn't exist
if (!localStorage.getItem(localStorageKey)) {
  localStorage.setItem(localStorageKey, window.matchMedia('(prefers-color-scheme: dark)').matches.toString())
}

const localStorageValue = localStorage.getItem(localStorageKey) === 'true'

const isDark = ref(localStorageValue)
const isLight = computed(() => !isDark.value)

const apply = () => {
  if (isDark.value) {
    document.documentElement.classList.add('sh-dark')
  }
  else {
    document.documentElement.classList.remove('sh-dark')
  }
}

const toggle = () => {
  isDark.value = !isDark.value
  localStorage.setItem(localStorageKey, isDark.value.toString())
  apply()
}

export default function useDarkMode() {
  return { isDark, isLight, apply, toggle }
}
