enum BoosterType { noBoost, x2, x10, x100, custom }

BoosterType boosterTypeFromJson(dynamic v) {
  if (v == null) return BoosterType.noBoost;

  int? code;
  if (v is int) {
    code = v;
  } else {
    code = int.tryParse(v.toString());
  }

  switch (code) {
    case 1:
      return BoosterType.noBoost;
    case 2:
      return BoosterType.x2;
    case 3:
      return BoosterType.x10;
    case 4:
      return BoosterType.x100;
    case 255:
      return BoosterType.custom;
    default:
      return BoosterType.noBoost;
  }
}

int boosterTypeToJson(BoosterType t) {
  switch (t) {
    case BoosterType.noBoost:
      return 1;
    case BoosterType.x2:
      return 2;
    case BoosterType.x10:
      return 3;
    case BoosterType.x100:
      return 4;
    case BoosterType.custom:
      return 255;
  }
}
