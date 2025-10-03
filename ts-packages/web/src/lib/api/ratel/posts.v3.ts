import { BoosterType } from '../models/feeds';
import { SpaceType } from '../models/spaces';
import { call } from './call';
import { ListResponse } from './common';

export async function listPosts(bookmark?: string): Promise<ListPostResponse> {
  let path = '/v3/posts';
  if (bookmark) {
    path += `?bookmark=${encodeURIComponent(bookmark)}`;
  }

  return call('GET', path);
}

export type ListPostResponse = ListResponse<PostResponse>;

export type PostResponse = {
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

  space_pk?: string;
  space_type?: SpaceType;
  booster?: BoosterType;
  // only for reward spaces
  rewards?: number;

  // Only for list posts Composed key
  urls: string[];
  liked: boolean;
};
