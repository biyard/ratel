import 'dart:convert';

import 'package:cryptography/cryptography.dart';
import 'package:flutter_secure_storage/flutter_secure_storage.dart';
import 'package:ratel/exports.dart';

class AnonymousService extends GetxService {
  static const _privateKeyKey = 'anon_private_key';
  static const _publicKeyKey = 'anon_public_key';
  final FlutterSecureStorage _storage = const FlutterSecureStorage();

  SimpleKeyPair? _keyPair;

  static void init() {
    Get.put<AnonymousService>(AnonymousService());
  }

  Future<SimpleKeyPair> getKeyPair() async {
    if (_keyPair != null) return _keyPair!;

    final privateKeyStr = await _storage.read(key: _privateKeyKey);
    final publicKeyStr = await _storage.read(key: _publicKeyKey);

    if (privateKeyStr != null && publicKeyStr != null) {
      final privateKey = base64Decode(privateKeyStr);
      final publicKey = base64Decode(publicKeyStr);

      _keyPair = SimpleKeyPairData(
        privateKey,
        publicKey: SimplePublicKey(publicKey, type: KeyPairType.ed25519),
        type: KeyPairType.ed25519,
      );
    } else {
      final algorithm = Ed25519();
      _keyPair = await algorithm.newKeyPair();
      final keyData = await _keyPair!.extract();
      await _storage.write(
        key: _privateKeyKey,
        value: base64Encode(keyData.bytes),
      );
      await _storage.write(
        key: _publicKeyKey,
        value: base64Encode(keyData.publicKey.bytes),
      );
    }

    return _keyPair!;
  }

  Future<List<int>> getPrivateKeyBytes() async {
    final pair = await getKeyPair();
    final keyData = await pair.extract();
    return keyData.bytes;
  }

  Future<List<int>> getPublicKeyBytes() async {
    final pair = await getKeyPair();
    final publicKey = await pair.extractPublicKey();
    return publicKey.bytes;
  }
}
