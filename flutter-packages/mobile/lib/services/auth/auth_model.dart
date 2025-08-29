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
