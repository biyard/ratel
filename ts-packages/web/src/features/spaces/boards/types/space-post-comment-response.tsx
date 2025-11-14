export class SpacePostCommentResponse {
  pk: string;
  sk: string;

  created_at: number;
  updated_at: number;

  content: string;

  likes: number;
  replies: number;

  parent_comment_sk: string | null | undefined;

  author_pk: string;
  author_display_name: string;
  author_username: string;
  author_profile_url: string;

  liked: boolean;

  //eslint-disable-next-line @typescript-eslint/no-explicit-any
  constructor(json: any) {
    this.pk = json.pk;
    this.sk = json.sk;

    this.created_at = json.created_at ?? 0;
    this.updated_at = json.updated_at;

    this.content = json.content;

    this.likes = json.likes;
    this.replies = json.replies;

    this.parent_comment_sk = json.parent_comment_sk;

    this.author_pk = json.author_pk;
    this.author_display_name = json.author_display_name;
    this.author_username = json.author_username;
    this.author_profile_url = json.author_profile_url;

    this.liked = json.liked;
  }
}
