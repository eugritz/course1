export interface EntryFieldValue {
  id: number;
  entry_id: number;
  entry_field_id: number;
  value: string;
}

export interface EntryFieldValueExtra {
  id: number;
  entry_id: number;
  entry_kind_id: number;
  entry_field_id: number;
  value: string;
  order: number;
  name: string;
  desc: string;
  type: string;
  immutable: boolean;
  is_default: boolean;
}
