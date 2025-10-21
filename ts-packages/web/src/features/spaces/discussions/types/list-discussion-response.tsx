import { SpaceDiscussionResponse } from './space-discussion-response';

export class ListDiscussionResponse {
  discussions: SpaceDiscussionResponse[];
  bookmark: string | null | undefined;

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  constructor(json: any) {
    const rows = Array.isArray(json.discussions) ? json.discussions : [];
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    this.discussions = rows.map((d: any) => new SpaceDiscussionResponse(d));
    this.bookmark = json.bookmark;
  }
}
