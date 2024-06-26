<script lang="ts">
export interface DataTableExposed {
  tableRef: HTMLElement | null,
  headerRef: HTMLElement | null,
  resizerRefs: HTMLElement[],
  selectNext: () => void;
  selectPrev: () => void;
  deselect: () => void;
  reset: () => void;
}

type RawSlot = {
  [name: string]: unknown;
  $stable?: boolean;
};

function hasSlot(children: VNodeNormalizedChildren, name: string):
  children is RawSlot
{
  return children !== null
    && !Array.isArray(children)
    && typeof children !== "string"
    && children[name] !== undefined
    && typeof children[name] === "function";
}

function getColumns(children: VNodeNormalizedChildren) {
  let columns = <VNode[]>[];
  if (!Array.isArray(children))
    return [];

  for (let i = 0; i < children.length; i++) {
    const vnode = children[i];
    if (!isVNode(vnode))
      continue;

    if (vnode.type === Column) {
      columns.push(vnode);
    } else if (vnode.type === Fragment)
      columns = columns.concat(getColumns(vnode.children));
  }

  return columns;
}

function getColumnMinWidth(el: HTMLElement) {
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

  return textWidth + sortWidth + 16;
}
</script>

<script setup lang="ts" generic="T">
import {
  ComponentPublicInstance,
  Fragment,
  VNode,
  VNodeNormalizedChildren,
  computed,
  isVNode,
  nextTick,
  onMounted,
  onUnmounted,
  ref,
  useSlots,
  watch,
} from 'vue';

import Column from './Column.vue';
import Resizer from './Resizer.vue';
import {DataTableHeaderExposed} from './DataTableHeader.vue';

const props = defineProps<{
  value: T[],
  dataTableHeaderRef?: DataTableHeaderExposed | null,
}>();

const emit = defineEmits<{
  (e: "item:contextmenu", event: MouseEvent, item: T): void,
}>();

const slots = useSlots();

const columns = computed(() => {
  if (!props.dataTableHeaderRef) {
    if (!slots || !slots.default)
      return [];
    return getColumns(slots.default());
  } else {
    return props.dataTableHeaderRef.columns;
  }
});

const tableRef = ref<HTMLElement | null>(null);
const tableRefReadOnly = computed(() => tableRef.value);
const headerRef = ref<HTMLElement | null>();
const headerRefReadOnly = computed(() => headerRef.value);
const resizerRefs = ref<HTMLElement[]>([]);
const resizerRefsReadOnly = computed(() => resizerRefs.value);
const oldResizersCount = ref(0);

const resizeCurrentColumn = ref<HTMLElement | null>(null);
const resizeCurrentColumnWidth = ref<number | null>(null);

const isItemDragging = ref(false);
const selectedItemIdx = ref<number | null>(null);
const pickedItem = defineModel<T | null>();

const sortColumnIdx = defineModel<number | null>("orderby");
const sortDirection = defineModel<boolean>("order"); // true is ASC, false is DESC

onMounted(() => {
  window.addEventListener("mouseup", handleItemDragStop);
  reset();
});

onUnmounted(() => {
  window.removeEventListener("mouseup", handleItemDragStop);
});

watch(columns, () => {
  nextTick(() => {
    reset();
  });
});

watch([tableRef, columns, () => props.value], () => {
  if (!tableRef.value)
    return;

  resizerRefs.value.splice(0, oldResizersCount.value);
  resizerRefs.value.forEach(x => {
    x.style.height = (tableRef.value?.offsetHeight ?? 0) + "px";
  });
  oldResizersCount.value = resizerRefs.value.length;
});

function reset() {
  if (!headerRef.value)
    return;

  sortColumnIdx.value = null;
  sortDirection.value = false;
  deselect();

  for (let i = 0; i < headerRef.value.children.length; i++) {
    const el = headerRef.value.children[i] as HTMLElement;
    const minWidth = getColumnMinWidth(el);
    el.style.width = minWidth + "px";
    el.style.minWidth = minWidth + "px";
  }
}

function addResizer(ref: Element | ComponentPublicInstance | null) {
  const el = ref as HTMLElement;
  if (!el)
    return;

  el.style.removeProperty("minWidth");
  el.style.width = getColumnMinWidth(el) + "px";

  if (el.children.length > 1) {
    const resizer = el.getElementsByClassName("resizer--small")[0] as HTMLElement;
    if (props.dataTableHeaderRef)
      resizer.style.height = "0px";
    else
      resizer.style.height = (tableRef.value?.offsetHeight ?? 0) + "px";
    resizerRefs.value.push(resizer);
  }
}

function handleResizeStart(event: MouseEvent) {
  const resizer = event.target as HTMLElement;

  const current = resizer.parentElement;
  if (!current)
    return;

  resizeCurrentColumn.value = current;
  resizeCurrentColumnWidth.value = current.offsetWidth;
}

function handleResize(event: MouseEvent, start: MouseEvent) {
  if (!resizeCurrentColumn.value || !resizeCurrentColumnWidth.value)
    return;

  const dx = event.clientX - start.clientX;

  const currentNewWidth = resizeCurrentColumnWidth.value + dx - 4; // wtf is that?
  if (currentNewWidth < parseInt(resizeCurrentColumn.value.style.width))
    return;

  resizeCurrentColumn.value.style.minWidth = currentNewWidth + "px";
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

function handleColumnHeaderClick(idx: number) {
  if (idx !== sortColumnIdx.value) {
    sortDirection.value = false;
  } else {
    if (sortDirection.value) {
      sortDirection.value = false;
      sortColumnIdx.value = null;
      return;
    }

    sortDirection.value = !sortDirection.value;
  }

  sortColumnIdx.value = idx;
}

function selectNext() {
  if (selectedItemIdx.value === null) {
    selectedItemIdx.value = 0;
  } else if (selectedItemIdx.value + 1 < props.value.length) {
    selectedItemIdx.value++;
  }
  pickedItem.value = props.value[selectedItemIdx.value];
}

function selectPrev() {
  if (!selectedItemIdx.value) {
    selectedItemIdx.value = 0;
  } else {
    selectedItemIdx.value--;
  }
  pickedItem.value = props.value[selectedItemIdx.value];
}

function deselect() {
  selectedItemIdx.value = null;
  pickedItem.value = null;
}

defineExpose({
  tableRef: tableRefReadOnly,
  headerRef: headerRefReadOnly,
  resizerRefs: resizerRefsReadOnly,
  selectNext,
  selectPrev,
  deselect,
  reset,
});
</script>

<template>
  <table ref="tableRef">
    <thead>
      <tr ref="headerRef">
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
        @contextmenu="$emit('item:contextmenu', $event, row)"
      >
        <td v-for="column in columns">
          <div v-if="hasSlot(column.children, 'body')">
            <component :is="() => ((column.children as RawSlot)['body'] as Function)(row)" />
          </div>
          <span v-else>
            {{column.props?.field
              ? row[column.props.field as keyof T]
              : column.props?.value ?? ""}}
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
  border-spacing: 2px 0;
  background-color: #272727;
  @include user-select-none;
}

thead {
  background-color: #454545;
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

tr {
  @include user-select-none;
}

th {
  position: relative;
  padding: 1px 0;
  border-bottom: 2px solid #272727;
  box-sizing: border-box;
  font-weight: 300;
  white-space: nowrap;
}

th > div:first-child {
  text-overflow: ellipsis;
  overflow: hidden;
}

td {
  max-width: 0px;
  padding: 1px 4px;
  text-align: left;
  text-overflow: ellipsis;
  overflow: hidden;
  white-space: nowrap;
}

td:last-child {
  width: 100%;
}

.selected {
  background-color: $selection-background-color !important;
}
</style>
