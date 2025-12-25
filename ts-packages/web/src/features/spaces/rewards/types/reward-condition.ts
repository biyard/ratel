export type RewardCondition =
  | 'None'
  | { MaxClaims: number }
  | { MaxPoints: number }
  | { MaxUserClaims: number }
  | { MaxUserPoints: number };

export enum ConditionType {
  None = 'None',
  MaxClaims = 'MaxClaims',
  MaxPoints = 'MaxPoints',
  MaxUserClaims = 'MaxUserClaims',
  MaxUserPoints = 'MaxUserPoints',
}
export function isConditionNone(c: RewardCondition): c is 'None' {
  return c === 'None';
}

export function getConditionValue(c: RewardCondition): number | null {
  if (c === 'None') return null;
  if ('MaxClaims' in c) return c.MaxClaims;
  if ('MaxPoints' in c) return c.MaxPoints;
  if ('MaxUserClaims' in c) return c.MaxUserClaims;
  if ('MaxUserPoints' in c) return c.MaxUserPoints;
  return null;
}

export function getConditionType(c: RewardCondition): ConditionType {
  if (c === 'None') return ConditionType.None;
  return Object.keys(c)[0] as ConditionType;
}
