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


  @if $theme == light {
    border: 2px solid #000000;
  }
  @if $theme == dark {
    border: 2px solid #ffffff;
  }

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
</style>
