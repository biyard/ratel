import 'dart:convert';

import 'package:http/http.dart' as http;
import 'package:ratel/exports.dart';

class AssetApi extends GetConnect {
  final apiEndpoint = Config.apiEndpoint;

  AssetApi() {
    httpClient.addRequestModifier<void>((req) async {
      final authApi = Get.find<AuthApi>();
      final cookie = await authApi.cookieHeaderAsync();
      logger.d('${req.method} ${req.url} | FeedsApi Cookie: $cookie');
      if (cookie?.isNotEmpty == true) req.headers['Cookie'] = cookie!;
      return req;
    });
  }

  Future<AssetPresignedUris> getPresignedUrl(
    FileType fileType, {
    int totalCount = 1,
  }) async {
    final uri = Uri.parse(apiEndpoint)
        .resolve('/v1/assets')
        .replace(
          queryParameters: {
            'action': 'get-presigned-uris',
            'file_type': fileType.value,
            'total_count': '$totalCount',
          },
        );

    final res = await get(
      uri.toString(),
      headers: {'Content-Type': 'application/json'},
    );

    if (!res.isOk || res.body == null) {
      throw Exception('Failed to get presigned url: ${res.statusCode}');
    }
    return AssetPresignedUris.fromJson(res.body);
  }
}
