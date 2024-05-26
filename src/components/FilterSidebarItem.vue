<script lang="ts">
export type ItemIcons = "decks"
  | "today"
  | "decks"
  | "deck"
  | "currentDeck"
  | "flag"
  | "flagUnspecified"
  | "flagRed"
  | "flagOrange"
  | "flagGreen"
  | "flagBlue"
  | "flagTorquoise"
  | "flagPink"
  | "flagPurple"
  | "cardState"
  | "cardStateNew"
  | "cardStateLearning"
  | "cardStateSuspended"
  | "cardKind"
  | "tag"
  | "tagUnspecified";
</script>

<script setup lang="ts">
import { computed } from 'vue';

const props = defineProps<{
  level?: number,
  icon?: ItemIcons,
  value: string,
}>();

const mapIconIdToName: Record<ItemIcons, string> = {
  "today": "clock",
  "decks": "book-alt",
  "deck": "book",
  "currentDeck": "book-open",
  "flag": "custom-flag",
  "flagUnspecified": "custom-flag-unspecified",
  "flagRed": "custom-flag-red",
  "flagOrange": "custom-flag-orange",
  "flagGreen": "custom-flag-green",
  "flagBlue": "custom-flag-blue",
  "flagTorquoise": "custom-flag-torquoise",
  "flagPink": "custom-flag-pink",
  "flagPurple": "custom-flag-purple",
  "cardState": "circle",
  "cardStateNew": "custom-card-state-new",
  "cardStateLearning": "custom-card-state-learning",
  "cardStateSuspended": "custom-card-state-suspended",
  "cardKind": "table",
  "tag": "tag-alt",
  "tagUnspecified": "custom-tag-unspecified",
};

const level = computed(() =>
  `filter-sidebar__list__item${((props.level ?? 0) > 1 ? `-lvl-${props.level}` : "")}`
);
</script>

<template>
  <li :class="level">
    <unicon
      v-if="icon"
      class="icon"
      width="18px"
      height="18px"
      :name="mapIconIdToName[icon]"
    />
    {{$props.value}}
  </li>
</template>

<style scoped lang="scss">
@import "../styles/mixins";

.filter-sidebar__list__item,
.filter-sidebar__list__item-lvl-1,
.filter-sidebar__list__item-lvl-2 {
  margin: 0;
  padding: 1px 1em;
  padding-right: 0;
  display: flex;
  align-items: center;
  gap: 0.5em;
  cursor: default;
  @include user-select-none;

  .unicon {
    height: 20px;
  }
}

.filter-sidebar__list__item-lvl-2 {
  padding-left: 3em;
}
</style>
