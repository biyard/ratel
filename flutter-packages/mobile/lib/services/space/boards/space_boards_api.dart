import 'package:ratel/exports.dart';

class SpaceBoardsApi extends GetConnect {
  final apiEndpoint = Config.apiEndpoint;

  SpaceBoardsApi() {
    httpClient.baseUrl = apiEndpoint;
    httpClient.timeout = const Duration(seconds: 10);

    httpClient.addRequestModifier<void>((req) async {
      final authApi = Get.find<AuthApi>();
      final cookie = await authApi.cookieHeaderAsync();
      logger.d('${req.method} ${req.url} | SpaceboardsApi Cookie: $cookie');
      if (cookie?.isNotEmpty == true) {
        req.headers['Cookie'] = cookie!;
      }
      return req;
    });
  }

  Future<List<String>> listCategories(String spacePk) async {
    final encodedSpacePk = Uri.encodeComponent(spacePk);
    final resp = await get<List<dynamic>>(
      '/v3/spaces/$encodedSpacePk/boards/categories',
    );

    if (!resp.isOk || resp.body == null) {
      throw Exception(
        'Failed to list categories for $spacePk: '
        '${resp.statusCode} ${resp.statusText}',
      );
    }

    return resp.body!.map((e) => e.toString()).toList();
  }

  Future<SpacePostListResult> listPosts(
    String spacePk, {
    String? bookmark,
    String? category,
  }) async {
    final encodedSpacePk = Uri.encodeComponent(spacePk);

    final query = <String, String>{};
    if (bookmark != null && bookmark.isNotEmpty) {
      query['bookmark'] = bookmark;
    }
    if (category != null && category.isNotEmpty) {
      query['category'] = category;
    }

    final resp = await get<Map<String, dynamic>>(
      '/v3/spaces/$encodedSpacePk/boards',
      query: query.isEmpty ? null : query,
    );

    if (!resp.isOk || resp.body == null) {
      throw Exception(
        'Failed to list posts for $spacePk: '
        '${resp.statusCode} ${resp.statusText}',
      );
    }

    return SpacePostListResult.fromJson(resp.body!);
  }

  Future<SpacePostModel> getPost(String spacePk, String postPk) async {
    final encodedSpacePk = Uri.encodeComponent(spacePk);
    final encodedPostPk = Uri.encodeComponent(postPk);

    final resp = await get<Map<String, dynamic>>(
      '/v3/spaces/$encodedSpacePk/boards/$encodedPostPk',
    );

    logger.d(
      'GET /v3/spaces/$encodedSpacePk/boards/$encodedPostPk '
      'status=${resp.statusCode}, body=${resp.body}',
    );

    if (!resp.isOk || resp.body == null) {
      throw Exception(
        'Failed to get post $postPk in $spacePk: '
        '${resp.statusCode} ${resp.statusText}',
      );
    }

    return SpacePostModel.fromJson(resp.body!);
  }

  Future<SpacePostCommentListResult> listComments(
    String spacePk,
    String postPk, {
    String? bookmark,
  }) async {
    final encodedSpacePk = Uri.encodeComponent(spacePk);
    final encodedPostPk = Uri.encodeComponent(postPk);

    final query = <String, String>{};
    if (bookmark != null && bookmark.isNotEmpty) {
      query['bookmark'] = bookmark;
    }

    final resp = await get<Map<String, dynamic>>(
      '/v3/spaces/$encodedSpacePk/boards/$encodedPostPk/comments',
      query: query.isEmpty ? null : query,
    );

    if (!resp.isOk || resp.body == null) {
      throw Exception(
        'Failed to list comments for post $postPk in $spacePk: '
        '${resp.statusCode} ${resp.statusText}',
      );
    }

    return SpacePostCommentListResult.fromJson(resp.body!);
  }

  Future<SpacePostCommentModel> addComment(
    String spacePk,
    String postPk,
    String content,
  ) async {
    final encodedSpacePk = Uri.encodeComponent(spacePk);
    final encodedPostPk = Uri.encodeComponent(postPk);

    final resp = await post<Map<String, dynamic>>(
      '/v3/spaces/$encodedSpacePk/boards/$encodedPostPk/comments',
      {'content': content},
    );

    if (!resp.isOk || resp.body == null) {
      throw Exception(
        'Failed to add comment to post $postPk in $spacePk: '
        '${resp.statusCode} ${resp.statusText}',
      );
    }

    return SpacePostCommentModel.fromJson(resp.body!);
  }

  Future<bool> likeComment(
    String spacePk,
    String postPk,
    String commentSk, {
    required bool like,
  }) async {
    final encodedSpacePk = Uri.encodeComponent(spacePk);
    final encodedPostPk = Uri.encodeComponent(postPk);
    final encodedCommentSk = Uri.encodeComponent(commentSk);

    final resp = await post<Map<String, dynamic>>(
      '/v3/spaces/$encodedSpacePk/boards/$encodedPostPk/comments/$encodedCommentSk/likes',
      {'like': like},
    );

    if (!resp.isOk || resp.body == null) {
      throw Exception(
        'Failed to like/unlike comment $commentSk in post $postPk: '
        '${resp.statusCode} ${resp.statusText}',
      );
    }

    final body = resp.body!;
    if (body.containsKey('liked')) {
      return body['liked'] as bool;
    }
    return like;
  }

  Future<void> deleteComment(
    String spacePk,
    String postPk,
    String commentSk,
  ) async {
    final encodedSpacePk = Uri.encodeComponent(spacePk);
    final encodedPostPk = Uri.encodeComponent(postPk);
    final encodedCommentSk = Uri.encodeComponent(commentSk);

    final resp = await delete(
      '/v3/spaces/$encodedSpacePk/boards/$encodedPostPk/comments/$encodedCommentSk',
    );

    if (!resp.isOk) {
      throw Exception(
        'Failed to delete comment $commentSk in post $postPk: '
        '${resp.statusCode} ${resp.statusText}',
      );
    }
  }

  Future<void> updateComment(
    String spacePk,
    String postPk,
    String commentSk,
    String content,
  ) async {
    final encodedSpacePk = Uri.encodeComponent(spacePk);
    final encodedPostPk = Uri.encodeComponent(postPk);
    final encodedCommentSk = Uri.encodeComponent(commentSk);

    final resp = await patch(
      '/v3/spaces/$encodedSpacePk/boards/$encodedPostPk/comments/$encodedCommentSk',
      {'content': content},
    );

    if (!resp.isOk) {
      throw Exception(
        'Failed to update comment $commentSk in post $postPk: '
        '${resp.statusCode} ${resp.statusText}',
      );
    }
  }
}
