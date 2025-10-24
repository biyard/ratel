import { SpaceDiscussionParticipantResponse } from './space-discussion-participant-response';

export class ListDiscussionParticipantResponse {
  participants: SpaceDiscussionParticipantResponse[];

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  constructor(json: any) {
    const rows = Array.isArray(json)
      ? json
      : Array.isArray(json?.participants)
        ? json.participants
        : [];
    this.participants = rows.map(
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      (d: any) => new SpaceDiscussionParticipantResponse(d),
    );
  }
}
