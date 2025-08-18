import 'package:ratel/exports.dart';

class FeedsApi extends GetConnect {
  final apiEndpoint = Config.apiEndpoint;

  FeedsApi() {
    httpClient.addRequestModifier<void>((req) async {
      final authApi = Get.find<AuthApi>();
      final cookie = await authApi.cookieHeaderAsync();
      logger.d('${req.method} ${req.url} | FeedsApi Cookie: $cookie');
      if (cookie?.isNotEmpty == true) req.headers['Cookie'] = cookie!;
      return req;
    });
  }

  //status == 1: draft, status == 2: published
  Future<List<FeedModel>> listFeedsByUserId(
    int page,
    int size,
    int userId,
    int status,
  ) async {
    final uri = Uri.parse(apiEndpoint)
        .resolve('/v1/feeds')
        .replace(
          queryParameters: <String, String>{
            'param-type': 'query',
            'action': 'posts-by-user-id',
            'bookmark': page.toString(),
            'size': size.toString(),
            'user-id': userId.toString(),
            'status': status.toString(),
          },
        );

    final headers = <String, String>{'Content-Type': 'application/json'};
    final res = await get(uri.toString(), headers: headers);

    if (!res.isOk) return [];

    final List<FeedModel> feeds = [];

    final items = res.body["items"];

    for (var i = 0; i < items.length; i++) {
      final List<int> spaceIds = ((items[i]['spaces'] ?? []) as List)
          .map((s) => s?['id'])
          .where((id) => id != null)
          .map<int>((id) => id is int ? id : int.parse(id.toString()))
          .toList();

      feeds.add(
        FeedModel(
          feedId: int.parse(items[i]["id"].toString()),
          spaceIds: spaceIds,
          //FIXME: fix to real feed type
          feedType: "Crypto",
          image: items[i]["url"] ?? "",
          title: items[i]["title"] ?? '',
          description: items[i]["html_contents"] ?? "",
          authorId: (items[i]["author"].length != 0)
              ? int.parse(items[i]["author"][0]["id"].toString())
              : 0,
          authorUrl: (items[i]["author"].length != 0)
              ? items[i]["author"][0]["profile_url"] ?? ""
              : "",
          authorName: (items[i]["author"].length != 0)
              ? items[i]["author"][0]["nickname"] ?? ""
              : "",
          createdAt: int.parse(items[i]["created_at"].toString()),
          rewards: int.parse(items[i]["rewards"].toString()),
          likes: int.parse(items[i]["likes"].toString()),
          comments: int.parse(items[i]["comments"].toString()),
          reposts: int.parse(items[i]["shares"].toString()),
        ),
      );
    }

    logger.d("feeds: ${res.body["items"]}");

    return feeds;
  }

  Future<List<FeedModel>> listFeeds(int page, int size) async {
    final uri = Uri.parse(apiEndpoint)
        .resolve('/v1/feeds')
        .replace(
          queryParameters: <String, String>{
            'param-type': 'query',
            'bookmark': page.toString(),
            'size': size.toString(),
          },
        );

    final headers = <String, String>{'Content-Type': 'application/json'};
    final res = await get(uri.toString(), headers: headers);

    if (!res.isOk) return [];

    final List<FeedModel> feeds = [];

    final items = res.body["items"];

    for (var i = 0; i < items.length; i++) {
      final List<int> spaceIds = ((items[i]['spaces'] ?? []) as List)
          .map((s) => s?['id'])
          .where((id) => id != null)
          .map<int>((id) => id is int ? id : int.parse(id.toString()))
          .toList();

      feeds.add(
        FeedModel(
          feedId: int.parse(items[i]["id"].toString()),
          spaceIds: spaceIds,
          //FIXME: fix to real feed type
          feedType: "Crypto",
          image: items[i]["url"] ?? "",
          title: items[i]["title"] ?? '',
          description: items[i]["html_contents"] ?? "",
          authorId: (items[i]["author"].length != 0)
              ? int.parse(items[i]["author"][0]["id"].toString())
              : 0,
          authorUrl: (items[i]["author"].length != 0)
              ? items[i]["author"][0]["profile_url"] ?? ""
              : "",
          authorName: (items[i]["author"].length != 0)
              ? items[i]["author"][0]["nickname"] ?? ""
              : "",
          createdAt: int.parse(items[i]["created_at"].toString()),
          rewards: int.parse(items[i]["rewards"].toString()),
          likes: int.parse(items[i]["likes"].toString()),
          comments: int.parse(items[i]["comments"].toString()),
          reposts: int.parse(items[i]["shares"].toString()),
        ),
      );
    }

    logger.d("feeds: ${res.body["items"]}");

    return feeds;
  }
}
