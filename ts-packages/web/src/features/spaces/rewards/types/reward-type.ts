export enum RewardType {
  PollRespond = 'POLL_RESPOND',
}

export function getRewardTypeI18nKey(type: RewardType): string {
  switch (type) {
    case RewardType.PollRespond:
      return 'poll_respond';
    default:
      return 'unknown';
  }
}
