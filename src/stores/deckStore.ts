import { invoke } from "@tauri-apps/api";
import { shallowReactive } from "vue";

import { Deck } from "entities/Deck";

export const deckStore = shallowReactive({
  cached_all: <Deck[]>[],
  cached_last: <Deck | null>null,

  async all(): Promise<Deck[]> {
    return invoke("get_all_decks").then((decks) => {
        this.cached_all = decks as Deck[];
        return decks as Deck[];
    });
  },

  async create(title: string): Promise<Deck> {
    return invoke("create_deck", {
      deckTitle: title,
    });
  },

  async rename(id: number, new_title: string): Promise<Deck> {
    return invoke("rename_deck", {
      deckId: id,
      newDeckTitle: new_title,
    });
  },

  async delete(id: number): Promise<Deck> {
    return invoke("delete_deck", { deckId: id });
  },

  async last(): Promise<Deck | null> {
    return invoke("last_deck").then((deck) => {
        this.cached_last = deck as Deck | null;
        return deck as Deck | null;
    });
  },
});
