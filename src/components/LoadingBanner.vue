<script setup lang="ts">
import Loader from './Loader.vue';

const show = defineModel<boolean>();
</script>

<template>
  <Teleport to="body">
    <div v-if="show" class="banner">
      <Loader class="loader" />
    </div>
  </Teleport>
</template>

<style scoped lang="scss">
@import "../styles/mixins";

.banner {
  position: absolute;
  left: 24px;
  top: calc(100% - 24px);
  transform: translate(-50%, -50%);
  padding: 1em;
  border-radius: 20px;
  font-size: 18px;
  background-color: rgba(0, 0, 0, 0.75);
  pointer-events: none;
  @include user-select-none;
}

.banner-enter-active {
  transition: opacity 0.2s ease;
}

.banner-leave-active {
  transition: opacity 0.3s ease;
}

.banner-enter-from,
.banner-leave-to {
  opacity: 0;
}

.loader {
  margin-top: -12px;
  margin-left: -12px;
}
</style>
