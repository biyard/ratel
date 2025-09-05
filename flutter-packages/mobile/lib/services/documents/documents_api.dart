import 'dart:convert';

import 'package:http/http.dart' as http;
import 'package:ratel/exports.dart';

class DocumentsApi extends GetConnect {
  final apiEndpoint = Config.apiEndpoint;

  DocumentsApi() {
    httpClient.addRequestModifier<void>((req) async {
      final authApi = Get.find<AuthApi>();
      final cookie = await authApi.cookieHeaderAsync();
      logger.d('${req.method} ${req.url} | DocumentsApi Cookie: $cookie');
      if (cookie?.isNotEmpty == true) req.headers['Cookie'] = cookie!;
      return req;
    });
  }

  Future<({String url, String key})> getPresigned({int total = 1}) async {
    final uri = Uri.parse(apiEndpoint)
        .resolve('/v2/documents')
        .replace(queryParameters: {'total_size': '$total'});

    final res = await get(uri.toString());

    if (!res.isOk) {
      final msg = res.bodyString?.isNotEmpty == true
          ? res.bodyString
          : 'status=${res.statusCode}';
      throw Exception('presign failed: $msg');
    }

    final Map<String, dynamic> data = (res.body is Map)
        ? (res.body as Map).cast<String, dynamic>()
        : jsonDecode(res.bodyString ?? '{}') as Map<String, dynamic>;

    final list = (data['presigned_uris'] as List?) ?? const [];
    if (list.isEmpty) {
      throw Exception('presign response empty: $data');
    }

    final first = (list.first as Map).cast<String, dynamic>();
    final url = first['presigned_uri'] as String?;
    final key = first['key'] as String?;

    if (url == null || key == null) {
      throw Exception('presign item missing url/key: $first');
    }

    logger.d('presign response: url=$url key=$key');
    return (url: url, key: key);
  }

  Future<void> putToS3(String url, Uint8List bytes) async {
    final put = await http.put(
      Uri.parse(url),
      headers: {'Content-Type': 'image/jpeg'},
      body: bytes,
    );
    if (put.statusCode < 200 || put.statusCode >= 300) {
      throw Exception('S3 PUT ${put.statusCode}: ${put.body}');
    }
  }

  Future<PassportInfo> submitKey(String key) => uploadPassportKey(key);

  Future<PassportInfo> uploadPassportKey(String key) async {
    final uri = Uri.parse(apiEndpoint).resolve('/v2/documents/passport');
    final body = {'key': key};

    final res = await post(uri.toString(), body);
    if (!res.isOk) {
      logger.e('uploadPassportKey failed: ${res.statusCode} ${res.bodyString}');
      throw Exception('Passport key submit failed (${res.statusCode}).');
    }

    final Map<String, dynamic> data = (res.body is Map)
        ? (res.body as Map).cast<String, dynamic>()
        : jsonDecode(res.bodyString ?? '{}') as Map<String, dynamic>;
    logger.d('uploadPassportKey response: $data');
    return PassportInfo.fromJson(data);
  }

  Future<MedicalInfo> uploadMedicalKeys(List<String> keys) async {
    final uri = Uri.parse(
      apiEndpoint,
    ).resolve('/v2/verifiable-credentials/medical');
    final body = {'document_keys': keys};

    final res = await post(uri.toString(), body);
    if (!res.isOk) {
      logger.e('uploadMedicalKeys failed: ${res.statusCode} ${res.bodyString}');
      throw Exception('Medical key submit failed (${res.statusCode}).');
    }

    final Map<String, dynamic> data = (res.body is Map)
        ? (res.body as Map).cast<String, dynamic>()
        : jsonDecode(res.bodyString ?? '{}') as Map<String, dynamic>;

    logger.d('uploadMedicalKeys response: $data');
    return MedicalInfo.fromJson(data);
  }
}
