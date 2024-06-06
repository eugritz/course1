import { shallowReactive } from "vue";
import { invoke } from "@tauri-apps/api";

import { EntryFieldValueExtra } from "entities/EntryFieldValue";

export const entryFieldValueStore = shallowReactive({
  async get_fields(id: number): Promise<EntryFieldValueExtra[]> {
    return invoke("get_entry_field_values", {
      entryId: id,
    });
  },

  async update(id: number, value: string):
    Promise<EntryFieldValueExtra[]> 
  {
    return invoke("update_entry_field_value", {
      entryFieldValueId: id,
      newValue: value,
    });
  },
});
