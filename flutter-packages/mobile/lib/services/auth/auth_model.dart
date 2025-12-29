import 'package:sqflite/sqflite.dart';
import 'package:path/path.dart' as p;

class AuthDb {
  static Database? _db;

  static Future<Database> open() async {
    if (_db != null) return _db!;
    final path = p.join(await getDatabasesPath(), 'auth.db');
    _db = await openDatabase(
      path,
      version: 1,
      onCreate: (db, v) async {
        await db.execute('''
          CREATE TABLE IF NOT EXISTS session (
            id INTEGER PRIMARY KEY,
            email TEXT NOT NULL,
            sid TEXT,
            auth_token TEXT,
            updated_at INTEGER NOT NULL
          )
        ''');
      },
    );
    return _db!;
  }

  static Future<void> save(String email, String? sid, String? authToken) async {
    final db = await open();
    await db.insert('session', {
      'id': 1,
      'email': email,
      'sid': sid,
      'auth_token': authToken,
      'updated_at': DateTime.now().millisecondsSinceEpoch ~/ 1000,
    }, conflictAlgorithm: ConflictAlgorithm.replace);
  }

  static Future<Map<String, Object?>?> read() async {
    final db = await open();
    final rows = await db.query('session', limit: 1);
    return rows.isEmpty ? null : rows.first;
  }

  static Future<void> clear() async {
    final db = await open();
    await db.delete('session');
  }
}

class AccountItemModel {
  final String userPk;
  final String displayName;
  final String profileUrl;
  final String username;
  final int userType;
  final int lastLoginAt;
  final bool revoked;

  const AccountItemModel({
    required this.userPk,
    required this.displayName,
    required this.profileUrl,
    required this.username,
    required this.userType,
    required this.lastLoginAt,
    required this.revoked,
  });

  static String _asString(dynamic v, {String fallback = ''}) {
    if (v == null) return fallback;
    return v.toString();
  }

  static int _asInt(dynamic v, {int fallback = 0}) {
    if (v == null) return fallback;
    if (v is int) return v;
    if (v is num) return v.toInt();
    return int.tryParse(v.toString()) ?? fallback;
  }

  static bool _asBool(dynamic v, {bool fallback = false}) {
    if (v == null) return fallback;
    if (v is bool) return v;
    if (v is num) return v != 0;
    final s = v.toString().toLowerCase();
    if (s == 'true') return true;
    if (s == 'false') return false;
    return fallback;
  }

  factory AccountItemModel.fromJson(Map<String, dynamic> json) {
    return AccountItemModel(
      userPk: _asString(json['user_pk']),
      displayName: _asString(json['display_name']),
      profileUrl: _asString(json['profile_url']),
      username: _asString(json['username']),
      userType: _asInt(json['user_type']),
      lastLoginAt: _asInt(json['last_login_at']),
      revoked: _asBool(json['revoked']),
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'user_pk': userPk,
      'display_name': displayName,
      'profile_url': profileUrl,
      'username': username,
      'user_type': userType,
      'last_login_at': lastLoginAt,
      'revoked': revoked,
    };
  }
}

class ListAccountsResult {
  final List<AccountItemModel> items;
  final String? bookmark;

  const ListAccountsResult({required this.items, this.bookmark});

  static String? _asOptString(dynamic v) {
    if (v == null) return null;
    return v.toString();
  }

  factory ListAccountsResult.fromJson(Map<String, dynamic> json) {
    final list =
        (json['items'] ?? json['accounts']) as List<dynamic>? ?? const [];
    return ListAccountsResult(
      items: list
          .map(
            (e) =>
                AccountItemModel.fromJson((e as Map).cast<String, dynamic>()),
          )
          .toList(),
      bookmark: _asOptString(json['bookmark']),
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'items': items.map((e) => e.toJson()).toList(),
      'bookmark': bookmark,
    };
  }
}

class ChangeAccountRequest {
  final String userPk;
  final String deviceId;
  final String refreshToken;

  const ChangeAccountRequest({
    required this.userPk,
    required this.deviceId,
    required this.refreshToken,
  });

  Map<String, dynamic> toJson() {
    return {
      'user_pk': userPk,
      'device_id': deviceId,
      'refresh_token': refreshToken,
    };
  }
}

class ChangeAccountResponse {
  final String? refreshToken;

  const ChangeAccountResponse({this.refreshToken});

  static String? _asOptString(dynamic v) {
    if (v == null) return null;
    return v.toString();
  }

  factory ChangeAccountResponse.fromJson(Map<String, dynamic> json) {
    return ChangeAccountResponse(
      refreshToken: _asOptString(json['refresh_token']),
    );
  }

  Map<String, dynamic> toJson() {
    return {'refresh_token': refreshToken};
  }
}
