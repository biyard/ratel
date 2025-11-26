import 'dart:convert';

import 'package:crypto/crypto.dart' as crypto;
import 'package:flutter_secure_storage/flutter_secure_storage.dart';
import 'package:cryptography/cryptography.dart' as cg;
import 'package:ratel/exports.dart';

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

  Future<String?> cookieHeaderAsync() => _buildCookieHeaderAsync();

  AuthApi() {
    httpClient.addRequestModifier<void>((req) async {
      final bypass = req.headers["_noCookieHeader"] == '1';
      req.headers.remove("_noCookieHeader");
      if (!bypass) {
        final cookie = await _buildCookieHeaderAsync();
        if (cookie?.isNotEmpty == true) {
          req.headers['Cookie'] = cookie!;
        }
      } else {
        req.headers.remove('Cookie');
      }
      return req;
    });
  }

  static const _noCookieHeader = '_noCookieHeader';

  Map<String, String> _noCookieJson({String? auth}) => {
    'Content-Type': 'application/json',
    if (auth != null) 'Authorization': auth,
    _noCookieHeader: '1',
  };

  Future<bool> _hasSession() async {
    await _lazyLoadJarIfEmpty();
    return (_cookieJar[_sidKeyStorage]?.isNotEmpty ?? false) ||
        (_cookieJar[_authKeyStorage]?.isNotEmpty ?? false);
  }

  Future<void> ensureLoggedOut() async {
    if (await _hasSession()) {
      try {
        await logout();
      } catch (_) {}
      await clearSession();
    }
  }

  Future<void> init() async {
    final sid = await _secure.read(key: _sidKeyStorage);
    final auth = await _secure.read(key: _authKeyStorage);
    if (sid?.isNotEmpty == true) _cookieJar[_sidKeyStorage] = sid!;
    if (auth?.isNotEmpty == true) _cookieJar[_authKeyStorage] = auth!;
  }

  Future<void> _lazyLoadJarIfEmpty() async {
    if (_cookieJar.isNotEmpty) return;
    final sid = await _secure.read(key: _sidKeyStorage);
    final auth = await _secure.read(key: _authKeyStorage);
    if (sid?.isNotEmpty == true) _cookieJar[_sidKeyStorage] = sid!;
    if (auth?.isNotEmpty == true) _cookieJar[_authKeyStorage] = auth!;
  }

  Future<String?> _buildCookieHeaderAsync() async {
    await _lazyLoadJarIfEmpty();
    return _buildCookieHeaderSync();
  }

  String? _buildCookieHeaderSync() {
    final sid = _cookieJar[_sidKeyStorage];
    final auth = _cookieJar[_authKeyStorage];
    final parts = <String>[];
    if (sid?.isNotEmpty == true) parts.add('$_sidKeyStorage=$sid');
    if (auth?.isNotEmpty == true) parts.add('$_authKeyStorage=$auth');
    return parts.isEmpty ? null : parts.join('; ');
  }

  static const _secure = FlutterSecureStorage(
    aOptions: AndroidOptions(encryptedSharedPreferences: true),
    iOptions: IOSOptions(accessibility: KeychainAccessibility.first_unlock),
  );

  String get _sidKeyStorage => '${env}_sid';
  String get _authKeyStorage => '${env}_auth_token';

  static final Map<String, String> _cookieJar = {};

  Future<bool> tryAutoSignIn() async {
    final saved = await AuthDb.read();
    if (saved == null) return false;
    final sid = saved['sid'] as String?;
    final auth = saved['auth_token'] as String?;
    if ((sid == null || sid.isEmpty) && (auth == null || auth.isEmpty)) {
      return false;
    }
    if (sid?.isNotEmpty == true) _cookieJar[_sidKeyStorage] = sid!;
    if (auth?.isNotEmpty == true) _cookieJar[_authKeyStorage] = auth!;
    if (sid?.isNotEmpty == true) {
      await _secure.write(key: _sidKeyStorage, value: sid);
    }
    if (auth?.isNotEmpty == true) {
      await _secure.write(key: _authKeyStorage, value: auth);
    }
    return true;
  }

  Future<dynamic> sendVerificationCode(String phone) async {
    final uri = Uri.parse(
      apiEndpoint,
    ).resolve('/v3/auth/verification/send-verification-code');
    final headers = <String, String>{'Content-Type': 'application/json'};
    final body = {'phone': phone};
    final res = await post(uri.toString(), body, headers: headers);
    if (!res.isOk) return null;
    return res.isOk;
  }

  Future<dynamic> verifyCode(String phone, String code) async {
    final uri = Uri.parse(
      apiEndpoint,
    ).resolve('/v3/auth/verification/verify-code');
    final headers = <String, String>{'Content-Type': 'application/json'};
    final body = {"phone": phone, "code": code};
    final res = await post(uri.toString(), body, headers: headers);
    if (!res.isOk) return null;
    return res.body;
  }

  Future<dynamic> logout() async {
    final uri = Uri.parse(apiEndpoint).resolve('/v2/users/logout');
    try {
      await post(
        uri.toString(),
        {},
        headers: {'Content-Type': 'application/json'},
      );
    } finally {
      await clearSession();
      await AuthDb.clear();
    }
  }

  Future<dynamic> socialSignup(
    String email,
    String displayName,
    String userName,
    String profileUrl,
    bool agree,
    String pkcs8B64,
  ) async {
    await ensureLoggedOut();
    final uri = Uri.parse(apiEndpoint)
        .resolve('/v1/users')
        .replace(queryParameters: <String, String>{'action': 'signup'});
    final authHeader = await _buildUserSigHeaderFromPkcs8(pkcs8B64);
    final body = {
      'signup': {
        'nickname': displayName,
        'email': email,
        'profile_url': profileUrl,
        'term_agreed': agree,
        'informed_agreed': false,
        'username': userName,
        'evm_address': '',
        'telegram_raw': '',
      },
    };
    final res = await post(
      uri.toString(),
      body,
      headers: _noCookieJson(auth: authHeader),
    );
    if (!res.isOk) return null;
    final loginRes = await socialLogin(email, pkcs8B64);
    return loginRes;
  }

  Future<dynamic> signup(
    String email,
    String password,
    String displayName,
    String userName,
    String profileUrl,
    bool agree,
  ) async {
    await ensureLoggedOut();
    final hashed = '0x${sha256Hex(password)}';
    final uri = Uri.parse(apiEndpoint)
        .resolve('/v1/users')
        .replace(queryParameters: <String, String>{'action': 'signup'});
    final kp = await cg.Ed25519().newKeyPair();
    final authHeader = await _buildUserSigHeader(kp);
    final body = {
      'email_signup': {
        'nickname': displayName,
        'email': email,
        'profile_url': profileUrl,
        'term_agreed': agree,
        'informed_agreed': false,
        'username': userName,
        'password': hashed,
        'telegram_raw': '',
      },
    };
    final res = await post(
      uri.toString(),
      body,
      headers: _noCookieJson(auth: authHeader),
    );
    if (!res.isOk) return null;
    final loginRes = await loginWithPassword(email, password);
    return loginRes;
  }

  Future<LoginResult?> socialLogin(String email, String pkcs8B64) async {
    await ensureLoggedOut();
    final uri = Uri.parse(apiEndpoint)
        .resolve('/v1/users')
        .replace(queryParameters: <String, String>{'action': 'login'});
    final authHeader = await _buildUserSigHeaderFromPkcs8(pkcs8B64);
    final res = await get(
      uri.toString(),
      headers: _noCookieJson(auth: authHeader),
    );
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
    await AuthDb.save(email, cookies[sidName], cookies[authName]);
    return LoginResult(
      body: res.body,
      sid: cookies[sidName],
      authToken: cookies[authName],
    );
  }

  Future<dynamic> loginWithPassword(String email, String password) async {
    await ensureLoggedOut();
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
    final kp = await cg.Ed25519().newKeyPair();
    final authHeader = await _buildUserSigHeader(kp);
    final res = await get(
      uri.toString(),
      headers: _noCookieJson(auth: authHeader),
    );
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
    await AuthDb.save(email, cookies[sidName], cookies[authName]);
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

  Future<void> clearSession() async {
    _cookieJar.clear();
    await _secure.delete(key: _sidKeyStorage);
    await _secure.delete(key: _authKeyStorage);
  }

  Future<String> _buildUserSigHeaderFromPkcs8(String pkcs8B64) async {
    final pair = await _pairFromPkcs8B64(pkcs8B64);
    return _buildUserSigHeader(pair);
  }

  Future<cg.SimpleKeyPair> _pairFromPkcs8B64(String b64) async {
    final der = base64Decode(b64);
    final seed = _extractSeed32FromPkcs8(der);
    return cg.Ed25519().newKeyPairFromSeed(seed);
  }

  Future<void> absorbSetCookieHeaders(Map<String, String> headers) async {
    final cookies = _extractCookies(headers);
    if (cookies.isEmpty) return;

    final sidName = _sidKeyStorage;
    final authName = _authKeyStorage;

    if (cookies[sidName] != null) {
      _cookieJar[sidName] = cookies[sidName]!;
      await _secure.write(key: sidName, value: cookies[sidName]!);
    }
    if (cookies[authName] != null) {
      _cookieJar[authName] = cookies[authName]!;
      await _secure.write(key: authName, value: cookies[authName]!);
    }
  }

  Uint8List _extractSeed32FromPkcs8(Uint8List der) {
    for (int i = 0; i + 34 <= der.length; i++) {
      if (der[i] == 0x04 && der[i + 1] == 0x20) {
        return Uint8List.fromList(der.sublist(i + 2, i + 34));
      }
    }
    throw ArgumentError('Ed25519 seed not found in PKCS#8');
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
