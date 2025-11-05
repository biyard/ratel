import PostComment from '@/features/posts/types/post-comment';
import { call } from './call';
import { ListResponse } from './common';

export function comment(postPk: string, content: string): Promise<PostComment> {
  return call('POST', `/v3/posts/${encodeURIComponent(postPk)}/comments`, {
    content,
  });
}

export function reply(
  postPk: string,
  commentSk: string,
  content: string,
): Promise<PostComment> {
  return call(
    'POST',
    `/v3/posts/${encodeURIComponent(postPk)}/comments/${encodeURIComponent(commentSk)}`,
    {
      content,
    },
  );
}

export function likeComment(
  postPk: string,
  commentSk: string,
  like: boolean,
): Promise<PostComment> {
  return call(
    'POST',
    `/v3/posts/${encodeURIComponent(postPk)}/comments/${encodeURIComponent(commentSk)}/likes`,
    {
      like,
    },
  );
}

export function listReplies(
  postPk: string,
  commentSk: string,
  bookmark?: string,
): Promise<ListResponse<PostComment>> {
  let path = `/v3/posts/${encodeURIComponent(postPk)}/comments/${encodeURIComponent(commentSk)}`;

  if (bookmark) {
    path = `${path}?bookmark=${encodeURIComponent(bookmark)}`;
  }

  return call('GET', path);
}

export function listSpaceReplies(
  spacePk: string,
  postPk: string,
  commentSk: string,
  bookmark?: string,
): Promise<ListResponse<PostComment>> {
  let path = `/v3/spaces/${encodeURIComponent(spacePk)}/boards/${encodeURIComponent(postPk)}/comments/${encodeURIComponent(commentSk)}`;

  if (bookmark) {
    path = `${path}?bookmark=${encodeURIComponent(bookmark)}`;
  }

  return call('GET', path);
}
