import 'dart:convert';

import 'package:crypto/crypto.dart' as crypto;
import 'package:flutter_secure_storage/flutter_secure_storage.dart';
import 'package:ratel/exports.dart';
import 'package:cryptography/cryptography.dart' as cg;

class LoginResult {
  final dynamic body;
  final String? sid;
  final String? authToken;
  LoginResult({this.body, this.sid, this.authToken});
}

class AuthApi extends GetConnect {
  final apiEndpoint = Config.apiEndpoint;
  final signDomain = Config.signDomain;
  final env = Config.env;

  static const _secure = FlutterSecureStorage(
    aOptions: AndroidOptions(encryptedSharedPreferences: true),
    iOptions: IOSOptions(accessibility: KeychainAccessibility.first_unlock),
  );

  String get _sidKeyStorage => '${env}_sid';
  String get _authKeyStorage => '${env}_auth_token';

  final Map<String, String> _cookieJar = {};

  Future<dynamic> sendVerificationCode(String email) async {
    final uri = Uri.parse(apiEndpoint).resolve('/v1/users/verifications');

    final headers = <String, String>{'Content-Type': 'application/json'};
    final body = {
      'send_verification_code': {'email': email},
    };

    final res = await post(uri.toString(), body, headers: headers);

    if (!res.isOk) return null;

    logger.d('response body: ${res.body}');

    return res.isOk;
  }

  Future<dynamic> verifyCode(String email, String value) async {
    final uri = Uri.parse(apiEndpoint).resolve('/v1/users/verifications');

    final headers = <String, String>{'Content-Type': 'application/json'};
    final body = {
      'verify': {'email': email, 'value': value},
    };

    final res = await post(uri.toString(), body, headers: headers);

    if (!res.isOk) return null;

    logger.d('response body: ${res.body}');

    return res.body;
  }

  Future<dynamic> loginWithPassword(String email, String password) async {
    final hashed = '0x${sha256Hex(password)}';

    final uri = Uri.parse(apiEndpoint)
        .resolve('/v1/users')
        .replace(
          queryParameters: <String, String>{
            'action': 'login-by-password',
            'email': email,
            'password': hashed,
          },
        );

    logger.d("login url: $uri");

    final kp = await cg.Ed25519().newKeyPair();
    final authHeader = await _buildUserSigHeader(kp);

    logger.d("login header: $authHeader");

    final res = await get(
      uri.toString(),
      headers: {
        'Content-Type': 'application/json',
        'Authorization': authHeader,
      },
    );

    logger.d("response body: ${res.body}");
    if (!res.isOk) return null;

    final cookies = _extractCookies(res.headers ?? {});
    final sidName = _sidKeyStorage;
    final authName = _authKeyStorage;

    if (cookies[sidName] != null) _cookieJar[sidName] = cookies[sidName]!;
    if (cookies[authName] != null) _cookieJar[authName] = cookies[authName]!;

    if (cookies[sidName] != null) {
      await _secure.write(key: sidName, value: cookies[sidName]!);
    }
    if (cookies[authName] != null) {
      await _secure.write(key: authName, value: cookies[authName]!);
    }

    logger.d('cookie jar updated: $_cookieJar');

    logger.d("cookie: ${cookies}");

    return LoginResult(
      body: res.body,
      sid: cookies[sidName],
      authToken: cookies[authName],
    );
  }

  Future<String> _buildUserSigHeader(cg.KeyPair keyPair) async {
    final ed25519 = cg.Ed25519();
    final simple = await keyPair.extract() as cg.SimpleKeyPairData;
    final pkBytes = simple.publicKey.bytes;

    final timestamp = (DateTime.now().millisecondsSinceEpoch / 1000).floor();
    final msg = '$signDomain-$timestamp';
    final msgBytes = utf8.encode(msg);

    final sig = await ed25519.sign(msgBytes, keyPair: keyPair);
    final token =
        '$timestamp:eddsa:${base64Encode(pkBytes)}:${base64Encode(sig.bytes)}';

    return 'UserSig $token';
  }

  Map<String, String> _extractCookies(Map<String, String> headers) {
    final setCookie =
        headers['set-cookie'] ?? headers['Set-Cookie'] ?? headers['SET-COOKIE'];

    logger.d("cookie: ${setCookie}");
    if (setCookie == null || setCookie.isEmpty) return {};

    final sidName = '${env}_sid';
    final authName = '${env}_auth_token';

    String? pick(String name) {
      final m = RegExp('$name=([^;]+)').firstMatch(setCookie);
      return m?.group(1);
    }

    final out = <String, String>{};
    final sid = pick(sidName);
    final auth = pick(authName);
    if (sid != null) out[sidName] = sid;
    if (auth != null) out[authName] = auth;
    return out;
  }
}

String _toHex(Uint8List bytes, {bool with0x = false}) {
  final sb = StringBuffer(with0x ? '0x' : '');
  for (final b in bytes) {
    sb.write(b.toRadixString(16).padLeft(2, '0'));
  }
  return sb.toString();
}

String sha256Hex(String input, {bool with0x = false}) {
  final bytes = utf8.encode(input);
  final digest = crypto.sha256.convert(bytes).bytes;
  return _toHex(Uint8List.fromList(digest), with0x: with0x);
}
