import { shallowReactive } from "vue";
import { invoke } from "@tauri-apps/api";

import { EntryFieldValueExtra } from "entities/EntryFieldValue";

export const entryFieldValueStore = shallowReactive({
  async get_fields(id: number): Promise<EntryFieldValueExtra[]> {
    return invoke("get_entry_field_values", {
      entryId: id,
    });
  },
});
