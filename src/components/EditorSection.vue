<script setup lang="ts">
import {ref} from 'vue';

const props = defineProps<{
  title: string,
  value?: string,
  placeholder?: string,
  type?: "text" | "tags",
}>();

const isSectionClosed = ref(true);
const value = ref(props.value ?? "");
</script>

<template>
  <div class="section" :class="{ 'section--closed': !isSectionClosed }">
    <div class="section__header" @click="isSectionClosed = !isSectionClosed">
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
    <div v-show="isSectionClosed" class="section__content">
      <slot>
        <input type="text" v-model="value" :placeholder="placeholder" />
      </slot>
    </div>
  </div>
</template>

<style scoped lang="scss">
@import "../styles/mixins";

.section {
  text-align: left;
}

.section__header {
  padding: 2px 5px;
  display: flex;
  justify-content: space-between;
  @include user-select-none;
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