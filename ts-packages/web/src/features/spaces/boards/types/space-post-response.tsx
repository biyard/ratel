import FileModel from '../../files/types/file';
import { SpacePostCommentResponse } from './space-post-comment-response';

export class SpacePostResponse {
  pk: string;

  created_at: number;
  updated_at: number;
  title: string;
  html_contents: string;
  category_name: string;
  number_of_comments: number;

  user_pk: string;
  author_display_name: string;
  author_profile_url: string;
  author_username: string;

  urls: string[];
  files: FileModel[];
  comments: SpacePostCommentResponse[];

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  constructor(json: any) {
    const rows = Array.isArray(json.comments) ? json.comments : [];

    this.pk = json.pk;

    this.created_at = json.created_at;
    this.updated_at = json.updated_at;
    this.title = json.title;
    this.html_contents = json.html_contents;
    this.category_name = json.category_name;
    this.number_of_comments = json.number_of_comments;

    this.user_pk = json.user_pk;
    this.author_display_name = json.author_display_name;
    this.author_profile_url = json.author_profile_url;
    this.author_username = json.author_username;

    this.urls = json.urls;
    this.files = json.files;

    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    this.comments = rows.map((d: any) => new SpacePostCommentResponse(d));
  }
}
