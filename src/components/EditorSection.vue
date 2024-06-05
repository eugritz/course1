<script lang="ts">
export interface EditorSectionExposed {
  inputRef: HTMLInputElement | null;
  getTags(): string[] | null;
  setTags(tags: string[]): void;
  reset(): void;
}
</script>

<script setup lang="ts">
import { computed, nextTick, ref, watch } from 'vue';

const props = defineProps<{
  title: string,
  placeholder?: string,
  type?: "text" | "tags",
}>();

const isSectionOpen = ref(true);
const inputRef = ref<HTMLInputElement | null>(null);
const inputRefReadOnly = computed<HTMLInputElement | null>(() => inputRef.value);
const tagsRef = ref<HTMLElement | null>(null);
const inputTabindex = ref<number>(0);
const inputFocused = ref<boolean>(false);

const value = ref("");
const tags = ref<string[]>([]);

watch(tags, () => {
  if (!inputRef.value || !tagsRef.value)
    return;
  inputRef.value.style.left = tagsRef.value.offsetWidth + "px";
});

function reset() {
  inputTabindex.value = 0;
  inputFocused.value = false;
  value.value = "";
  tags.value = [];

  if (!inputRef.value)
    return;

  if (props.type === "text") {
    inputRef.value.value = "";
  } else if (props.type === "tags") {
    inputRef.value.innerHTML = "";
  }
}

function removeTag(idx: number) {
  tags.value.splice(idx, 1);
}

function handleClick() {
  if (inputFocused.value && inputRef.value && value.value.length > 0) {
    const selection = window.getSelection();
    const range = document.createRange();
    range.setStart(inputRef.value as Node, 1);
    selection?.removeAllRanges();
    selection?.addRange(range);
  }
}

function handleInput(event: Event) {
  const target = event.target as HTMLElement;
  const input = target.textContent!;
  const whitespace = input.search(/\s/);

  if (input.startsWith("_")) {
    target.textContent = "";
    return;
  }

  if (whitespace == -1) {
    value.value = input;
    return;
  }

  const left = input
    .slice(0, whitespace)
    .replace(/_+$/, "")
    .replace(/_{2,}/, "_");
  if (left === "") {
    target.textContent = "";
    return;
  }

  const right = input.slice(whitespace + 1);
  target.textContent = right;
  value.value = right;
  tags.value.push(left.toLowerCase());
}

function handleKeyBackspace(event: Event) {
  const target = event.target as HTMLElement;
  const input = target.textContent ?? "";
  if (input !== "" || tags.value.length === 0)
    return;

  const prev = tags.value.pop() as string;
  target.innerHTML = prev + "&nbsp";

  nextTick(() => {
    const selection = window.getSelection();
    const range = document.createRange();
    range.setStart(inputRef.value as Node, 1);
    selection?.removeAllRanges();
    selection?.addRange(range);
  });
}

function handleFocus() {
  inputTabindex.value = -1;
  inputFocused.value = true;

  const selection = window.getSelection();
  const range = document.createRange();
  range.selectNodeContents(inputRef.value as Node);
  selection?.removeAllRanges();
  selection?.addRange(range);
}

function handleBlur() {
  inputTabindex.value = 0;
  inputFocused.value = false;
}

function getTags() {
  if (props.type !== 'tags')
    return null;
  return tags.value;
}

function setTags(tags_: string[]) {
  if (props.type !== 'tags' || !inputRef.value)
    return;
  inputRef.value.innerHTML = "";
  value.value = "";
  tags.value = tags_;
}

defineExpose({
  inputRef: inputRefReadOnly,
  getTags,
  setTags,
  reset,
});
</script>

<template>
  <div class="section" :class="{ 'section--closed': !isSectionOpen }">
    <div class="section__header" @click="isSectionOpen = !isSectionOpen">
      <div class="section__header__title">
        <span>
          <unicon
            class="section__header__title__icon"
            name="angle-down"
            fill="white"
            width="20"
            height="20"
          />
        </span>
        <span>{{title}}</span>
      </div>
      <div v-if="type !== 'tags'" class="section__header__controls">
        <span>
          <unicon name="arrow" fill="white" width="20" height="20" />
        </span>
        <span>
          <unicon name="custom-pin" fill="white" width="18" height="18" />
        </span>
      </div>
    </div>
    <div v-show="isSectionOpen" class="section__content">
      <input
        v-if="type === 'text'"
        type="text"
        ref="inputRef"
        :placeholder="placeholder"
      />
      <div
        v-else-if="type === 'tags'"
        class="tags-input"
        :class="{ 'tags-input--focus': inputFocused }"
        :tabindex="inputTabindex"
        @focus="handleFocus"
      >
        <div class="tags-input__start-adornment">
          <unicon class="icon" name="tag-alt" width="21" height="21" />
        </div>
        <div class="tags-input__wrapper" @click="handleClick">
          <div class="tags-input__wrapper2" @click="handleClick">
            <div v-show="tags.length > 0" ref="tagsRef" class="tags-input__tags">
              <span
                v-for="(tag, idx) in tags"
                class="tags-input__tags__tag"
                @click="removeTag(idx)"
              >
                {{tag}}
                <unicon class="icon" name="times" width="16" height="16" />
              </span>
            </div>
            <span
              contenteditable
              class="tags-input__input"
              type="text"
              ref="inputRef"
              :value="value"
              @click.stop
              @input="handleInput"
              @focus="handleFocus"
              @blur="handleBlur"
              @keydown.backspace="handleKeyBackspace"
              :placeholder="placeholder"
            >
            </span>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped lang="scss">
@import "../styles/mixins";
@import "../styles/variables";

.section {
  text-align: left;
  @include user-select-none;
}

.section__header {
  padding: 2px 5px;
  display: flex;
  justify-content: space-between;
  cursor: pointer;
}

.section__header__title,
.section__header__controls {
  display: flex;
  gap: 8px;
}

.section__header__controls {
  align-items: center;
}

$tags-input-h-padding: 12px;
.tags-input {
  border-radius: 8px;
  border: 1px solid transparent;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
  padding: 0.6em 0;
  padding-right: 1.2em;
  display: flex;
  align-items: center;
  background-color: #1b1b1b;
}

.tags-input__wrapper {
  position: relative;
  width: 100%;
}

.tags-input__wrapper2 {
  position: absolute;
  width: 100%;
  display: flex;
  overflow: hidden;
  transform: translateY(-50%);
}

.tags-input__input {
  min-width: 2em;
  padding-left: 0;
  padding-right: 10px;
  outline: none;
  box-shadow: none;
  white-space: nowrap;

  * {
    display: none;
  }
}

.tags-input__start-adornment {
  display: flex;
  align-items: center;
  padding: 0 $tags-input-h-padding;

  .unicon {
    width: 21px;
    height: 21px;
  }
}

.tags-input--focus {
  outline: 2px solid $selection-ring-color;
  cursor: text;
}

.tags-input__tags {
  display: flex;
  justify-content: flex-end;
  gap: $tags-input-h-padding;
  margin-right: 4px;
  padding: 0 4px;
  overflow: hidden;
}

.tags-input__tags__tag {
  z-index: 1;
  position: relative;
  display: flex;
  align-items: center;
  cursor: pointer;

  .unicon {
    width: 16px;
    height: 16px;
  }
}

.tags-input__tags__tag::before {
  z-index: -1;
  position: absolute;
  content: "";
  width: 100%;
  height: 100%;
  padding: 0 4px;
  margin-left: -4px;
  border-radius: 4px;
  background-color: #39434f;
}
</style>

<style lang="scss">
.section__content {
  > input[type="text"] {
    width: 100%;
  }
}

.section--closed {
  .section__header__title__icon {
    transform: rotate(-90deg);
  }
}
</style>
