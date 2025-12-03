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
}
