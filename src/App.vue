<script setup lang="ts">
import { computed, ref, watchEffect } from "vue";
import { Event } from "@tauri-apps/api/event";

import { deckStore } from "stores/deckStore";
import { useTauriEvent } from "utils/tauriEvent";

import Header from "components/Header.vue";
import DeckList from "components/DeckList.vue";
import DeckListItem from 'components/DeckListItem.vue';
import Popup, { PopupExposed } from "components/Popup.vue";

const decks = computed(() => deckStore.cached);
const optionsPopup = ref<PopupExposed | null>(null);

function reset() {
  deckStore.all();
}

watchEffect(() => {
  reset();
});

function handleDialogResult(event: Event<unknown>) {
  if (event.windowLabel !== "NewDeckDialog") {
    return;
  }

  reset();
}

function handleOptionsToggle(event: MouseEvent) {
  optionsPopup.value?.toggle(event);
}

useTauriEvent("dialog_result", handleDialogResult);
</script>

<template>
  <Header />
  <div class="content--center">
    <div class="content--focus">
      <DeckList class="full-width">
        <DeckListItem
          v-for="item in decks"
          :key="item.id"
          @options:toggle="handleOptionsToggle"
        >
          {{ item.name }}
        </DeckListItem>
      </DeckList>
      <Popup ref="optionsPopup">
        <div>
          Hello!
        </div>
      </Popup>
    </div>
  </div>
</template>

<style scoped>
.full-width {
  width: 100%;
}
</style>
