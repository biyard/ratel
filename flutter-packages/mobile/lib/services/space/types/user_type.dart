enum UserType { individual, team, bot, anonymousSpaceUser, admin, anonymous }

UserType userTypeFromJson(dynamic v) {
  if (v == null) return UserType.individual;

  int? code;
  if (v is int) {
    code = v;
  } else {
    code = int.tryParse(v.toString());
  }

  switch (code) {
    case 1:
      return UserType.individual;
    case 2:
      return UserType.team;
    case 3:
      return UserType.bot;
    case 4:
      return UserType.anonymousSpaceUser;
    case 98:
      return UserType.admin;
    case 99:
      return UserType.anonymous;
    default:
      return UserType.individual;
  }
}

int userTypeToJson(UserType t) {
  switch (t) {
    case UserType.individual:
      return 1;
    case UserType.team:
      return 2;
    case UserType.bot:
      return 3;
    case UserType.anonymousSpaceUser:
      return 4;
    case UserType.admin:
      return 98;
    case UserType.anonymous:
      return 99;
  }
}
