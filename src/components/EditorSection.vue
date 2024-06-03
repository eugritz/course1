<script lang="ts">
export interface EditorSectionExposed {
  inputRef: HTMLInputElement | null;
}
</script>

<script setup lang="ts">
import { ref } from 'vue';

defineProps<{
  title: string,
  placeholder?: string,
  type?: "text" | "tags",
}>();

const isSectionOpen = ref(true);
const value = defineModel();
</script>

<template>
  <div class="section" :class="{ 'section--closed': !isSectionOpen }">
    <div class="section__header" @click="isSectionOpen = !isSectionOpen">
      <div class="section__header__title">
        <span>
          <unicon
            class="section__header__title__icon"
            name="angle-down"
            fill="white"
            width="20"
            height="20"
          />
        </span>
        <span>{{title}}</span>
      </div>
      <div v-if="type !== 'tags'" class="section__header__controls">
        <span>
          <unicon name="arrow" fill="white" width="20" height="20" />
        </span>
        <span>
          <unicon name="custom-pin" fill="white" width="18" height="18" />
        </span>
      </div>
    </div>
    <div v-show="isSectionOpen" class="section__content">
      <input
        ref="inputRef"
        type="text"
        v-model="value"
        :placeholder="placeholder"
      />
    </div>
  </div>
</template>

<style scoped lang="scss">
@import "../styles/mixins";

.section {
  text-align: left;
  @include user-select-none;
}

.section__header {
  padding: 2px 5px;
  display: flex;
  justify-content: space-between;
  cursor: pointer;
}

.section__header__title,
.section__header__controls {
  display: flex;
  gap: 8px;
}

.section__header__controls {
  align-items: center;
}
</style>

<style lang="scss">
.section__content {
  input[type="text"] {
    width: 100%;
  }
}

.section--closed {
  .section__header__title__icon {
    transform: rotate(-90deg);
  }
}
</style>
