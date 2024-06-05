import { invoke } from "@tauri-apps/api";
import { shallowReactive } from "vue";

export const entryTagStore = shallowReactive({
  async get_tags(id: number): Promise<string[]> {
    return invoke("get_entry_tags", {
      entryId: id,
    });
  },
});
