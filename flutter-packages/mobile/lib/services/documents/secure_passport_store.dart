import 'package:flutter_secure_storage/flutter_secure_storage.dart';
import 'package:intl/intl.dart';
import 'package:ratel/exports.dart';

class SecurePassportStore {
  static const _kBirth = 'passport_birth_date';
  static const _kCountry = 'passport_country';
  static const _kGender = 'passport_gender';

  static const aOpts = AndroidOptions(
    encryptedSharedPreferences: true,
    resetOnError: true,
  );

  static const iOpts = IOSOptions(
    accessibility: KeychainAccessibility.first_unlock,
  );

  final FlutterSecureStorage s = const FlutterSecureStorage(
    aOptions: aOpts,
    iOptions: iOpts,
  );

  Future<void> saveFromPassport(int userId, PassportInfo info) async {
    final birthIso = DateFormat(
      'yyyy-MM-dd',
    ).format(DateTime.fromMillisecondsSinceEpoch(info.birthDate * 1000));
    final country = info.nationality;
    final gender = info.gender;

    await s.write(key: "$_kBirth $userId", value: birthIso);
    await s.write(key: "$_kCountry $userId", value: country);
    await s.write(key: "$_kGender $userId", value: gender);
  }

  Future<({String? birth, String? country, String? gender})> read(
    int userId,
  ) async {
    final birth = await s.read(key: "$_kBirth $userId");
    final country = await s.read(key: "$_kCountry $userId");
    final gender = await s.read(key: "$_kGender $userId");

    logger.d("Read from secure store: birth=$birth, country=$country");
    return (birth: birth, country: country, gender: gender);
  }

  Future<void> clear(int userId) async {
    await Future.wait([
      s.delete(key: "$_kBirth $userId"),
      s.delete(key: "$_kCountry $userId"),
      s.delete(key: "$_kGender $userId"),
    ]);
  }
}
