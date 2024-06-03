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
  entry_kind_id: number;
  deck_id: number;
  deck_name: number;
  color_tag: number;
  progress: number;
  created_at: Date;
  last_show_at: Date | null;
  next_shown_at: Date | null;
  sort_field: string;
}
