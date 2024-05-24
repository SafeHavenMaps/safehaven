<template>
  <div class="h-full">
    <AdminNavbar />
    <div
      class="h-full flex"
    >
      <div
        class="h-full flex"
      >
        <AdminSidebar />
      </div>
      <div
        class="h-full flex"
      >
        <slot />
      </div>
    </div>
  </div>
</template>
