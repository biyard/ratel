export interface Promotion {
  id: number;
  created_at: number;
  updated_at: number;

  name: string;
  description: string;
  image_url: string;

  feed_id: number;
}
