export const DeliberationTab = {
  SUMMARY: 'Summary',
  DELIBERATION: 'Deliberation',
  POLL: 'Poll',
  RECOMMANDATION: 'Recommendation',
  ANALYZE: 'Analyze',
} as const;

export type DeliberationTab =
  (typeof DeliberationTab)[keyof typeof DeliberationTab];

export type DeliberationTabType = DeliberationTab;
