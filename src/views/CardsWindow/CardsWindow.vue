<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { TauriEvent } from '@tauri-apps/api/event';

import { useTauriEvent } from 'utils/tauriEvent';

import CardSwitch from 'components/CardSwitch.vue';
import FilterSidebar, { FilterSidebarExposed } from 'components/FilterSidebar.vue';
import Splitter, { SplitterExposed } from 'components/Splitter.vue';
import SplitterPanel from 'components/SplitterPanel.vue';

const splitter = ref<SplitterExposed | null>(null);
const filterSidebar = ref<FilterSidebarExposed | null>(null);
useTauriEvent(TauriEvent.WINDOW_CLOSE_REQUESTED, reset);

onMounted(() => {
  reset();
});

function reset() {
  splitter.value?.reset();
  filterSidebar.value?.reset();
}
</script>

<template>
  <Splitter class="content" ref="splitter">
    <SplitterPanel size="20%" min-size="200px">
      <FilterSidebar ref="filterSidebar" />
    </SplitterPanel>
    <SplitterPanel class="data-view" min-size="400px">
      <CardSwitch class="data-view__switch" />
      <input
        class="data-view__search"
        type="text"
        placeholder="Поиск по картам/записям"
      />
    </SplitterPanel>
    <SplitterPanel size="30%">
      three
    </SplitterPanel>
  </Splitter>
</template>

<style scoped lang="scss">
@import "../../styles/theme";

.content {
  height: 100vh;
  justify-content: start;
}

.data-view {
  display: flex;
  gap: 8px;
}

.data-view__switch {
  flex-shrink: 0;

  @if $theme == dark {
    box-shadow: none;
  }
}

.data-view__search {
  width: 100%;
  padding: 0.3em 0.6em;

  @if $theme == dark {
    box-shadow: none;
  }
}
</style>
