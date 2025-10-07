import Artwork from './artwork';
import Oracle from './oracle';

export interface Dagit {
  id: number;
  created_at: number;
  started_at: number | null;
  ended_at: number | null;

  title?: string;
  html_contents: string;
  onwer_id: number;
  artworks: Artwork[];

  oracles: Oracle[];
  is_oracle: boolean;
}
