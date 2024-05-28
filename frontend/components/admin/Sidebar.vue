<template>
  <PanelMenu
    :model="getMenuItems()"
    class="navigation-tree mt-4"
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
            :dynamic-type="item.icon_dytpe"
          />
          <span
            class="ml-2 flex-grow-1 text-link"
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
          :dynamic-type="item.icon_dytpe"
        />

        <span
          class="ml-2 flex-grow-1 text-link"
        >{{ item.label }}</span>

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

await state.fetchFamilies()

function getMenuItems() {
  const currentRoute = useRoute().fullPath

  return nodes.map((node) => {
    node.active = (!!node.route && currentRoute.startsWith(`/admin/${node.route}`))

    return node
  })
}

function classForLink(active: boolean) {
  const classes = ['flex', 'align-items-center', 'cursor-pointer', 'text-color', 'px-3', 'py-2']

  if (active) {
    classes.push('active-link')
  }

  return classes
}

const familyNodes = [{
  label: 'Configuration',
  icon: 'config',
  route: 'families',
  active: false,
}].concat(
  state.families.map(item => ({
    label: item.title,
    key: item.id,
    icon: item.icon_hash!,
    icon_dytpe: 'families',
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
        label: 'Commentaires',
        icon: 'comment',
        route: item.id + '/comments',
      },
      {
        label: 'Entités en attente',
        icon: 'pendingEntity',
        route: item.id + '/pending-entities',
      },
      {
        label: 'Commentaires en attente',
        icon: 'pendingComment',
        route: item.id + '/pending-categories',
      },
    ],
  })),
)

const nodes = [
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
    items: familyNodes,
    active: false,
  },
  {
    label: 'Tags',
    icon: 'tag',
    route: 'tag',
    active: false,
  },
]
</script>

<style>
.active-link {
  color: #e86ba7 !important;
}

.p-panelmenu-panel {
  border-width: 0;
}

.p-panelmenu-header-content {
  cursor: pointer;
  background-color: transparent;
}

.p-panelmenu-content {
  background-color: transparent;
}

.arrow-icon {
  transform: rotate(0deg);
  transition: all 0.2s ease-in-out;

}

div[aria-expanded="true"] .arrow-icon,
li[aria-expanded="true"] .arrow-icon {
  transform: rotate(-180deg);
  transition: all 0.2s ease-in-out;
}

a {
  text-decoration: none;
}

.navigation-tree {
  background-color: transparent;
}
</style>
