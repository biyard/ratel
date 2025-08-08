// Shared constants and utilities for reward calculations

export const BASE_REWARD = 10_000;

export const BOOSTER_MULTIPLIERS: Record<string, number> = {
  none: 0,
  '0': 0,
  '1': 0, // NoBoost = 0 reward
  '2': 2,
  '3': 10,
  '4': 100,
};

export const getBoosterMultiplier = (boosterType?: string | number): number => {
  const typeStr = String(boosterType).toLowerCase();
  return BOOSTER_MULTIPLIERS[typeStr] ?? 0; // Default to 0 if unknown
};

export const calculateBaseReward = (boosterType?: string | number): number => {
  return BASE_REWARD * getBoosterMultiplier(boosterType);
};

export const calculateRewardWithPenalties = (
  boosterType?: string | number,
  penaltyCount: number = 0,
): number => {
  const baseValue = calculateBaseReward(boosterType);
  const penaltyMultiplier = Math.pow(0.5, Math.min(penaltyCount, 2));
  return baseValue * penaltyMultiplier;
};

export const formatRewardAmount = (amount: number): string => {
  return `+${amount.toLocaleString()} P`;
};
