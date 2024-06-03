<script setup lang="ts">
defineProps<{
  field: string,
  header?: string,
} | {
  value: string,
  header?: string,
}>();
</script>

<template>
  <slot />
</template>

<style scoped lang="scss">
</style>
