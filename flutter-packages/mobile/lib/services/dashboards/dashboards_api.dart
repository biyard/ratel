import 'package:ratel/exports.dart';

class DashboardsApi extends GetConnect {
  final apiEndpoint = Config.apiEndpoint;

  DashboardsApi() {
    httpClient.addRequestModifier<void>((req) async {
      final authApi = Get.find<AuthApi>();
      final cookie = await authApi.cookieHeaderAsync();
      logger.d('${req.method} ${req.url} | DashboardsApi Cookie: $cookie');
      if (cookie?.isNotEmpty == true) req.headers['Cookie'] = cookie!;
      return req;
    });
  }

  Future<DashboardsModel> getDashboards() async {
    final uri = Uri.parse(apiEndpoint).resolve('/v2/dashboards');
    final headers = <String, String>{'Content-Type': 'application/json'};
    final res = await get(uri.toString(), headers: headers);

    if (!res.isOk) {
      return DashboardsModel(topSpaces: [], matchedFeeds: [], newFeeds: []);
    }

    logger.d("top spaces: ${res.body["top_spaces"]}");
    logger.d("matched_feeds: ${res.body["matched_feeds"]}");
    logger.d("new feeds: ${res.body["new_feeds"]}");

    final List<SpaceSummary> topSpaces = [];
    final List<FeedSummary> matchedFeeds = [];
    final List<FeedSummary> newFeeds = [];

    for (var i = 0; i < res.body["top_spaces"].length; i++) {
      final space = res.body["top_spaces"][i];
      final author = space["author"][0];

      topSpaces.add(
        SpaceSummary(
          id: int.parse(space["id"].toString()),
          createdAt: int.parse(space["created_at"].toString()),
          updatedAt: int.parse(space["updated_at"].toString()),
          feedId: int.parse(space["feed_id"].toString()),
          title: space["title"] ?? "",
          htmlContents: space["html_contents"] ?? "",
          imageUrl: space["image_url"] ?? "",
          authorUrl: author["profile_url"] ?? "",
          authorName: author["username"] ?? "",
          likes: int.parse(space["likes"].toString()),
          rewards: int.parse(space["rewards"].toString()),
          comments: int.parse(space["number_of_comments"].toString()),
        ),
      );
    }

    for (var i = 0; i < res.body["matched_feeds"].length; i++) {
      final feed = res.body["matched_feeds"][i];
      final List<int> spaceIds = ((feed['spaces'] ?? []) as List)
          .map((s) => s?['id'])
          .where((id) => id != null)
          .map<int>((id) => id is int ? id : int.parse(id.toString()))
          .toList();
      final feedType = (feed["industry"].length == 0)
          ? "Crypto"
          : feed["industry"][0]["name"];

      matchedFeeds.add(
        FeedSummary(
          feedId: int.parse(feed["id"].toString()),
          spaceIds: spaceIds,
          feedType: feedType,
          image: feed["url"] ?? "",
          title: feed["title"] ?? "",
          description: feed["html_contents"] ?? "",
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

    for (var i = 0; i < res.body["new_feeds"].length; i++) {
      final feed = res.body["new_feeds"][i];
      final List<int> spaceIds = ((feed['spaces'] ?? []) as List)
          .map((s) => s?['id'])
          .where((id) => id != null)
          .map<int>((id) => id is int ? id : int.parse(id.toString()))
          .toList();
      final feedType = (feed["industry"].length == 0)
          ? "Crypto"
          : feed["industry"][0]["name"];

      newFeeds.add(
        FeedSummary(
          feedId: int.parse(feed["id"].toString()),
          spaceIds: spaceIds,
          feedType: feedType,
          image: feed["url"] ?? "",
          title: feed["title"] ?? "",
          description: feed["html_contents"] ?? "",
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

    return DashboardsModel(
      topSpaces: topSpaces,
      matchedFeeds: matchedFeeds,
      newFeeds: newFeeds,
    );
  }
}
