export interface NewsSummary {
  id: number;
  created_at: number;
  updated_at: number;

  title: string;
  html_content: string;
  user_id: number;
}
