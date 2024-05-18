<script setup lang="ts">
import FilterSidebar from 'components/FilterSidebar.vue';
import Splitter from 'components/Splitter.vue';
import SplitterPanel from 'components/SplitterPanel.vue';
</script>

<template>
  <Splitter class="content">
    <SplitterPanel size="23%" min-size="200px">
      <FilterSidebar />
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
