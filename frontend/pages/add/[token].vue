<template>
  <div class="flex flex-col justify-center items-center h-full gap-8 hot-pink-bg">
    <StartPopup />

    <Card class="p-3 m-2">
      <template #title>
        <div class="flex items-center">
          <img
            height="40"
            width="40"
            alt="icon"
            :src="state.logo ?? defaultLogo"
          >
          <div class="pl-2 xl:pl-4">
            <div class="my-0 text-lg font-extrabold">
              {{ state.title }}
            </div>
            <div class="text-xs italic">
              {{ state.subtitle }}
            </div>
          </div>
        </div>
      </template>
      <template #content>
        <div class="flex flex-col gap-6">
          <Divider class="!mb-0" />
          <ViewerFamilySwitcher />

          <Button
            :label="$t('page.add.token.add')"
            @click="entityAddForm!.open()"
          >
            <template #icon>
              <AppIcon
                icon-name="addEntity"
              />
            </template>
          </Button>
          <ViewerEntityAddForm
            ref="entityAddForm"
            :family="state.activeFamily"
            :categories="
              state.categories
                .filter(category => category.family_id == state.activeFamily.id)
            "
          />
        </div>
      </template>
    </Card>
    <Toast />
  </div>
</template>

<script setup lang="ts">
import state from '~/lib/viewer-state'
import defaultLogo from '~/assets/logo_square.svg'
import type { ViewerEntityAddForm } from '#build/components'

const toast = useToast()
const { t } = useI18n()

// Init state with url token
const route = useRoute()
const token = route.params.token as string
try {
  await state.bootstrapWithToken(token)
  if (!state.permissions?.can_add_entity)
    throw 'Unauthorized'
}
catch {
  toast.add({
    severity: 'error',
    summary: t('page.add.token.error'),
    detail: t('page.add.token.couldNotLoadMap'),
    life: 3000,
  })
  if (state.redirectUrl) {
    window.location.href = state.redirectUrl
  }
  else {
    throw createError({
      statusCode: 404,
      statusMessage: 'Page Not Found',
      fatal: true,
    })
  }
}

const entityAddForm = ref<InstanceType<typeof ViewerEntityAddForm>>()
</script>
