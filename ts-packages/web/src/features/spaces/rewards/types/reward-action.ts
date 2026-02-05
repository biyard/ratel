export enum RewardAction {
  Poll = 'Poll',
  // Quiz = 'Quiz',
}

import { RewardUserBehavior } from './reward-user-behavior';

export function getRewardActionI18nKey(type: RewardAction): string {
  switch (type) {
    case RewardAction.Poll:
      return 'poll';
    default:
      return 'unknown';
  }
}

export function getRewardUserBehaviorFromAction(
  action: RewardAction,
): RewardUserBehavior[] {
  switch (action) {
    case RewardAction.Poll:
      return [RewardUserBehavior.RespondPoll];
    default:
      return [];
  }
}
