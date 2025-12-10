import 'dart:convert';
import 'dart:io';

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

  Future<bool> uploadPost({
    required String postPk,
    required String title,
    required String content,
  }) async {
    final encodedPk = Uri.encodeComponent(postPk);
    final uri = Uri.parse(apiEndpoint).resolve('/v3/posts/$encodedPk');

    final authApi = Get.find<AuthApi>();
    final cookie = await authApi.cookieHeaderAsync();
    final client = HttpClient();

    try {
      final request = await client.patchUrl(uri);

      if (cookie != null && cookie.isNotEmpty) {
        request.headers.set('Cookie', cookie);
      }
      request.headers.set(HttpHeaders.contentTypeHeader, 'application/json');

      final payload = {
        'publish': true,
        'title': title,
        'content': content,
        'image_urls': null,
      };

      logger.d('uploadPost uri=$uri body=$payload');

      request.add(utf8.encode(jsonEncode(payload)));

      final response = await request.close();
      final bodyString = await response.transform(utf8.decoder).join();

      logger.d('uploadPost status=${response.statusCode} body=$bodyString');

      return response.statusCode >= 200 && response.statusCode < 300;
    } finally {
      client.close();
    }
  }

  Future<bool> updatePost({
    required String postPk,
    required String title,
    required String content,
  }) async {
    final encodedPk = Uri.encodeComponent(postPk);
    final base = Uri.parse(apiEndpoint);
    final uri = base.replace(
      pathSegments: [...base.pathSegments, 'v3', 'posts', encodedPk],
    );

    logger.d('updatePost postPk=$postPk encodedPk=$encodedPk uri=$uri');

    final authApi = Get.find<AuthApi>();
    final cookie = await authApi.cookieHeaderAsync();
    final client = HttpClient();

    try {
      final request = await client.patchUrl(uri);

      if (cookie != null && cookie.isNotEmpty) {
        request.headers.set('Cookie', cookie);
      }
      request.headers.set('Content-Type', 'application/json');

      final body = jsonEncode({'title': title, 'content': content});
      request.add(utf8.encode(body));

      final response = await request.close();
      final bodyString = await response.transform(utf8.decoder).join();

      logger.d('updatePost status=${response.statusCode} body=$bodyString');

      return response.statusCode >= 200 && response.statusCode < 300;
    } finally {
      client.close();
    }
  }

  Future<String> createPost({String? teamPk}) async {
    final uri = Uri.parse(apiEndpoint).resolve('/v3/posts');
    final authApi = Get.find<AuthApi>();
    final cookie = await authApi.cookieHeaderAsync();

    final client = HttpClient();

    try {
      final request = await client.postUrl(uri);

      if (cookie != null && cookie.isNotEmpty) {
        request.headers.set('Cookie', cookie);
      }

      if (teamPk != null) {
        request.headers.set('Content-Type', 'application/json');
        final jsonBody = jsonEncode({'team_pk': teamPk});
        request.add(utf8.encode(jsonBody));
      }

      final response = await request.close();
      final bodyString = await response.transform(utf8.decoder).join();

      logger.d('createPost status=${response.statusCode} body=$bodyString');

      if (response.statusCode < 200 || response.statusCode >= 300) {
        return '';
      }

      final decoded = jsonDecode(bodyString) as Map<String, dynamic>;
      return decoded['post_pk'] as String;
    } finally {
      client.close();
    }
  }

  Future<FeedV2ListResult> listPostsV2({String? bookmark}) async {
    final base = Uri.parse(apiEndpoint).resolve('/v3/me/posts');
    final uri = bookmark == null
        ? base
        : base.replace(queryParameters: <String, String>{'bookmark': bookmark});

    final headers = <String, String>{'Content-Type': 'application/json'};
    final res = await get(uri.toString(), headers: headers);

    if (!res.isOk) {
      return const FeedV2ListResult(items: [], bookmark: null);
    }

    final body = res.body;
    final items = (body['items'] as List?) ?? const [];
    final nextBookmark = body['bookmark'] as String?;

    final posts = items
        .map(
          (e) =>
              FeedV2SummaryModel.fromJson(Map<String, dynamic>.from(e as Map)),
        )
        .toList();

    return FeedV2ListResult(items: posts, bookmark: nextBookmark);
  }

  Future<FeedV2ListResult> listDraftsV2({String? bookmark}) async {
    final base = Uri.parse(apiEndpoint).resolve('/v3/me/drafts');
    final uri = bookmark == null
        ? base
        : base.replace(queryParameters: <String, String>{'bookmark': bookmark});

    final headers = <String, String>{'Content-Type': 'application/json'};
    final res = await get(uri.toString(), headers: headers);

    if (!res.isOk) {
      return const FeedV2ListResult(items: [], bookmark: null);
    }

    final body = res.body;
    final items = (body['items'] as List?) ?? const [];
    final nextBookmark = body['bookmark'] as String?;

    final drafts = items
        .map(
          (e) =>
              FeedV2SummaryModel.fromJson(Map<String, dynamic>.from(e as Map)),
        )
        .toList();

    return FeedV2ListResult(items: drafts, bookmark: nextBookmark);
  }

  Future<bool> deletePostV2(String feedPk) async {
    final base = Uri.parse(apiEndpoint);
    final uri = base.replace(
      pathSegments: [...base.pathSegments, 'v3', 'posts', feedPk],
    );

    final headers = <String, String>{'Content-Type': 'application/json'};
    final res = await delete(uri.toString(), headers: headers);

    return res.isOk;
  }

  Future<FeedV2ListResult> listFeedsV2({String? bookmark}) async {
    final base = Uri.parse(apiEndpoint).resolve('/v3/posts');
    final uri = bookmark == null
        ? base
        : base.replace(queryParameters: <String, String>{'bookmark': bookmark});

    final headers = <String, String>{'Content-Type': 'application/json'};
    final res = await get(uri.toString(), headers: headers);

    if (!res.isOk) {
      return const FeedV2ListResult(items: [], bookmark: null);
    }

    final body = res.body;
    final items = (body['items'] as List?) ?? const [];
    final nextBookmark = body['bookmark'] as String?;

    logger.d('feeds v2 items: $items');
    logger.d('feeds v2 bookmark: $nextBookmark');

    final feeds = items
        .map(
          (e) =>
              FeedV2SummaryModel.fromJson(Map<String, dynamic>.from(e as Map)),
        )
        .toList();

    return FeedV2ListResult(items: feeds, bookmark: nextBookmark);
  }

  Future<FeedV2Model> getFeedV2(String feedPk) async {
    final encodedPk = Uri.encodeComponent(feedPk);
    final uri = Uri.parse(apiEndpoint).resolve('/v3/posts/$encodedPk');

    final headers = <String, String>{'Content-Type': 'application/json'};
    final res = await get(uri.toString(), headers: headers);

    if (!res.isOk) {
      throw Exception('Failed to load feed detail: ${res.statusCode}');
    }

    final body = res.body;
    if (body is! Map<String, dynamic>) {
      throw Exception('Invalid feed detail response');
    }

    return FeedV2Model.fromJson(body);
  }

  Future<PostCommentListResult> listComments({
    required String postPk,
    required String commentSk,
    String? bookmark,
  }) async {
    final encodedPostPk = Uri.encodeComponent(postPk);
    final encodedCommentSk = Uri.encodeComponent(commentSk);

    final base = Uri.parse(
      apiEndpoint,
    ).resolve('/v3/posts/$encodedPostPk/comments/$encodedCommentSk');

    final uri = bookmark == null
        ? base
        : base.replace(queryParameters: <String, String>{'bookmark': bookmark});

    final headers = <String, String>{'Content-Type': 'application/json'};
    final res = await get(uri.toString(), headers: headers);

    if (!res.isOk) {
      return const PostCommentListResult(items: [], bookmark: null);
    }

    final body = res.body as Map<String, dynamic>;
    final items = (body['items'] as List?) ?? const [];
    final nextBookmark = body['bookmark'] as String?;

    final comments = items
        .map(
          (e) => PostCommentModel.fromJson(Map<String, dynamic>.from(e as Map)),
        )
        .toList();

    return PostCommentListResult(items: comments, bookmark: nextBookmark);
  }

  Future<PostCommentModel?> createComment({
    required String postPk,
    required String content,
  }) async {
    final encodedPostPk = Uri.encodeComponent(postPk);
    final uri = Uri.parse(
      apiEndpoint,
    ).resolve('/v3/posts/$encodedPostPk/comments');

    final headers = <String, String>{'Content-Type': 'application/json'};
    final body = {'content': content};

    final res = await post(uri.toString(), body, headers: headers);

    if (!res.isOk) {
      logger.e(
        'createComment failed status=${res.statusCode} body=${res.body}',
      );
      return null;
    }

    final json = res.body;
    if (json is! Map<String, dynamic>) {
      logger.e('createComment invalid body=${res.body}');
      return null;
    }

    return PostCommentModel.fromJson(json);
  }

  Future<PostCommentModel?> replyToComment({
    required String postPk,
    required String parentCommentSk,
    required String content,
  }) async {
    final encodedPostPk = Uri.encodeComponent(postPk);
    final encodedCommentSk = Uri.encodeComponent(parentCommentSk);

    final uri = Uri.parse(
      apiEndpoint,
    ).resolve('/v3/posts/$encodedPostPk/comments/$encodedCommentSk');

    final headers = <String, String>{'Content-Type': 'application/json'};
    final body = {'content': content};

    final res = await post(uri.toString(), body, headers: headers);

    if (!res.isOk) {
      logger.e(
        'replyToComment failed status=${res.statusCode} body=${res.body}',
      );
      return null;
    }

    final json = res.body;
    if (json is! Map<String, dynamic>) {
      logger.e('replyToComment invalid body=${res.body}');
      return null;
    }

    return PostCommentModel.fromJson(json);
  }

  Future<LikeCommentResponse?> likeComment({
    required String postPk,
    required String commentSk,
    required bool like,
  }) async {
    final encodedPostPk = Uri.encodeComponent(postPk);
    final encodedCommentSk = Uri.encodeComponent(commentSk);

    final uri = Uri.parse(
      apiEndpoint,
    ).resolve('/v3/posts/$encodedPostPk/comments/$encodedCommentSk/likes');

    final headers = <String, String>{'Content-Type': 'application/json'};
    final body = {'like': like};

    final res = await post(uri.toString(), body, headers: headers);

    if (!res.isOk) {
      logger.e('likeComment failed status=${res.statusCode} body=${res.body}');
      return null;
    }

    final json = res.body;
    if (json is! Map<String, dynamic>) {
      logger.e('likeComment invalid body=${res.body}');
      return null;
    }

    return LikeCommentResponse.fromJson(json);
  }

  Future<LikePostResponse?> likePost({
    required String postPk,
    required bool like,
  }) async {
    final encodedPostPk = Uri.encodeComponent(postPk);

    final uri = Uri.parse(
      apiEndpoint,
    ).resolve('/v3/posts/$encodedPostPk/likes');

    final headers = <String, String>{'Content-Type': 'application/json'};
    final body = {'like': like};

    final res = await post(uri.toString(), body, headers: headers);

    if (!res.isOk) {
      logger.e('likePost failed status=${res.statusCode} body=${res.body}');
      return null;
    }

    final json = res.body;
    if (json is! Map<String, dynamic>) {
      logger.e('likePost invalid body=${res.body}');
      return null;
    }

    return LikePostResponse.fromJson(Map<String, dynamic>.from(json));
  }
}
