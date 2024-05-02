<template>
  <span class="badge">
    <slot />
  </span>
</template>

<style scoped lang="scss">
@import "../styles/theme";

.badge {
  width: fit-content;
  background-color: #cae0fc;
  padding: 2px 5px;
  margin: 0 10px;
  border-radius: 10px;
  font-size: 1em;
  text-align: center;
}

@if $theme == light {
  .badge {
    color: #194680;
  }
}
@if $theme == dark {
  .badge {
    background-color: #39434f;
  }
}
</style>
