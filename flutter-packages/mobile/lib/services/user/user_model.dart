//FIXME: add field
class UserModel {
  final int id;
  final String profileUrl;
  final String nickname;
  final String username;
  final int points;
  final int followingsCount;
  final int followersCount;

  final List<Team> teams;

  const UserModel({
    required this.id,
    required this.profileUrl,
    required this.nickname,
    required this.username,
    required this.points,
    required this.followersCount,
    required this.followingsCount,

    required this.teams,
  });
}

class Team {
  final int id;
  final String profileUrl;
  final String nickname;
  final String username;

  const Team({
    required this.id,
    required this.profileUrl,
    required this.nickname,
    required this.username,
  });
}
