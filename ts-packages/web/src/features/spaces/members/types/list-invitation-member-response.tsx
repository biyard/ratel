import { InvitationMemberResponse } from './invitation-member-response';

export class ListInvitationMemberResponse {
  members: InvitationMemberResponse[];

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  constructor(json: any) {
    const rows = Array.isArray(json)
      ? json
      : Array.isArray(json?.members)
        ? json.members
        : [];
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    this.members = rows.map((d: any) => new InvitationMemberResponse(d));
  }
}
