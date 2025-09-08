import 'package:flutter_secure_storage/flutter_secure_storage.dart';
import 'package:ratel/exports.dart';

class SecureMedicalStore {
  static const _kBmi = 'medical_bmi';
  static const _kHeight = 'medical_height';
  static const _kWeight = 'medical_weight';

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

  Future<void> saveFromMedical(int userId, MedicalInfo info) async {
    await s.write(key: "$_kBmi $userId", value: info.bmi.toString());
    await s.write(key: "$_kHeight $userId", value: info.height.toString());
    await s.write(key: "$_kWeight $userId", value: info.weight.toString());
  }

  Future<({double? bmi, double? height, double? weight})> read(
    int userId,
  ) async {
    final bmi = await s.read(key: "$_kBmi $userId");
    final height = await s.read(key: "$_kHeight $userId");
    final weight = await s.read(key: "$_kWeight $userId");

    return (
      bmi: double.parse(bmi ?? "0"),
      height: double.parse(height ?? "0"),
      weight: double.parse(weight ?? "0"),
    );
  }
}
