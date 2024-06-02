import { invoke } from "@tauri-apps/api";
import { shallowReactive } from "vue";

export const entryStore = shallowReactive({
  async filter(query: string): Promise<void> {
    invoke("filter_entries", {
      query,
    });
  },
});
