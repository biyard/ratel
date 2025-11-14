interface PostComment {
  pk: string;
  sk: string;
  updated_at: number;
  created_at?: number;
  content: string;
  author_pk: string;
  author_display_name: string;
  author_profile_url: string;
  author_username: string;

  likes: number;
  replies: number;

  parent_comment_pk: string;

  liked: boolean;
}

export default PostComment;
