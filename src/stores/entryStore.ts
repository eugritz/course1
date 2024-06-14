import { invoke } from "@tauri-apps/api";
import { shallowReactive } from "vue";
import { Entry, FilteredCard, FilteredEntry } from "entities/Entry";

export const entryStore = shallowReactive({
  loading: false,

  async filter(switch_: "entries" | "cards", query?: string):
    Promise<(FilteredEntry | FilteredCard)[]>
  {
    if (switch_ === "cards") {
      return invoke("filter_cards", {
        query: query === undefined ? "" : query,
      });
    } else {
      return invoke("filter_entries", {
        query: query === undefined ? "" : query,
      });
    }
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

  async delete(
    id: number,
  ): Promise<void> {
    return invoke("delete_entry", {
      entryId: id,
    });
  },
});
