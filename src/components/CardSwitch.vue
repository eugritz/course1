<script setup lang="ts">
import { ref, watch } from 'vue';

const checked = ref(false);
const transition = ref(false);

function handleChange() {
  transition.value = true;
  setTimeout(() => {
    transition.value = false;
  }, 200);
}
</script>

<template>
  <div class="card-switch" :class="{ checked }">
    <input
      class="card-switch__input"
      type="checkbox"
      role="switch"
      v-model="checked"
      @change="handleChange"
    />
    <span class="card-switch__slider"></span>
    <span class="card-switch__label--left" v-show="!checked && !transition">
      Карты
    </span>
    <span class="card-switch__label--right" v-show="checked && !transition">
      Записи
    </span>
  </div>
</template>

<style scoped lang="scss">
@import "../styles/theme";

.card-switch {
  z-index: 1;
  position: relative;
  width: 6rem;
  height: 1.9rem;
  margin-right: 2px;
  display: inline-flex;
  align-items: center;
  padding: 0.8rem;
  box-sizing: border-box;
}

.card-switch__input {
  appearance: none;
  position: absolute;
  left: 0;
  top: 0;
  width: 100%;
  height: 100%;
  margin: 0;
  padding: 0;
  border: 0;
  box-sizing: border-box;
  box-shadow: none;
  opacity: 0;
  cursor: pointer;
}

.card-switch__slider {
  position: absolute;
  left: 0;
  top: 0;
  width: 100%;
  height: 100%;
  pointer-events: none;
  background: #93c5fd;
  border-radius: 30px;

  &::before {
    content: "";
    position: absolute;
    top: 50%;
    left: 0.25rem;
    width: 1.5rem;
    height: 1.5rem;
    margin-top: -0.75rem;
    border-radius: 50%;
    transition: 0.2s linear;
    background: #ffffff;

    @if $theme == dark {
      background: #454545;
    }
  }
}

.card-switch.checked {
  .card-switch__slider {
    background: #4ade80;

    &::before {
      transform: translateX(4rem);
    }
  }
}

.card-switch__label--left,
.card-switch__label--right {
  z-index: 2;
  position: relative;
  width: 100%;
  font-size: 14px;
  font-weight: 700;
  color: #ffffff;
  pointer-events: none;

  @if $theme == dark {
    color: #2f2f2f;
  }
}

.card-switch__label--left {
  margin-left: 1.5rem;
}

.card-switch__label--right {
  margin-right: 1.5rem;
}
</style>
