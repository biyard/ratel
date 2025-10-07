export interface QueryResponse<T> {
  total_count: number;
  items: T[];
}
