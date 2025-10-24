import { SpaceDiscussionResponse } from './space-discussion-response';

export class DiscussionResponse {
  discussion: SpaceDiscussionResponse;

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  constructor(json: any) {
    this.discussion = new SpaceDiscussionResponse(json.discussion);
  }
}
