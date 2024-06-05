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
});
