<script setup lang="ts">
import Header from "components/Header.vue";
import DeckList from "components/DeckList.vue";
import DeckListItem from 'components/DeckListItem.vue';
</script>

<template>
  <Header />
  <div class="content--center">
    <div class="content--focus">
      <DeckList class="full-width">
      </DeckList>
    </div>
  </div>
</template>

<style scoped>
.full-width {
  width: 100%;
}
</style>
