import { SpaceDiscussionMemberResponse } from './space-discussion-member-response';

export class ListDiscussionMemberResponse {
  members: SpaceDiscussionMemberResponse[];

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  constructor(json: any) {
    const rows = Array.isArray(json)
      ? json
      : Array.isArray(json?.members)
        ? json.members
        : [];
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    this.members = rows.map((d: any) => new SpaceDiscussionMemberResponse(d));
  }
}
