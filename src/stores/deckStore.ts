import { invoke } from "@tauri-apps/api";
import { shallowReactive } from "vue";

import { Deck } from "entities/Deck";

export const deckStore = shallowReactive({
  cached: <Deck[]>[],

  async all(): Promise<Deck[]> {
    return invoke("get_all_decks").then((decks) => {
        this.cached = decks as Deck[];
        return decks as Deck[];
    });
  },

  async create(title: string): Promise<Deck> {
    return invoke("create_deck", {
      deckTitle: title,
    });
  },
});
