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

  Future<MyNetworkModel> getNetworksByV1() async {
    final userUri = Uri.parse(apiEndpoint)
        .resolve('/v1/users')
        .replace(queryParameters: <String, String>{'action': 'user-info'});

    final networkUri = Uri.parse(apiEndpoint)
        .resolve('/v1/network')
        .replace(
          queryParameters: <String, String>{
            'param-type': 'read',
            'action': 'find-one',
          },
        );

    final headers = <String, String>{'Content-Type': 'application/json'};
    final userRes = await get(userUri.toString(), headers: headers);
    final networkRes = await get(networkUri.toString(), headers: headers);

    logger.d(
      "user info: ${userRes.body["id"]} followings: ${userRes.body["followings"]} followers: ${userRes.body["followers"]}",
    );
    logger.d("network info: ${networkRes.body["suggested_users"]}");

    final List<NetworkModel> followers = [];
    final List<NetworkModel> followings = [
      NetworkModel(
        id: 51,
        profileUrl:
            "https://ca.slack-edge.com/T03H3B09USV-U03GQMUNE2W-20ccd88c2612-512",
        nickname: "Summer Park",
        username: "Summer Park",
        description:
            "<div>educator in the Korean Web3 industry, specializing in industry convergence and DAO technology and applications in the context of blockchain and Web3.0</div>",
      ),
      NetworkModel(
        id: 52,
        profileUrl:
            "https://lh3.googleusercontent.com/a/ACg8ocJH35xGbTc7wWb1C8n55KDYdoIKAthJEvFGYXRP9qgFRO9dWM8=s96-c",
        nickname: "Rosa Park",
        username: "Rosa Park",
        description: "<div>Project manager at Ratel foundation</div>",
      ),
    ];

    if (!userRes.isOk || !networkRes.isOk) {
      return MyNetworkModel(followers: followers, followings: followings);
    }

    // final followersRaw = userRes.body["followers"] as List? ?? [];
    // final followingRaw = userRes.body["followings"] as List? ?? [];

    // final followerIds = followersRaw
    //     .map((e) => e is Map ? e["id"] : e)
    //     .map((v) => int.tryParse(v?.toString() ?? ''))
    //     .whereType<int>()
    //     .toSet();

    // final alreadyAdded = followings.map((n) => n.id).toSet();

    // for (final f in followingRaw) {
    //   final id = int.tryParse(f["id"]?.toString() ?? '');
    //   logger.d("following raws ids: ${followerIds} ${id}");
    //   if (id == null) continue;
    //   if (followerIds.contains(id)) continue;
    //   if (alreadyAdded.contains(id)) continue;

    //   followings.add(
    //     NetworkModel(
    //       id: id,
    //       profileUrl: f["profile_url"] ?? "",
    //       nickname: f["nickname"] ?? "",
    //       username: f["username"] ?? "",
    //       description: f["html_contents"] ?? "",
    //     ),
    //   );
    // }

    for (
      var i = 0;
      i < min(networkRes.body["suggested_users"].length, 4);
      i++
    ) {
      logger.d("index: ${i} network: ${networkRes.body["suggested_users"]}");
      final follower = networkRes.body["suggested_users"][i];
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

  Future<dynamic> getNetworks() async {
    final uri = Uri.parse(apiEndpoint).resolve('/v2/networks');

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

  Future<dynamic> getNetworksByKeyword(String keyword) async {
    final uri = Uri.parse(apiEndpoint)
        .resolve('/v2/networks/search')
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

  Future<dynamic> follow(List<int> followeeIds) async {
    final uri = Uri.parse(apiEndpoint).resolve('/v2/networks/follow');

    final headers = <String, String>{'Content-Type': 'application/json'};
    final body = {'followee_ids': followeeIds};

    final res = await post(uri.toString(), body, headers: headers);

    if (!res.isOk) return null;

    logger.d('response body: ${res.body}');

    return res.isOk;
  }
}
