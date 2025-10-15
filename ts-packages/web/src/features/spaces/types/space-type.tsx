export enum SpaceType {
  Legislation = 1,
  Poll = 2,
  Deliberation = 3,
  Nft = 4,
  Commitee = 5,
  SprintLeague = 6,
  Notice = 7,
  dAgit = 8,
}

export function getSpaceTypeLabel(type: SpaceType): string {
  switch (type) {
    case SpaceType.Notice:
      return 'Notice';
    case SpaceType.Deliberation:
      return 'Deliberation';
    case SpaceType.SprintLeague:
      return 'Sprint League';
    case SpaceType.Poll:
      return 'Poll';
    case SpaceType.dAgit:
      return 'd.AGIT';
    case SpaceType.Nft:
      return 'NFT';
    default:
      return 'Space';
  }
}
