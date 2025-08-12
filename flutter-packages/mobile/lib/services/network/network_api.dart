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
