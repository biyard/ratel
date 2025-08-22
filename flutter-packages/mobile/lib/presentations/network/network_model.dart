class InvitationModel {
  final int id;
  final String nickname;
  final String profileUrl;
  final String username;
  final String description;
  InvitationModel({
    required this.id,
    required this.nickname,
    required this.profileUrl,
    required this.username,
    required this.description,
  });
}

class SuggestionModel {
  final int id;
  final String nickname;
  final String profileUrl;
  final String description;
  final int? spaces;
  final int? follows;

  SuggestionModel({
    required this.id,
    required this.nickname,
    required this.profileUrl,
    required this.description,
    required this.spaces,
    required this.follows,
  });
}
