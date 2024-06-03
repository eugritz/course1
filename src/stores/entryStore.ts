import { invoke } from "@tauri-apps/api";
import { shallowReactive } from "vue";

export const entryStore = shallowReactive({
  async filter(query: string): Promise<void> {
    invoke("filter_entries", {
      query,
    });
  },

  async create(
    entry_kind_id: number,
    deck_id: number,
    field_values: string[]
  ): Promise<void> {
    return invoke("create_entry", {
      entryKindId: entry_kind_id,
      deckId: deck_id,
      values: field_values,
      tags: [],
    });
  },
});
