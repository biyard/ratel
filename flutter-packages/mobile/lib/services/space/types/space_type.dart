enum SpaceType {
  legislation,
  poll,
  deliberation,
  nft,
  commitee,
  sprintLeague,
  notice,
  dagit,
}

SpaceType spaceTypeFromJson(dynamic v) {
  final n = _asIntOrNull(v) ?? 1;
  switch (n) {
    case 1:
      return SpaceType.legislation;
    case 2:
      return SpaceType.poll;
    case 3:
      return SpaceType.deliberation;
    case 4:
      return SpaceType.nft;
    case 5:
      return SpaceType.commitee;
    case 6:
      return SpaceType.sprintLeague;
    case 7:
      return SpaceType.notice;
    case 8:
      return SpaceType.dagit;
    default:
      return SpaceType.legislation;
  }
}

int spaceTypeToJson(SpaceType t) {
  switch (t) {
    case SpaceType.legislation:
      return 1;
    case SpaceType.poll:
      return 2;
    case SpaceType.deliberation:
      return 3;
    case SpaceType.nft:
      return 4;
    case SpaceType.commitee:
      return 5;
    case SpaceType.sprintLeague:
      return 6;
    case SpaceType.notice:
      return 7;
    case SpaceType.dagit:
      return 8;
  }
}

int? _asIntOrNull(dynamic v) {
  if (v == null) return null;
  if (v is int) return v;
  if (v is num) return v.toInt();
  return int.tryParse(v.toString());
}
