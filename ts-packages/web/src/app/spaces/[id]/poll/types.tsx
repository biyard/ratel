export const PollTab = {
  POLL: 'Poll',
  ANALYZE: 'Analyze',
} as const;

export type PollTab = typeof PollTab[keyof typeof PollTab];

export type PollTabType = PollTab;
