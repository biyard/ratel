import 'package:ratel/exports.dart';

class UserApi extends GetConnect {
  final apiEndpoint = Config.apiEndpoint;

  UserApi() {
    httpClient.addRequestModifier<void>((req) async {
      final authApi = Get.find<AuthApi>();
      final cookie = await authApi.cookieHeaderAsync();
      logger.d('${req.method} ${req.url} | FeedsApi Cookie: $cookie');
      if (cookie?.isNotEmpty == true) req.headers['Cookie'] = cookie!;
      return req;
    });
  }

  Future<UserModel> getUserInfoV2() async {
    final uri = Uri.parse(apiEndpoint).resolve('/v3/me');

    final headers = <String, String>{'Content-Type': 'application/json'};
    final res = await get(uri.toString(), headers: headers);

    if (!res.isOk || res.body == null) {
      return UserModel(
        pk: '',
        email: '',
        nickname: '',
        profileUrl: '',
        description: '',
        userType: 0,
        username: '',
        followersCount: 0,
        followingsCount: 0,
        theme: 0,
        point: 0,
        referralCode: null,
        phoneNumber: null,
        principal: null,
        evmAddress: null,
        teams: const [],
      );
    }

    final item = res.body as Map<String, dynamic>;
    logger.d("user info v2: $item");

    return UserModel.fromJson(item);
  }

  Future<DidDocument?> getOrCreateDid() async {
    final uri = Uri.parse(apiEndpoint).resolve('/v3/me/did');

    final headers = <String, String>{'Content-Type': 'application/json'};
    final res = await get(uri.toString(), headers: headers);

    logger.d('getOrCreateDid: status=${res.statusCode}, body=${res.body}');

    if (!res.isOk || res.body == null) {
      return null;
    }

    final body = res.body;

    if (body is Map<String, dynamic>) {
      return DidDocument.fromJson(body);
    }

    if (body is Map) {
      return DidDocument.fromJson(Map<String, dynamic>.from(body));
    }

    logger.e('Unexpected getOrCreateDid response type: ${body.runtimeType}');
    return null;
  }

  Future<UserAttributes> getAttributes() async {
    final uri = Uri.parse(apiEndpoint).resolve('/v3/me/did/attributes');

    final headers = <String, String>{'Content-Type': 'application/json'};
    final res = await get(uri.toString(), headers: headers);

    logger.d('getAttributes: status=${res.statusCode}, body=${res.body}');

    if (!res.isOk || res.body == null) {
      return UserAttributes.empty;
    }

    final body = res.body;

    if (body is Map<String, dynamic>) {
      return UserAttributes.fromJson(body);
    }

    if (body is Map) {
      return UserAttributes.fromJson(Map<String, dynamic>.from(body));
    }

    logger.e('Unexpected getAttributes response type: ${body.runtimeType}');
    return UserAttributes.empty;
  }

  Future<UserAttributes> signAttributesWithCode(String code) async {
    final uri = Uri.parse(apiEndpoint).resolve('/v3/me/did');

    final headers = <String, String>{'Content-Type': 'application/json'};
    final body = {'type': 'code', 'code': code};

    final res = await put(uri.toString(), body, headers: headers);

    logger.d(
      'signAttributesWithCode: status=${res.statusCode}, body=${res.body}',
    );

    if (!res.isOk || res.body == null) {
      return UserAttributes.empty;
    }

    final data = res.body;

    if (data is Map<String, dynamic>) {
      return UserAttributes.fromJson(data);
    }

    if (data is Map) {
      return UserAttributes.fromJson(Map<String, dynamic>.from(data));
    }

    logger.e(
      'Unexpected signAttributesWithCode response type: ${data.runtimeType}',
    );
    return UserAttributes.empty;
  }
}
