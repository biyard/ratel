import { SpacePostResponse } from './space-post-response';

export class ListSpacePostsResponse {
  posts: SpacePostResponse[];
  bookmark: string | null | undefined;

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  constructor(json: any) {
    const rows = Array.isArray(json.posts) ? json.posts : [];
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    this.posts = rows.map((d: any) => new SpacePostResponse(d));
    this.bookmark = json.bookmark;
  }
}
