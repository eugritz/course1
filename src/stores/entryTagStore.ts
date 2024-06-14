import { invoke } from "@tauri-apps/api";
import { shallowReactive } from "vue";

export const entryTagStore = shallowReactive({
  async get_tags(id: number): Promise<string[]> {
    return invoke("get_entry_tags", {
      entryId: id,
    });
  },

  async set_tags(id: number, tags: string[]): Promise<string[]> {
    return invoke("set_entry_tags", {
      entryId: id,
      tags,
    });
  },
});
