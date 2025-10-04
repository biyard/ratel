import { FeedType } from '../feeds';

export interface CreateDraftRequest {
  create_draft: {
    feed_type: FeedType;
    user_id: number | string;
  };
}

export function createDraftRequest(
  feed_type: FeedType,
  user_id: number | string,
): CreateDraftRequest {
  return {
    create_draft: {
      feed_type,
      user_id,
    },
  };
}
