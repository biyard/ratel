export enum RewardUserBehavior {
  RespondPoll = 'RespondPoll',
  // BoardComment = 'BoardComment',
  // ParticipateQuiz = 'ParticipateQuiz',
}

export function getRewardUserBehaviorI18nKey(
  behavior: RewardUserBehavior,
): string {
  switch (behavior) {
    case RewardUserBehavior.RespondPoll:
      return 'respond_poll';
    default:
      return 'unknown';
  }
}
