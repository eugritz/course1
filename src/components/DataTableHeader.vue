<script lang="ts">
export interface DataTableHeaderExposed {
  tableRef: HTMLElement | null;
  columns: VNode[];
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

<script setup lang="ts">
import {
  ComponentPublicInstance,
  Fragment,
  VNode,
  VNodeNormalizedChildren,
  computed,
  getCurrentInstance,
  isVNode,
  nextTick,
  ref,
  useSlots,
  watch,
} from 'vue';

import Column from './Column.vue';
import { DataTableExposed } from './DataTable.vue';
import Resizer from './Resizer.vue';

const props = defineProps<{
  resizerHeight?: number,
  dataTableRef: DataTableExposed | null,
}>();

const slots = useSlots();

const hTableRef = ref<HTMLElement | null>(null);
const hTableRefReadOnly = computed(() => hTableRef.value);
const tableRef = computed(() => props.dataTableRef?.tableRef);
const headerRef = ref<HTMLElement | null>(null);

const columns = computed(() => {
  if (!slots || !slots.default)
    return [];
  return getColumns(slots.default());
});

const resizerRefs = ref<HTMLElement[]>([]);
const oldResizersCount = ref(0);

const hResizeCurrentColumn = ref<HTMLElement | null>(null);
const resizeCurrentColumn = ref<HTMLElement | null>(null);
const hResizeCurrentColumnWidth = ref<number | null>(null);
const resizeCurrentColumnWidth = ref<number | null>(null);

const sortColumnIdx = defineModel<number | null>("orderby");
const sortDirection = defineModel<boolean>("order"); // true is ASC, false is DESC

const instance = getCurrentInstance();

watch([tableRef, columns], () => {
  instance?.proxy?.$forceUpdate();
}, {
  once: true,
});

watch([tableRef, columns], () => {
  if (!tableRef.value)
    return;

  resizerRefs.value.splice(0, oldResizersCount.value);
  resizerRefs.value.forEach(x => {
    x.style.height = (tableRef.value?.offsetHeight ?? 0) + "px";
  });
  oldResizersCount.value = resizerRefs.value.length;

  nextTick(() => {
    reset();
  });
});

function reset() {
  if (!headerRef.value)
    return;

  sortColumnIdx.value = null;
  sortDirection.value = false;

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
    if (props.resizerHeight)
      resizer.style.height = props.resizerHeight + "px";
    else
      resizer.style.height = (tableRef.value?.offsetHeight ?? 0) + "px";
    resizerRefs.value.push(resizer);
  }
}

function handleResizeStart(event: MouseEvent, columnIdx: number) {
  const resizer = event.target as HTMLElement;

  const hCurrent = resizer.parentElement;
  const current = props.dataTableRef?.resizerRefs[columnIdx].parentElement;
  if (!current || !hCurrent)
    return;

  hResizeCurrentColumn.value = hCurrent;
  resizeCurrentColumn.value = current;
  hResizeCurrentColumnWidth.value = hCurrent.offsetWidth;
  resizeCurrentColumnWidth.value = current.offsetWidth;
}

function handleResize(event: MouseEvent, start: MouseEvent) {
  if (!resizeCurrentColumn.value || !resizeCurrentColumnWidth.value
    || !hResizeCurrentColumn.value || !hResizeCurrentColumnWidth.value)
    return;

  const dx = event.clientX - start.clientX;

  const currentNewWidth = resizeCurrentColumnWidth.value + dx - 4; // wtf is that?
  if (currentNewWidth < parseInt(resizeCurrentColumn.value.style.width))
    return;

  hResizeCurrentColumn.value.style.minWidth = currentNewWidth + "px";
  resizeCurrentColumn.value.style.minWidth = currentNewWidth + "px";
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

defineExpose({
  tableRef: hTableRefReadOnly,
  columns,
});
</script>

<template>
  <table tabindex="-1" ref="hTableRef">
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
            @resizestart="handleResizeStart($event, idx)"
            @resize="handleResize"
          />
        </th>
      </tr>
    </thead>
    <tbody>
      <tr>
        <td v-for="_column in columns">
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

tbody {
  opacity: 0;
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
  padding: 0px 4px;
  text-align: left;
  text-overflow: ellipsis;
  overflow: hidden;
  white-space: nowrap;
}

td:last-child {
  width: 100%;
}
</style>
