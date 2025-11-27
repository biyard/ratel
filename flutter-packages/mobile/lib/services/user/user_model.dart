class UserV2Model {
  final String pk;
  final String email;
  final String nickname;
  final String profileUrl;
  final String description;
  final int userType;
  final String username;

  final int followersCount;
  final int followingsCount;

  final int theme;
  final int point;

  final String? referralCode;
  final String? phoneNumber;
  final String? principal;
  final String? evmAddress;

  final List<TeamV2> teams;

  const UserV2Model({
    required this.pk,
    required this.email,
    required this.nickname,
    required this.profileUrl,
    required this.description,
    required this.userType,
    required this.username,
    required this.followersCount,
    required this.followingsCount,
    required this.theme,
    required this.point,
    this.referralCode,
    this.phoneNumber,
    this.principal,
    this.evmAddress,
    this.teams = const [],
  });

  factory UserV2Model.fromJson(Map<String, dynamic> json) {
    final teamsJson = (json['teams'] as List?) ?? const [];

    return UserV2Model(
      pk: json['pk'] as String,
      email: json['email'] as String? ?? '',
      nickname: json['nickname'] as String? ?? '',
      profileUrl: json['profile_url'] as String? ?? '',
      description: json['description'] as String? ?? '',
      userType: (json['user_type'] as num?)?.toInt() ?? 0,
      username: json['username'] as String? ?? '',
      followersCount: (json['followers_count'] as num?)?.toInt() ?? 0,
      followingsCount: (json['followings_count'] as num?)?.toInt() ?? 0,
      theme: (json['theme'] as num?)?.toInt() ?? 0,
      point: (json['point'] as num?)?.toInt() ?? 0,
      referralCode: json['referral_code'] as String?,
      phoneNumber: json['phone_number'] as String?,
      principal: json['principal'] as String?,
      evmAddress: json['evm_address'] as String?,
      teams: teamsJson
          .map((e) => TeamV2.fromJson(e as Map<String, dynamic>))
          .toList(),
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'pk': pk,
      'email': email,
      'nickname': nickname,
      'profile_url': profileUrl,
      'description': description,
      'user_type': userType,
      'username': username,
      'followers_count': followersCount,
      'followings_count': followingsCount,
      'theme': theme,
      'point': point,
      'referral_code': referralCode,
      'phone_number': phoneNumber,
      'principal': principal,
      'evm_address': evmAddress,
      'teams': teams.map((t) => t.toJson()).toList(),
    };
  }
}

class TeamV2 {
  final String pk;
  final String profileUrl;
  final String nickname;
  final String username;

  const TeamV2({
    required this.pk,
    required this.profileUrl,
    required this.nickname,
    required this.username,
  });

  factory TeamV2.fromJson(Map<String, dynamic> json) {
    final rawPk = json['pk'] ?? json['id'] ?? json['username'] ?? '';

    return TeamV2(
      pk: rawPk.toString(),
      profileUrl: json['profile_url'] as String? ?? '',
      nickname: json['nickname'] as String? ?? '',
      username: json['username'] as String? ?? '',
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'pk': pk,
      'profile_url': profileUrl,
      'nickname': nickname,
      'username': username,
    };
  }
}

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
