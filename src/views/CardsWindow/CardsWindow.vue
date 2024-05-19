<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { TauriEvent } from '@tauri-apps/api/event';

import { useTauriEvent } from 'utils/tauriEvent';
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
    <SplitterPanel size="23%" min-size="200px">
  <Splitter class="content" ref="splitter">
      <FilterSidebar ref="filterSidebar" />
    </SplitterPanel>
    <SplitterPanel>
      two
    </SplitterPanel>
    <SplitterPanel size="20%">
      three
    </SplitterPanel>
  </Splitter>
</template>

<style scoped lang="scss">
.content {
  height: 100vh;
  justify-content: start;
}
</style>
