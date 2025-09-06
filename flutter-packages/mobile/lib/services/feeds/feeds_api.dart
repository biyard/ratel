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

  Future<dynamic> addBookmark(int feedId) async {
    final uri = Uri.parse(apiEndpoint).resolve('/v2/bookmarks/add');
    final headers = <String, String>{'Content-Type': 'application/json'};
    final body = {'feed_id': feedId};

    final res = await post(uri.toString(), body, headers: headers);

    logger.d('response body: ${res.isOk} ');

    if (!res.isOk) return null;

    logger.d('response body: ${res.body}');

    return "success";
  }

  Future<dynamic> removeBookmark(int feedId) async {
    final uri = Uri.parse(apiEndpoint).resolve('/v2/bookmarks/remove');
    final headers = <String, String>{'Content-Type': 'application/json'};
    final body = {'feed_id': feedId};

    final res = await post(uri.toString(), body, headers: headers);

    logger.d('response body: ${res.isOk} ');

    if (!res.isOk) return null;

    logger.d('response body: ${res.body}');

    return "success";
  }

  Future<dynamic> deleteFeed(int feedId) async {
    final uri = Uri.parse(apiEndpoint)
        .resolve('/v1/feeds/$feedId')
        .replace(queryParameters: {'action': 'delete'});
    final headers = <String, String>{'Content-Type': 'application/json'};
    final body = {'delete': {}};

    final res = await post(uri.toString(), body, headers: headers);

    logger.d('response body: ${res.isOk} ');

    if (!res.isOk) return null;

    return "success";
  }

  Future<List<FeedSummary>> listBookmarkedFeeds() async {
    final uri = Uri.parse(apiEndpoint).resolve('/v2/bookmarks');

    final headers = <String, String>{'Content-Type': 'application/json'};
    final res = await get(uri.toString(), headers: headers);

    if (!res.isOk) return [];

    final List<FeedSummary> feeds = [];

    final items = res.body["bookmarked_feeds"];

    logger.d("bookmarked feeds: $items");

    for (var i = 0; i < items.length; i++) {
      final feed = items[i];
      final List<int> spaceIds = ((feed['spaces'] ?? []) as List)
          .map((s) => s?['id'])
          .where((id) => id != null)
          .map<int>((id) => id is int ? id : int.parse(id.toString()))
          .toList();
      final feedType = (feed["industry"].length == 0)
          ? "Crypto"
          : feed["industry"][0]["name"];

      feeds.add(
        FeedSummary(
          feedId: int.parse(items[i]["id"].toString()),
          spaceIds: spaceIds,
          feedType: feedType,
          image: feed["url"] ?? "",
          title: feed["title"] ?? "",
          description: feed["html_contents"] ?? "",
          isBookmarked: bool.parse(feed["is_bookmarked"].toString()) ?? false,
          authorId: (feed["author"].length != 0)
              ? int.parse(feed["author"][0]["id"].toString())
              : 0,
          authorUrl: (feed["author"].length != 0)
              ? feed["author"][0]["profile_url"] ?? ""
              : "",
          authorName: (feed["author"].length != 0)
              ? feed["author"][0]["nickname"] ?? ""
              : "",
          createdAt: int.parse(feed["created_at"].toString()),
          rewards: int.parse(feed["rewards"].toString()),
          likes: int.parse(feed["likes"].toString()),
          comments: int.parse(feed["comments"].toString()),
          reposts: int.parse(feed["shares"].toString()),
        ),
      );
    }

    return feeds;
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

  Future<FeedModel> getFeedById(int feedId) async {
    final uri = Uri.parse(apiEndpoint).resolve('/v1/feeds/${feedId}');
    final headers = <String, String>{'Content-Type': 'application/json'};
    final res = await get(uri.toString(), headers: headers);

    if (!res.isOk) {
      return FeedModel(
        feedId: 0,
        spaceIds: [],
        feedType: '',
        image: '',
        title: '',
        description: '',
        authorId: 0,
        authorUrl: '',
        authorName: '',
        createdAt: 0,
        rewards: 0,
        likes: 0,
        comments: 0,
        reposts: 0,
      );
    }
    final item = res.body;
    final List<int> spaceIds = ((item['spaces'] ?? []) as List)
        .map((s) => s?['id'])
        .where((id) => id != null)
        .map<int>((id) => id is int ? id : int.parse(id.toString()))
        .toList();

    return FeedModel(
      feedId: int.parse(item["id"].toString()),
      spaceIds: spaceIds,
      //FIXME: fix to real feed type
      feedType: "Crypto",
      image: item["url"] ?? "",
      title: item["title"] ?? '',
      description: item["html_contents"] ?? "",
      authorId: (item["author"].length != 0)
          ? int.parse(item["author"][0]["id"].toString())
          : 0,
      authorUrl: (item["author"].length != 0)
          ? item["author"][0]["profile_url"] ?? ""
          : "",
      authorName: (item["author"].length != 0)
          ? item["author"][0]["nickname"] ?? ""
          : "",
      createdAt: int.parse(item["created_at"].toString()),
      rewards: int.parse(item["rewards"].toString()),
      likes: int.parse(item["likes"].toString()),
      comments: int.parse(item["comments"].toString()),
      reposts: int.parse(item["shares"].toString()),
    );
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
