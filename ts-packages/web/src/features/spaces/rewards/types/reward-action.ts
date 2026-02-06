export enum RewardAction {
  Poll = 'Poll',
  // Quiz = 'Quiz',
}

import { RewardUserBehavior } from './reward-user-behavior';

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
