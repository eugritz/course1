<script setup lang="ts">
defineProps<{
  minSize?: string,
  size?: string,
}>();
</script>

<template>
  <div class="splitter__splitter-panel">
    <slot />
  </div>
</template>

<style scoped lang="scss">
</style>
