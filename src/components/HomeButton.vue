<template>
  <button><slot /></button>
</template>

<style scoped lang="scss">
@import "../styles/theme";

button {
  border-radius: 0;
  border: 1px solid rgba(0, 0, 0, 0.03);
  padding: 0.6em 1.2em;
  font-size: 1.1em;
  font-weight: 500;
  font-family: inherit;
  box-shadow: none;
  transition: border-color 0.25s;
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
