<script setup lang="ts">
import HomeButton from "./HomeButton.vue";
</script>

<template>
  <header>
    <main>
      <home-button>Главная</home-button>
      <home-button>Словарь</home-button>
      <home-button>Статистика</home-button>
    </main>
  </header>
</template>

<style scoped lang="scss">
@import "../styles/theme";

header {
  display: flex;
  justify-content: center;
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
}

main {
  border-radius: 0% 0% 10px 10px;
  box-shadow: 0 5px 50px rgba(0, 0, 0, 0.2);
}

@if $theme == dark {
  main {
    box-shadow: 0 5px 100px rgba(255, 255, 255, 0.05);
  }
}
</style>
