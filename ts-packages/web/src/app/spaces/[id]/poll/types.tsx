export const PollTab = {
  POLL: 'Poll',
  ANALYZE: 'Analyze',
} as const;

export type PollTabType = (typeof PollTab)[keyof typeof PollTab];
