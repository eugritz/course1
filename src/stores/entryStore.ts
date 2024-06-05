import { invoke } from "@tauri-apps/api";
import { shallowReactive } from "vue";
import { Entry, FilteredEntry } from "entities/Entry";

export const entryStore = shallowReactive({
  async filter(query?: string): Promise<FilteredEntry[]> {
    return invoke("filter_entries", {
      query: query === undefined ? "" : query,
    });
  },

  async create(
    entry_kind_id: number,
    deck_id: number,
    field_values: string[],
    tags: string[],
  ): Promise<Entry> {
    return invoke("create_entry", {
      entryKindId: entry_kind_id,
      deckId: deck_id,
      values: field_values,
      tags,
    });
  },
});
