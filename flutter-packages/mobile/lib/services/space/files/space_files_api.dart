import 'package:ratel/exports.dart';

class SpaceFilesApi extends GetConnect {
  final apiEndpoint = Config.apiEndpoint;

  SpaceFilesApi() {
    httpClient.baseUrl = apiEndpoint;
    httpClient.timeout = const Duration(seconds: 10);

    httpClient.addRequestModifier<void>((req) async {
      final authApi = Get.find<AuthApi>();
      final cookie = await authApi.cookieHeaderAsync();
      logger.d('${req.method} ${req.url} | SpaceFilesApi Cookie: $cookie');
      if (cookie?.isNotEmpty == true) {
        req.headers['Cookie'] = cookie!;
      }
      return req;
    });
  }

  Future<List<FileModel>> listSpaceFiles(String spacePk) async {
    final encodedPk = Uri.encodeComponent(spacePk);
    final url = '/v3/spaces/$encodedPk/files';

    logger.d('GET $url | listSpaceFiles');

    final response = await get<Map<String, dynamic>>(url);

    if (!response.isOk || response.body == null) {
      logger.e(
        'Failed to load space files: ${response.statusCode} ${response.statusText}',
      );
      return [];
    }

    final body = response.body!;
    final rawFiles = body['files'] as List<dynamic>? ?? const [];

    final files = rawFiles
        .whereType<Map<String, dynamic>>()
        .map<FileModel>((json) => FileModel.fromJson(json))
        .toList();

    logger.d('Loaded ${files.length} files for spacePk=$spacePk');

    return files;
  }
}
