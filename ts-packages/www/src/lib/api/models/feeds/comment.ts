export interface CommentRequest {
  comment: {
    html_contents: string;
    user_id: number;
    parent_id: number;
  };
}

export function writeCommentRequest(
  html_contents: string,
  user_id: number,
  parent_id: number,
): CommentRequest {
  return {
    comment: {
      html_contents,
      user_id,
      parent_id,
    },
  };
}
