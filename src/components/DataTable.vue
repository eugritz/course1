<script lang="ts">
export interface DataTableExposed {
  next: () => void;
  prev: () => void;
  deselect: () => void;
}
</script>

<script setup lang="ts" generic="T">
import {
  ComponentPublicInstance,
  computed,
  onMounted,
  onUnmounted,
  ref,
  useSlots,
  watch,
} from 'vue';
import Column from './Column.vue';
import Resizer from './Resizer.vue';

const props = defineProps<{
  value: T[],
}>();

const slots = useSlots();
const columns = computed(() =>
  slots && slots.default ? slots.default().filter(x => x.type === Column) : []);

const table = ref<HTMLElement | null>(null);
const header = ref<HTMLElement | null>();
const resizers = ref<HTMLElement[]>([]);

const resizeCurrentColumn = ref<HTMLElement | null>(null);
const resizeCurrentColumnWidth = ref<number | null>(null);
const resizeNextColumn = ref<HTMLElement | null>(null);
const resizeNextColumnWidth = ref<number | null>(null);

const isItemDragging = ref(false);
const selectedItemIdx = ref<number | null>(null);
const pickedItem = defineModel<T | null>();

const sortColumnIdx = ref<number | null>(null);
const sortDirection = ref(false);

onMounted(() => {
  window.addEventListener("mouseup", handleItemDragStop);

  if (!header.value)
    return;

  for (let i = 0; i < header.value.children.length; i++) {
    const el = header.value.children[i] as HTMLElement;

    let textWidth = 0;
    let sortWidth = 0;

    const textEl = el.getElementsByTagName("span");
    if (textEl.length > 0) {
      textWidth = textEl[0].getBoundingClientRect().width;
    }

    const sortEl = el.getElementsByClassName("sort");
    if (sortEl.length > 0) {
      sortWidth = sortEl[0].getBoundingClientRect().width;
    }

    const minWidth = textWidth + sortWidth + 16;
    el.style.minWidth = minWidth + "px";
  }
});

onUnmounted(() => {
  window.removeEventListener("mouseup", handleItemDragStop);
});

watch([table, () => props.value], () => {
  if (!table.value)
    return;

  resizers.value.forEach(x => {
    x.style.height = (table.value?.offsetHeight ?? 0) + "px";
  });
});

function addResizer(ref: Element | ComponentPublicInstance | null) {
  const el = ref as HTMLElement;
  if (el.children.length > 1)
    resizers.value.push(
      el.getElementsByClassName("resizer--small")[0] as HTMLElement
    );
}

function handleResizeStart(event: MouseEvent) {
  const resizer = event.target as HTMLElement;

  const current = resizer.parentElement;
  if (!current)
    return;

  resizeCurrentColumn.value = current;
  resizeCurrentColumnWidth.value = current.offsetWidth;

  const next = current.nextElementSibling as HTMLElement;
  if (next) {
    resizeNextColumn.value = next;
    resizeNextColumnWidth.value = next.offsetWidth;
  }
}

function handleResize(event: MouseEvent, start: MouseEvent) {
  if (!resizeCurrentColumn.value || !resizeCurrentColumnWidth.value)
    return;

  const dx = event.clientX - start.clientX;

  const currentNewWidth = resizeCurrentColumnWidth.value + dx - 4; // wtf is that?
  if (currentNewWidth < parseInt(resizeCurrentColumn.value.style.minWidth))
    return;

  if (resizeNextColumn.value && resizeNextColumnWidth.value) {
    const nextNewWidth = resizeNextColumnWidth.value - dx;
    if (nextNewWidth < parseInt(resizeNextColumn.value.style.minWidth))
      return;
    resizeNextColumn.value.style.width = nextNewWidth + "px";
  }

  resizeCurrentColumn.value.style.width = currentNewWidth + "px";
}

function handleItemDragStart(item: number, event: MouseEvent) {
  if (event.button === 0)
    isItemDragging.value = true;
  selectedItemIdx.value = item;
}

function handleItemDragUpdate(item: number) {
  if (!isItemDragging.value)
    return;

  selectedItemIdx.value = item;
}

function handleItemDragStop() {
  if (selectedItemIdx.value === null)
    return;

  isItemDragging.value = false;
  pickedItem.value = props.value[selectedItemIdx.value];
}

function handleItemNext() {
  if (selectedItemIdx.value === null) {
    selectedItemIdx.value = 0;
  } else if (selectedItemIdx.value + 1 < props.value.length) {
    selectedItemIdx.value++;
  }
  pickedItem.value = props.value[selectedItemIdx.value];
}

function handleItemPrev() {
  if (!selectedItemIdx.value) {
    selectedItemIdx.value = 0;
  } else {
    selectedItemIdx.value--;
  }
  pickedItem.value = props.value[selectedItemIdx.value];
}

function handleColumnHeaderClick(idx: number) {
  if (idx !== sortColumnIdx.value)
    sortDirection.value = false;
  else
    sortDirection.value = !sortDirection.value;
  sortColumnIdx.value = idx;
}

function deselect() {
  selectedItemIdx.value = null;
  pickedItem.value = null;
}

defineExpose({
  next: handleItemNext,
  prev: handleItemPrev,
  deselect,
});
</script>

<template>
  <table ref="table">
    <thead>
      <tr ref="header">
        <th
          v-for="(column, idx) in columns"
          scope="col"
          :ref="addResizer"
          @click="handleColumnHeaderClick(idx)"
        >
          <div>
            <div>
              <span>
                {{column.props?.header ?? ""}}
              </span>
            </div>
            <div
              :class="{
                'sort': true,
                'sort--asc': idx === sortColumnIdx && sortDirection,
                'sort--desc': idx === sortColumnIdx && !sortDirection,
                'hidden': idx !== sortColumnIdx
              }"
            >
            </div>
          </div>
          <Resizer
            v-if="idx !== columns.length - 1"
            :icon="false"
            @resizestart="handleResizeStart"
            @resize="handleResize"
          />
        </th>
      </tr>
    </thead>
    <tbody>
      <tr
        v-for="(row, idx) in value"
        :class="{ 'selected': selectedItemIdx === idx }"
        @mousedown="handleItemDragStart(idx, $event)"
        @mouseenter="handleItemDragUpdate(idx)"
      >
        <td v-for="column in columns">
          <span>
            {{column.props?.field ? row[column.props.field as keyof T] : ""}}
          </span>
        </td>
      </tr>
    </tbody>
  </table>
</template>

<style scoped lang="scss">
@import "../styles/mixins";
@import "../styles/variables";

.hidden {
  opacity: 0;
}

.sort,
.sort--asc,
.sort--desc {
  width: 0;
  height: 0;
  margin-right: 5px;
  border-style: solid;
  border-width: 5px;
  border-color: rgba(0, 0, 0, 0);
  border-top-width: 0;
}

.sort--asc {
  border-top-width: 0;
  border-bottom-color: #fff;
}

.sort--desc {
  border-top-width: 5px;
  border-bottom-width: 0;
  border-top-color: #fff;
}

table {
  table-layout: fixed;
  border-radius: 4px;
  background-color: #272727;
  @include user-select-none;
}

thead {
  background-color: #454545;
}

tr {
  @include user-select-none;
}

th {
  position: relative;
  white-space: nowrap;
}

td {
  max-width: 0px;
  padding: 0px 4px;
  text-align: left;
  text-overflow: ellipsis;
  overflow: hidden;
  white-space: nowrap;
}

thead > tr:first-child th:first-child {
  border-top-left-radius: 4px;
}

thead > tr:first-child th:last-child {
  border-top-right-radius: 4px;
}

thead > tr > th > div {
  display: flex;
  flex-direction: row;
  align-items: center;

  div:nth-child(1) {
    flex: 1
  }
}

tbody > tr:nth-of-type(odd) {
  background-color: #272727;
}

tbody > tr:nth-of-type(even) {
  background-color: #2f2f2f;
}

.selected {
  background-color: $selection-background-color !important;
}
</style>
