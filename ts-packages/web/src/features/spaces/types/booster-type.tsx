export enum BoosterType {
  NoBoost = 1,
  X2 = 2,
  X10 = 3,
  X100 = 4,
}

export function BoosterTypeToMultiplier(boostType: BoosterType): number {
  switch (boostType) {
    case BoosterType.X2:
      return 2;
    case BoosterType.X10:
      return 10;
    case BoosterType.X100:
      return 100;
    case BoosterType.NoBoost:
    default:
      return 1;
  }
}
