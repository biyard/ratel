import { Author } from './spaces';

export interface SpaceComment {
  id: number;
  created_at: number;
  updated_at: number;
  html_contents: string;
  user_id: number;
  author: Author[];
}
