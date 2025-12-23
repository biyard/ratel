import { RewardTypeRequest } from './reward-type-request';

export enum RewardAction {
  PollRespond = 'POLL_RESPOND',
}

export function getRewardActionI18nKey(type: RewardAction): string {
  switch (type) {
    case RewardAction.PollRespond:
      return 'poll_respond';
    default:
      return 'unknown';
  }
}

export function convertRewardActionToRequest(
  rewardAction: RewardAction,
  entityKey: string,
): RewardTypeRequest {
  switch (rewardAction) {
    case RewardAction.PollRespond:
      return {
        poll_sk: entityKey,
      };
    default:
      throw new Error('Invalid reward action');
  }
}
