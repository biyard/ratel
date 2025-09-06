import 'package:ratel/exports.dart';

class UserApi extends GetConnect {
  final apiEndpoint = Config.apiEndpoint;

  UserApi() {
    httpClient.addRequestModifier<void>((req) async {
      final authApi = Get.find<AuthApi>();
      final cookie = await authApi.cookieHeaderAsync();
      logger.d('${req.method} ${req.url} | FeedsApi Cookie: $cookie');
      if (cookie?.isNotEmpty == true) req.headers['Cookie'] = cookie!;
      return req;
    });
  }

  //getUserInfo: () => '/v1/users?action=user-info',
  Future<UserModel> getUserInfo() async {
    final uri = Uri.parse(apiEndpoint)
        .resolve('/v1/users')
        .replace(queryParameters: <String, String>{'action': 'user-info'});

    final headers = <String, String>{'Content-Type': 'application/json'};
    final res = await get(uri.toString(), headers: headers);

    if (!res.isOk) {
      return UserModel(
        id: 0,
        profileUrl: '',
        nickname: '',
        username: '',
        points: 0,
        followersCount: 0,
        followingsCount: 0,

        teams: [],
      );
    }

    logger.d("user info: ${res.body}");

    final item = res.body;
    final List<Team> teams = [];

    for (var i = 0; i < item["teams"].length; i++) {
      final team = item["teams"][i];

      teams.add(
        Team(
          id: int.parse(team["id"].toString()),
          profileUrl: team["profile_url"] ?? "",
          nickname: team["nickname"] ?? "",
          username: team["username"] ?? "",
        ),
      );
    }

    return UserModel(
      id: int.parse(item["id"].toString()),
      profileUrl: item["profile_url"] ?? "",
      nickname: item["nickname"] ?? "",
      username: item["username"] ?? "",
      points: int.parse(item["points"].toString()),
      followersCount: int.parse(item["followers_count"].toString()),
      followingsCount: int.parse(item["followings_count"].toString()),

      teams: teams,
    );
  }
}
