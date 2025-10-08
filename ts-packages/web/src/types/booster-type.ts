export const BoosterType = {
  NoBoost: 1,
  X2: 2,
  X10: 3,
  X100: 4,
} as const;

export type BoosterType = typeof BoosterType[keyof typeof BoosterType];
