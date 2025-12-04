import 'package:ratel/exports.dart';

class SpaceApi extends GetConnect {
  final apiEndpoint = Config.apiEndpoint;

  SpaceApi() {
    httpClient.addRequestModifier<void>((req) async {
      final authApi = Get.find<AuthApi>();
      final cookie = await authApi.cookieHeaderAsync();
      logger.d('${req.method} ${req.url} | FeedsApi Cookie: $cookie');
      if (cookie?.isNotEmpty == true) req.headers['Cookie'] = cookie!;
      return req;
    });
  }

  Future<MySpaces> getMySpaces({String? bookmark}) async {
    final base = Uri.parse(apiEndpoint).resolve('/v3/me/spaces');

    final uri = (bookmark == null || bookmark.isEmpty)
        ? base
        : base.replace(queryParameters: {'bookmark': bookmark});

    final headers = <String, String>{'Content-Type': 'application/json'};
    final res = await get(uri.toString(), headers: headers);

    logger.d('getMySpaces res: status=${res.statusCode}, body=${res.body}');

    if (!res.isOk || res.body == null) {
      return const MySpaces(items: [], bookmark: null);
    }

    try {
      final body = res.body;

      if (body is Map<String, dynamic>) {
        return MySpaces.fromJson(body);
      }

      if (body is Map) {
        return MySpaces.fromJson(Map<String, dynamic>.from(body));
      }

      logger.e('Unexpected getMySpaces response type: ${body.runtimeType}');
      return const MySpaces(items: [], bookmark: null);
    } catch (e, s) {
      logger.e('Failed to parse MySpacesV3: $e\n$s');
      return const MySpaces(items: [], bookmark: null);
    }
  }

  Future<SpaceModel?> getSpace(String spacePk) async {
    final encodedPk = Uri.encodeComponent(spacePk);
    final uri = Uri.parse(apiEndpoint).resolve('/v3/spaces/$encodedPk');

    final headers = <String, String>{'Content-Type': 'application/json'};
    final res = await get(uri.toString(), headers: headers);

    logger.d(
      'getSpace($spacePk) res: status=${res.statusCode}, body=${res.body}',
    );

    if (!res.isOk || res.body == null) {
      return null;
    }

    try {
      final body = res.body;

      if (body is Map<String, dynamic>) {
        return SpaceModel.fromJson(body);
      }

      if (body is Map) {
        return SpaceModel.fromJson(Map<String, dynamic>.from(body));
      }

      logger.e('Unexpected getSpace response type: ${body.runtimeType}');
      return null;
    } catch (e, s) {
      logger.e('Failed to parse SpaceModel from getSpace: $e\n$s');
      return null;
    }
  }
}
