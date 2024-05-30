import { shallowReactive } from "vue";
import { invoke } from "@tauri-apps/api";

import { EntryKindField } from "entities/EntryKindField";

export const entryKindFieldStore = shallowReactive({
  async fields(id: number): Promise<EntryKindField[]> {
    return invoke("get_entry_kind_fields", {
      entryKindId: id,
    });
  },
});
