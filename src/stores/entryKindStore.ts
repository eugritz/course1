import { invoke } from "@tauri-apps/api";
import { shallowReactive } from "vue";

import { EntryKind } from "entities/EntryKind";

export const entryKindStore = shallowReactive({
  cached_all: <EntryKind[]>[],
  cached_last: <EntryKind | null>null,

  async all(): Promise<EntryKind[]> {
    return invoke("get_all_entry_kinds").then((entry_kinds) => {
        this.cached_all = entry_kinds as EntryKind[];
        return entry_kinds as EntryKind[];
    });
  },

  async create(baseId: number, name: string): Promise<EntryKind> {
    return invoke("create_entry_kind", {
      entryKindId: baseId,
      entryKindName: name,
    });
  },

  async rename(id: number, new_name: string): Promise<EntryKind> {
    return invoke("rename_entry_kind", {
      entryKindId: id,
      newEntryKindName: new_name,
    });
  },

  async delete(id: number): Promise<EntryKind> {
    return invoke("delete_entry_kind", { deckId: id });
  },

  async last(): Promise<EntryKind | null> {
    return invoke("last_entry_kind").then((entry_kind) => {
        this.cached_last = entry_kind as EntryKind | null;
        return entry_kind as EntryKind | null;
    });
  },
});
