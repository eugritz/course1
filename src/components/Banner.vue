<script setup lang="ts">
const show = defineModel<boolean>();

function handleEnter() {
  setTimeout(() => {
    show.value = false;
  }, 1500);
}
</script>

<template>
  <Teleport to="body">
    <Transition name="banner" @enter="handleEnter">
      <div class="banner" v-if="show">
        <slot />
      </div>
    </Transition>
  </Teleport>
</template>

<style scoped lang="scss">
@import "../styles/mixins";

.banner {
  position: absolute;
  left: 50%;
  top: 50%;
  transform: translate(-50%, -50%);
  padding: 2em;
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
</style>
