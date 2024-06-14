export interface Entry {
  id: number;
  entry_kind_id: number;
  deck_id: number;
  color_tag: number;
  progress: number;
  created_at: Date;
  last_show_at: Date | null;
  next_shown_at: Date | null;
}

export interface FilteredEntry {
  id: number;
  sort_field: string;
  entry_kind_id: number;
  entry_kind_name: string;
  deck_id: number;
  deck_name: number;
  card_count: number;
  tags: string[];
  joined_tags: string;
  color_tag: number;
  progress: number;
  created_at: Date;
  last_show_at: Date | null;
  next_shown_at: Date | null;
}

export interface FilteredCard {
  id: number;
  sort_field: string;
  entry_kind_id: number;
  deck_id: number;
  deck_name: number;
  card_id: number;
  card_name: string;
  color_tag: number;
  progress: number;
  created_at: Date;
  last_show_at: Date | null;
  next_shown_at: Date | null;
}
