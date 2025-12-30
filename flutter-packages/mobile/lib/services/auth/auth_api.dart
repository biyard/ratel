import 'dart:convert';

import 'package:crypto/crypto.dart' as crypto;
import 'package:flutter_secure_storage/flutter_secure_storage.dart';
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
  String _rtKey(String userPk) => '${env}_rt_$userPk';

  static final Map<String, String> _cookieJar = {};

  Future<bool> tryAutoSignIn() async {
    final saved = await AuthDb.read();
    if (saved == null) return false;
    final sid = saved['sid'] as String?;
    final auth = saved['auth_token'] as String?;
    if ((sid == null || sid.isEmpty)) {
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

    try {
      await NotificationsService.to.registerForCurrentUserIfPossible();
    } catch (e) {
      logger.w(
        'AuthApi.tryAutoSignIn: failed to register notification device: $e',
      );
    }

    final userService = Get.find<UserService>();
    await userService.getUser();
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

  Future<dynamic> signup(String phone, String code) async {
    final notificationService = Get.find<NotificationsService>();
    final deviceId = await notificationService.getOrCreateDeviceId();
    final uri = Uri.parse(apiEndpoint).resolve('/v3/auth/signup');
    final body = {
      "phone": phone,
      "code": code,
      "display_name": "signup",
      "username": "signup",
      "profile_url": "",
      "description": "",
      "term_agreed": true,
      "informed_agreed": false,
      "device_id": deviceId,
    };
    final res = await post(
      uri.toString(),
      body,
      headers: {'Content-Type': 'application/json'},
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
    await AuthDb.save(phone, cookies[sidName], cookies[authName]);

    try {
      await NotificationsService.to.registerForCurrentUserIfPossible();
    } catch (e) {
      logger.w('AuthApi.signup: failed to register notification device: $e');
    }

    final userService = Get.find<UserService>();
    final user = await userService.getUser();

    final map = (res.body is Map) ? (res.body as Map) : null;
    final refreshToken = map?['refresh_token'] as String?;
    if (refreshToken != null) {
      await _secure.write(key: _rtKey(user.pk), value: refreshToken);
    }

    return LoginResult(
      body: res.body,
      sid: cookies[sidName],
      authToken: cookies[authName],
    );
  }

  Future<dynamic> logout() async {
    final uri = Uri.parse(apiEndpoint).resolve('/v3/auth/logout');
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

  Future<dynamic> loginWithPassword(String email, String password) async {
    await ensureLoggedOut();
    final notificationService = Get.find<NotificationsService>();
    final deviceId = await notificationService.getOrCreateDeviceId();
    final hashed = '0x${sha256Hex(password)}';
    final uri = Uri.parse(apiEndpoint).resolve('/v3/auth/login');
    final body = {"email": email, "password": hashed, "device_id": deviceId};

    final res = await post(uri.toString(), body, headers: _noCookieJson());
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

    try {
      await NotificationsService.to.registerForCurrentUserIfPossible();
    } catch (e) {
      logger.w(
        'AuthApi.loginWithPassword: failed to register notification device: $e',
      );
    }

    final userService = Get.find<UserService>();
    final user = await userService.getUser();

    final map = (res.body is Map) ? (res.body as Map) : null;
    logger.d("login response body: $map");
    final refreshToken = map?['refresh_token'] as String?;
    if (refreshToken != null) {
      await _secure.write(key: _rtKey(user.pk), value: refreshToken);
    }

    return LoginResult(
      body: res.body,
      sid: cookies[sidName],
      authToken: cookies[authName],
    );
  }

  Future<ListAccountsResult?> listAccounts({String? bookmark}) async {
    final notificationService = Get.find<NotificationsService>();
    final deviceId = await notificationService.getOrCreateDeviceId();

    final uri = Uri.parse(apiEndpoint).resolve('/v3/auth/accounts');
    final qp = <String, String>{
      'device_id': deviceId,
      if (bookmark != null && bookmark.isNotEmpty) 'bookmark': bookmark,
    };

    final res = await get(
      uri.replace(queryParameters: qp).toString(),
      headers: _noCookieJson(),
    );
    if (!res.isOk) return null;

    final map = (res.body is Map)
        ? (res.body as Map).cast<String, dynamic>()
        : null;
    if (map == null) return null;

    return ListAccountsResult.fromJson(map);
  }

  Future<ChangeAccountResponse?> changeAccount(String userPk) async {
    final notificationService = Get.find<NotificationsService>();
    final deviceId = await notificationService.getOrCreateDeviceId();

    final storedRt = await _secure.read(key: _rtKey(userPk));
    if (storedRt == null || storedRt.isEmpty) {
      throw Exception('Missing refresh token for userPk=$userPk');
    }

    final uri = Uri.parse(apiEndpoint).resolve('/v3/auth/change-account');
    final req = ChangeAccountRequest(
      userPk: userPk,
      deviceId: deviceId,
      refreshToken: storedRt,
    );

    final res = await post(
      uri.toString(),
      req.toJson(),
      headers: _noCookieJson(),
    );
    if (!res.isOk) return null;

    await absorbSetCookieHeaders(res.headers ?? {});

    final map = (res.body is Map)
        ? (res.body as Map).cast<String, dynamic>()
        : null;
    final resp = map == null
        ? const ChangeAccountResponse()
        : ChangeAccountResponse.fromJson(map);

    final newRt = resp.refreshToken;
    if (newRt != null && newRt.isNotEmpty) {
      await _secure.write(key: _rtKey(userPk), value: newRt);
    }

    final sid = _cookieJar[_sidKeyStorage];
    final auth = _cookieJar[_authKeyStorage];
    await AuthDb.save(userPk, sid, auth);

    try {
      await NotificationsService.to.registerForCurrentUserIfPossible();
    } catch (e) {
      logger.w('AuthApi.changeAccount: register notification failed: $e');
    }

    final userService = Get.find<UserService>();
    await userService.getUser();

    return resp;
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
