import { User } from './user';
import { Promotion } from './promotion';
import type { Follower } from './network';

// Interface for FeedSummary from the backend
export interface FeedSummary {
  id: number;
  created_at: number;
  updated_at: number;

  title?: string | null;
  html_contents: string;

  user_id: number;
  industry_id: number;

  parent_id?: number | null;
  quote_feed_id?: number | null;

  likes: number;
  is_liked: boolean;
  comments: number;
  rewards: number;
  shares: number;
  is_bookmarked: boolean;

  feed_type: number;
  status: number;

  // Additional fields that might be present
  url?: string | null;
  proposer_name?: string | null;
  profile_image?: string | null;
}

// Interface for NewsSummary from the backend
export interface NewsSummary {
  id: number;
  created_at: number;
  updated_at: number;

  title: string;
  html_content: string;
  user_id: number;
}

// Main response interface matching the Rust HomeGatewayResponse
export interface HomeGatewayResponse {
  user_info: User | null;
  feeds: FeedSummary[];
  promotions: Promotion | null;
  news: NewsSummary[];
  suggested_users: Follower[];
}
