import { shallowReactive } from "vue";
import { invoke } from "@tauri-apps/api";

import { EntryKindField } from "entities/EntryKindField";

export const entryKindFieldStore = shallowReactive({
  async get_fields(id: number): Promise<EntryKindField[]> {
    return invoke("get_entry_kind_fields", {
      entryKindId: id,
    });
  },

  async update_fields(
    id: number,
    fields: EntryKindField[],
    deleted_fields: number[],
  ): Promise<EntryKindField[]> {
    return invoke("update_entry_kind_fields", {
      entryKindId: id,
      fields,
      deletedFields: deleted_fields,
    });
  },
});
