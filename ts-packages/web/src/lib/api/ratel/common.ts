export type ListResponse<T> = {
  items: T[];
  bookmark: string | null;
};
