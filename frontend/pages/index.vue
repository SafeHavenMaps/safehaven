<script setup lang="ts">
import state from '~/lib/viewer-state'

if (state.redirectUrl) {
  window.location.href = state.redirectUrl
}
</script>
