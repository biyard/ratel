import { BoosterType } from '@/features/spaces/types/booster-type';
import { SpaceVisibility } from '@/features/spaces/types/space-common';
import { SpaceType } from '@/features/spaces/types/space-type';
import { UserType } from '@/lib/api/ratel/users.v3';

interface Post {
  pk: string;
  created_at: number;
  updated_at: number;
  title: string;
  html_contents: string;
  post_type: PostType;
  status: FeedStatus;
  visibility?: Visibility;
  shares: number;
  likes: number;
  comments: number;
  user_pk: string;
  author_display_name: string;
  author_profile_url: string;
  author_username: string;
  author_type: UserType;
  space_pk?: string;
  space_type?: SpaceType;
  space_visibility?: SpaceVisibility;

  booster?: BoosterType;
  rewards?: number;
  urls: string[];
}
export default Post;

export enum PostType {
  Post = 1,
  Repost = 2,
  Artwork = 3,
}

export enum Visibility {
  Public = 'PUBLIC',
  TeamOnly = 'TEAM_ONLY',
}

export enum FeedStatus {
  Draft = 1,
  Published = 2,
}
