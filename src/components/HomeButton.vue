<template>
  <button><slot /></button>
</template>

<style scoped lang="scss">
@import "../styles/theme";

button {
  margin: 0;
  border-radius: 0;
  border: 1px solid rgba(0, 0, 0, 0.03);
  font-size: 1.1em;
  box-shadow: none;
}

button:focus {
  position: relative;
  z-index: 1000;
}

button:first-child {
  border-bottom-left-radius: 8px;
}

button:last-child {
  border-bottom-right-radius: 8px;
}

@if $theme == dark {
  button {
    border: 1px solid rgba(0, 0, 0, 0.05);
  }
}
</style>
