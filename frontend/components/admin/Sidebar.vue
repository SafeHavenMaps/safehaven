<template>
  <PanelMenu
    :model="menuItems"
    class="navigation-tree mt-6"
    style="width: 17rem;"
  >
    <template #item="{ item }">
      <NuxtLink
        v-if="item.route"
        v-slot="{ href, navigate }"
        :to="`/admin/${item.route}`"
        custom
      >
        <a
          :class="classForLink(item.active!)"
          :href="href"
          @click="navigate"
        >
          <AppIcon
            size="18px"
            :icon-name="item.icon!"
            :dynamic="item.iconDynamic"
          />
          <Badge
            v-if="item.pending_count"
            :value="item.pending_count"
            severity="danger"
          />
          <span
            class="grow text-link"
          >{{ item.label }}</span>
        </a>
      </NuxtLink>

      <a
        v-else
        :class="classForLink(item.active!)"
        :href="item.url"
        :target="item.target"
      >
        <AppIcon
          size="18px"
          :icon-name="item.icon!"
          :dynamic="item.iconDynamic"
        />
        <Badge
          v-if="item.pending_count"
          :value="item.pending_count"
          severity="danger"
        />
        <span
          class="grow text-link"
        >
          {{ item.label }}
        </span>

        <AppIcon
          v-if="item.items"
          class="arrow-icon"
          size="16px"
          icon-name="chevronDown"
        />
      </a>
    </template>
  </PanelMenu>
</template>

<script setup lang="ts">
import state from '~/lib/admin-state'

try {
  await state.fetchFamilies()
  await state.getEntitiesCommentsCounts()
}
catch {
  // Do nothing
}

const currentRoute = useRoute()

const menuItems = computed(() => {
  return nodes.value.map((node) => {
    node.active = (!!node.route && currentRoute.fullPath.startsWith(`/admin/${node.route}`))
    return node
  })
})

function classForLink(active: boolean) {
  const classes = ['flex', 'items-center', 'cursor-pointer', 'text-color', 'px-4', 'py-2', 'gap-2']

  if (active) {
    classes.push('active-link')
  }

  return classes
}

const familyNodes = computed(() => [{
  label: 'Gestion',
  icon: 'listEdit',
  route: 'families',
  active: false,
}].concat(
  state.families.map((item) => {
    const counts = state.countsByFamily[item.id] ?? [0, 0, 0, 0]
    return {
      label: item.title,
      key: item.id,
      icon: item.icon_hash!,
      pending_count: counts[1] + counts[3],
      iconDynamic: true,
      route: '',
      active: false,
      items: [
        {
          label: 'Catégories',
          icon: 'category',
          route: item.id + '/categories',
        },
        {
          label: 'Entités',
          icon: 'entity',
          route: item.id + '/entities',
        },
        {
          label: 'Entités en attente',
          icon: 'pendingEntity',
          pending_count: counts[1],
          route: item.id + '/entities/pending',
        },
        {
          label: 'Commentaires en attente',
          icon: 'pendingComment',
          pending_count: counts[3],
          route: item.id + '/comments/pending',
        },
      ],
    }
  }),
))

const nodes = computed(() => [
  {
    label: 'Accueil',
    icon: 'home',
    route: 'home',
    active: false,
  },
  {
    label: 'Configuration',
    icon: 'config',
    route: 'config',
    active: false,
  },
  {
    label: 'Utilisateur⋅ices',
    icon: 'user',
    route: 'users',
    admin_only: true,
    active: false,
  },
  {
    label: 'Jetons d\'accès',
    icon: 'accessToken',
    route: 'access-tokens',
    active: false,
  },
  {
    label: 'Familles',
    icon: 'family',
    pending_count: Object.values(state.countsByFamily).reduce((summed_count, counts) => summed_count + counts[1] + counts[3], 0),
    items: familyNodes.value,
    active: false,
  },
  {
    label: 'Tags',
    icon: 'tag',
    route: 'tags',
    active: false,
  },
].filter(node => !node.admin_only || state.is_admin))
</script>

<style>
.active-link {
  color: #e86ba7 !important;
}

.p-panelmenu-panel {
  border-width: 0 !important;
  background: transparent !important;
}

.p-panelmenu-header-content {
  cursor: pointer !important;
  background-color: transparent !important;
}

.p-panelmenu-content {
  background-color: transparent !important;
}

.arrow-icon {
  transform: rotate(0deg) !important;
  transition: all 0.2s ease-in-out !important;

}

div[aria-expanded="true"] .arrow-icon,
li[aria-expanded="true"] .arrow-icon {
  transform: rotate(-180deg) !important;
  transition: all 0.2s ease-in-out !important;
}

a {
  text-decoration: none !important;
}

.navigation-tree {
  background-color: transparent !important;
}
</style>
