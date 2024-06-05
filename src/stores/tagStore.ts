import { invoke } from "@tauri-apps/api";
import { shallowReactive } from "vue";

import { Tag } from "entities/Tag";

export const tagStore = shallowReactive({
  cached_all: <Tag[]>[],

  async all(): Promise<Tag[]> {
    return invoke("get_all_tags").then((tags) => {
      this.cached_all = tags as Tag[];
      return tags as Tag[];
    });
  },

  async rename(old_name: string, new_name: string): Promise<void> {
    return invoke("rename_tag", {
      tag: old_name,
      tagNewName: new_name,
    });
  },

  async delete(name: string): Promise<void> {
    return invoke("delete_tag", {
      tag: name,
    });
  },
});
