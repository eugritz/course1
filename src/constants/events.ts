export default {
  // Windows API
  window_open: "window_open",
  window_close: "window_close",
  // Modals
  ConfirmationModal: {
    open: "open_confirmation_modal",
    setData: "ConfirmationModal:setData",
    onResult: "ConfirmationModal:onResult",
    onReady: "ConfirmationModal:onReady",
  },
  DeckNewModal: {
    open: "open_deck_new_modal",
    onResult: "DeckNewModal:onResult",
  },
  DeckRenameModal: {
    open: "open_deck_rename_modal",
    setData: "DeckRenameModal:setData",
    onResult: "DeckRenameModal:onResult",
  },
  DeckFilterModal: {
    open: "open_deck_filter_modal",
    setData: "DeckFilterModal:setData",
    onResult: "DeckFilterModal:onResult",
  },
  EntryKindFilterModal: {
    open: "open_entry_kind_filter_modal",
    setData: "EntryKindFilterModal:setData",
    onResult: "EntryKindFilterModal:onResult",
  },
  // Windows
  CardsWindow: {
    open: "open_cards_window",
  },
  AddWindow: {
    open: "open_add_window",
  },
};
