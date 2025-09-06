import 'dart:math';

import 'package:ratel/exports.dart';

class NetworkApi extends GetConnect {
  final apiEndpoint = Config.apiEndpoint;

  NetworkApi() {
    httpClient.addRequestModifier<void>((req) async {
      final authApi = Get.find<AuthApi>();
      final cookie = await authApi.cookieHeaderAsync();
      logger.d('${req.method} ${req.url} | IndustryApi Cookie: $cookie');
      if (cookie?.isNotEmpty == true) req.headers['Cookie'] = cookie!;
      return req;
    });
  }

  Future<NetworkModel?> acceptSuggestion(
    List<int> suggestionIds,
    int followeeId,
  ) async {
    final uri = Uri.parse(
      apiEndpoint,
    ).resolve('/v2/networks/suggestions/accept');

    final headers = <String, String>{'Content-Type': 'application/json'};
    final body = {'suggestion_ids': suggestionIds, 'followee_id': followeeId};

    final res = await post(uri.toString(), body, headers: headers);

    logger.d('response body: ${res.bodyString} ');

    if (!res.isOk) return null;

    final suggestion = res.body["suggestion"];

    return NetworkModel(
      id: int.parse(suggestion["id"].toString()),
      profileUrl: suggestion["profile_url"] ?? "",
      nickname: suggestion["nickname"] ?? "",
      username: suggestion["username"] ?? "",
      description: suggestion["html_contents"] ?? "",
    );
  }

  Future<NetworkModel?> rejectSuggestion(
    List<int> suggestionIds,
    int followeeId,
  ) async {
    final uri = Uri.parse(
      apiEndpoint,
    ).resolve('/v2/networks/suggestions/reject');

    final headers = <String, String>{'Content-Type': 'application/json'};
    final body = {'suggestion_ids': suggestionIds, 'followee_id': followeeId};

    final res = await post(uri.toString(), body, headers: headers);

    logger.d('response body: ${res.isOk} ');

    if (!res.isOk) return null;

    final suggestion = res.body["suggestion"];

    return NetworkModel(
      id: int.parse(suggestion["id"].toString()),
      profileUrl: suggestion["profile_url"] ?? "",
      nickname: suggestion["nickname"] ?? "",
      username: suggestion["username"] ?? "",
      description: suggestion["html_contents"] ?? "",
    );
  }

  Future<NetworkModel?> acceptInvitation(
    List<int> invitationIds,
    int followeeId,
  ) async {
    final uri = Uri.parse(
      apiEndpoint,
    ).resolve('/v2/networks/invitations/accept');

    final headers = <String, String>{'Content-Type': 'application/json'};
    final body = {'followee_id': followeeId, 'invitation_ids': invitationIds};

    final res = await post(uri.toString(), body, headers: headers);

    logger.d('response body: ${res.isOk} ');

    if (!res.isOk) return null;

    final invitation = res.body["invitation"];

    if (invitation == null) {
      return NetworkModel(
        id: 0,
        profileUrl: "",
        nickname: "",
        username: "",
        description: "",
      );
    }

    return NetworkModel(
      id: int.parse(invitation["id"].toString()),
      profileUrl: invitation["profile_url"] ?? "",
      nickname: invitation["nickname"] ?? "",
      username: invitation["username"] ?? "",
      description: invitation["html_contents"] ?? "",
    );
  }

  Future<NetworkModel?> rejectInvitation(
    List<int> invitationIds,
    int followeeId,
  ) async {
    final uri = Uri.parse(
      apiEndpoint,
    ).resolve('/v2/networks/invitations/reject');

    final headers = <String, String>{'Content-Type': 'application/json'};
    final body = {'followee_id': followeeId, 'invitation_ids': invitationIds};

    final res = await post(uri.toString(), body, headers: headers);

    logger.d('response body: ${res.isOk} ');

    if (!res.isOk) return null;

    final invitation = res.body["invitation"];

    if (invitation == null) {
      return NetworkModel(
        id: 0,
        profileUrl: "",
        nickname: "",
        username: "",
        description: "",
      );
    }

    return NetworkModel(
      id: int.parse(invitation["id"].toString()),
      profileUrl: invitation["profile_url"] ?? "",
      nickname: invitation["nickname"] ?? "",
      username: invitation["username"] ?? "",
      description: invitation["html_contents"] ?? "",
    );
  }

  Future<MyNetworkModel> getNetworksByV1() async {
    final userUri = Uri.parse(apiEndpoint)
        .resolve('/v1/users')
        .replace(queryParameters: <String, String>{'action': 'user-info'});

    final networkUri = Uri.parse(apiEndpoint).resolve('/v2/networks');

    final headers = <String, String>{'Content-Type': 'application/json'};
    final userRes = await get(userUri.toString(), headers: headers);
    final networkRes = await get(networkUri.toString(), headers: headers);

    logger.d(
      "user info: ${userRes.body["id"]} followings: ${userRes.body["followings"]} followers: ${userRes.body["followers"]}",
    );
    logger.d("network info: ${networkRes.body["suggestions"]}");

    final List<NetworkModel> followers = [];
    final List<NetworkModel> followings = [];

    if (!userRes.isOk || !networkRes.isOk) {
      return MyNetworkModel(followers: followers, followings: followings);
    }

    for (final f in networkRes.body["invitations"]) {
      followings.add(
        NetworkModel(
          id: int.parse(f["id"].toString()),
          profileUrl: f["profile_url"] ?? "",
          nickname: f["nickname"] ?? "",
          username: f["username"] ?? "",
          description: f["html_contents"] ?? "",
        ),
      );
    }

    for (var i = 0; i < min(networkRes.body["suggestions"].length, 4); i++) {
      logger.d("index: ${i} network: ${networkRes.body["suggestions"]}");
      final follower = networkRes.body["suggestions"][i];
      logger.d("follower: ${follower}");
      followers.add(
        NetworkModel(
          id: int.parse(follower["id"].toString()),
          profileUrl: follower["profile_url"] ?? "",
          nickname: follower["nickname"] ?? "",
          username: follower["username"] ?? "",
          description: follower["html_contents"] ?? "",
        ),
      );
    }

    return MyNetworkModel(followers: followers, followings: followings);
  }

  Future<dynamic> getConnections() async {
    final uri = Uri.parse(apiEndpoint).resolve('/v2/connections');

    final headers = <String, String>{'Content-Type': 'application/json'};
    final res = await get(uri.toString(), headers: headers);

    if (!res.isOk) return [];

    final List<NetworkModel> networks = [];

    for (var i = 0; i < res.body["suggested_teams"].length; i++) {
      final network = res.body["suggested_teams"][i];
      networks.add(
        NetworkModel(
          id: int.parse(network["id"].toString()),
          profileUrl: network["profile_url"] ?? "",
          nickname: network["nickname"],
          username: network["username"] ?? "",
          description: network["html_contents"] ?? "",
        ),
      );
    }

    for (var i = 0; i < res.body["suggested_users"].length; i++) {
      final network = res.body["suggested_users"][i];
      networks.add(
        NetworkModel(
          id: int.parse(network["id"].toString()),
          profileUrl: network["profile_url"] ?? "",
          nickname: network["nickname"],
          username: network["username"] ?? "",
          description: network["html_contents"] ?? "",
        ),
      );
    }
    logger.d("response: ${res.body}");

    return networks;
  }

  Future<dynamic> getConnectionByKeyword(String keyword) async {
    final uri = Uri.parse(apiEndpoint)
        .resolve('/v2/connections/search')
        .replace(queryParameters: <String, String>{'keyword': keyword});

    final headers = <String, String>{'Content-Type': 'application/json'};
    final res = await get(uri.toString(), headers: headers);

    if (!res.isOk) return [];

    final List<NetworkModel> networks = [];

    for (var i = 0; i < res.body["suggested_teams"].length; i++) {
      final network = res.body["suggested_teams"][i];
      networks.add(
        NetworkModel(
          id: int.parse(network["id"].toString()),
          profileUrl: network["profile_url"] ?? "",
          nickname: network["nickname"],
          username: network["username"] ?? "",
          description: network["description"] ?? "",
        ),
      );
    }

    for (var i = 0; i < res.body["suggested_users"].length; i++) {
      final network = res.body["suggested_users"][i];
      networks.add(
        NetworkModel(
          id: int.parse(network["id"].toString()),
          profileUrl: network["profile_url"] ?? "",
          nickname: network["nickname"],
          username: network["username"] ?? "",
          description: network["description"] ?? "",
        ),
      );
    }
    logger.d("response: ${networks}");

    return networks;
  }

  Future<dynamic> connectionFollow(List<int> followeeIds) async {
    final uri = Uri.parse(apiEndpoint).resolve('/v2/connections/follow');

    final headers = <String, String>{'Content-Type': 'application/json'};
    final body = {'followee_ids': followeeIds};

    final res = await post(uri.toString(), body, headers: headers);

    if (!res.isOk) return null;

    logger.d('response body: ${res.body}');

    return res.isOk;
  }
}
