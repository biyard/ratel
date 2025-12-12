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
          (e) => FeedSummaryModel.fromJson(Map<String, dynamic>.from(e as Map)),
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
          (e) => FeedSummaryModel.fromJson(Map<String, dynamic>.from(e as Map)),
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
          (e) => FeedSummaryModel.fromJson(Map<String, dynamic>.from(e as Map)),
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
