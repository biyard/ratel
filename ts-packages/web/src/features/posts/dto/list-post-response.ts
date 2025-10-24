import { BoosterType } from '@/features/spaces/types/booster-type';
import { SpaceType } from '@/features/spaces/types/space-type';
import { UserType } from '@/lib/api/models/user';
import { ListResponse } from '@/lib/api/ratel/common';

export type ListPostResponse = ListResponse<PostResponse>;

type PostResponse = {
  pk: string;

  created_at: number;
  updated_at: number;

  title: string;
  html_contents: string;

  shares: number;
  likes: number;
  comments: number;

  author_display_name: string;
  author_profile_url: string;
  author_username: string;
  author_pk: string;
  author_type: UserType;

  space_pk?: string;
  space_type?: SpaceType;
  booster?: BoosterType;
  // only for reward spaces
  rewards?: number;

  // Only for list posts Composed key
  urls: string[];
  liked: boolean;
};

export default PostResponse;
