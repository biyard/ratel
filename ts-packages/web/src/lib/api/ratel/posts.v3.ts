import { BoosterType } from '@/types/booster-type';
import { SpaceType } from '../models/spaces';
import { UserType } from '../models/user';
import { call } from './call';
import { ListResponse } from './common';

export function createPost(team_pk?: string): Promise<CreatePostResponse> {
  if (team_pk) {
    return call('POST', '/v3/posts', { team_pk });
  }
  return call('POST', '/v3/posts');
}

export function getPost(postPk: string): Promise<PostDetailResponse> {
  return call('GET', `/v3/posts/${encodeURIComponent(postPk)}`);
}

export function deletePost(postPk: string): Promise<void> {
  return call('DELETE', `/v3/posts/${encodeURIComponent(postPk)}`);
}

export type PostComment = {
  pk: string;
  sk: string;
  updated_at: number;
  content: string;
  author_pk: string;
  author_display_name: string;
  author_profile_url: string;
  author_username: string;

  // FIXME: add
  likes: number;
  replies: number;

  parent_comment_pk: string;

  liked: boolean;
};

export type PostDetailResponse = {
  post: Post;
  is_liked: boolean;
  comments: PostComment[];
  // FIXME: Define the type properly
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  artwork_metadata?: any;
};

export function updatePostWithTitleAndContents(
  postPk: string,
  title: string,
  content: string,
): Promise<Post> {
  return call('PATCH', `/v3/posts/${encodeURIComponent(postPk)}`, {
    title,
    content,
  });
}

export function updatePostWithImage(
  postPk: string,
  image: string,
): Promise<Post> {
  return call('PATCH', `/v3/posts/${encodeURIComponent(postPk)}`, {
    images: [image],
  });
}

export const Visibility = {
  Public: 'PUBLIC',
  TeamOnly: 'TEAM_ONLY',
} as const;

export type Visibility = (typeof Visibility)[keyof typeof Visibility];

export function updatePostVisibility(
  postPk: string,
  visibility: Visibility,
): Promise<Post> {
  return call('PATCH', `/v3/posts/${encodeURIComponent(postPk)}`, {
    visibility,
  });
}

export function publishPost(
  postPk: string,
  title: string,
  content: string,
): Promise<Post> {
  return call('PATCH', `/v3/posts/${encodeURIComponent(postPk)}`, {
    publish: true,
    title,
    content,
  });
}

export async function listPosts(bookmark?: string): Promise<ListPostResponse> {
  let path = '/v3/posts';
  if (bookmark) {
    path += `?bookmark=${encodeURIComponent(bookmark)}`;
  }

  return call('GET', path);
}

export async function likePost(
  postPk: string,
  like: boolean,
): Promise<LikePostResponse> {
  return call('POST', `/v3/posts/${encodeURIComponent(postPk)}/like`, { like });
}

export type Post = {
  pk: string;
  created_at: number;
  updated_at: number;
  title: string;
  html_contents: string;
  post_type: PostType;
  status: number;
  visibility?: number;
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
  booster?: BoosterType;
  rewards?: number;
  urls: string[];
};

export const PostType = {
  Post: 1,
  Repost: 2,
  Artwork: 3,
} as const;

export type PostType = (typeof PostType)[keyof typeof PostType];

export type CreatePostResponse = {
  post_pk: string;
};

export type LikePostResponse = {
  like: boolean;
};

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
