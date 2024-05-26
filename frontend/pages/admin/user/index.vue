<template>
  <div>
    <DataTable
      :value="state.users"
      striped-rows
      :table-style="{ 'min-width': '42rem' }"
      :rows="10"
      :rows-per-page-options="[10, 20, 50]"
      removable-sort
    >
      <template #header>
        <div class="flex align-items-center justify-content-between">
          <h4 class="flex m-0 align-items-end ">
            <AppIcon
              icon-name="user"
              class="mr-2"
            />
            Utilisateur⋅ices
          </h4>
          <Button
            outlined
            rounded
            severity="success"
            @click="navigateTo('/admin/users/add')"
          >
            <template #icon>
              <AppIcon icon-name="add" />
            </template>
          </Button>
        </div>
      </template>

      <Column
        field="name"
        header="Nom"
        sortable
      />
      <Column
        header="Droits"
        field="is_admin"
        sortable
      >
        <template #body="slotProps">
          <Tag
            :value="slotProps.data.is_admin ? 'Administrateur⋅ice' : 'Modérateur⋅ice'"
            :severity="slotProps.data.is_admin ? 'success' : 'warning'"
          />
        </template>
      </Column>

      <Column
        field="id"
        header="Id"
      />

      <Column>
        <template #body="slotProps">
          <Button
            outlined
            rounded
            class="mx-2"
            severity="warning"
            @click="() => navigateTo(`/admin/users/edit/${slotProps.data.id}`)"
          >
            <template #icon>
              <AppIcon icon-name="edit" />
            </template>
          </Button>

          <Button
            outlined
            rounded
            severity="danger"
            :disabled="state.username==slotProps.data.name"
            @click="onDelete($event as Event, slotProps.data)"
          >
            <template #icon>
              <AppIcon icon-name="delete" />
            </template>
          </Button>
        </template>
      </Column>
    </DataTable>

    <ConfirmationDialog
      ref="confirmationDialogRef"
      message-object-descriptor="l'utilisateur⋅ice"
    />
  </div>
</template>

<script setup lang="ts">
import ConfirmationDialog from '~/components/ConfirmationDialog.vue'
import type { User } from '~/lib'
import state from '~/lib/admin-state'

definePageMeta({
  layout: 'admin-ui',
})
await state.fetchUsers()

const confirmationDialogRef: Ref<InstanceType<typeof ConfirmationDialog> | null> = ref(null)

function onDelete(event: Event, user: User) {
  confirmationDialogRef.value!.displayDialog(event, user.name, async () => await state.deleteUser(user.id))
}
// async createUser(newUser: NewUser): Promise<void>

// async getUser(id: string): Promise<User> {

// async updateUserPassword(id: string, newPasswordDetails: ChangePassword): Promise<void> {
</script>
