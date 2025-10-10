import { call } from './call';
import { ListResponse } from './common';
import { PostComment } from './posts.v3';

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
