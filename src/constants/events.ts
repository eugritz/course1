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
  NewDeckModal: {
    open: "open_new_deck_modal",
    onResult: "NewDeckModal:onResult",
  },
  RenameDeckModal: {
    open: "open_rename_deck_modal",
    setData: "RenameDeckModal:setData",
    onResult: "RenameDeckModal:onResult",
  },
  DeckFilterModal: {
    open: "open_deck_filter_modal",
    setData: "DeckFilterModal:setData",
    onResult: "DeckFilterModal:onResult",
  },
  // Windows
  CardsWindow: {
    open: "open_cards_window",
  },
  AddWindow: {
    open: "open_add_window",
  },
};
