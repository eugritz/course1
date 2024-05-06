<template>
  <span class="loader"></span>
</template>

<style scoped lang="scss">
@import "../styles/theme";

.loader {
  position: absolute;
  width: 24px;
  height: 24px;
  display: inline-block;
  border: 1px solid #000000;
  border-bottom-color: transparent;
  border-radius: 50%;
  box-sizing: border-box;
  animation: rotation 1s linear infinite;
}

@keyframes rotation {
  0% {
    transform: rotate(0deg);
  }
  100% {
    transform: rotate(360deg);
  }
} 

@if $theme == dark {
  .loader {
    border-width: 2px;
    border-color: #ffffff;
  }
}
</style>
