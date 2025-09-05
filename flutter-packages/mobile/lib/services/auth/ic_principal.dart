import 'dart:convert';
import 'dart:typed_data';
import 'package:agent_dart/agent_dart.dart';
import 'package:cryptography/cryptography.dart' as cg;

class IcpPrincipalAgent {
  static Future<String> fromPkcs8Base64(String pkcs8b64) async {
    final der = base64Decode(pkcs8b64);
    final seed = _extractSeed32(der);

    final pair = await cg.Ed25519().newKeyPairFromSeed(seed);
    final pub = await pair.extractPublicKey();
    final spkiDer = _ed25519SpkiFromRaw(Uint8List.fromList(pub.bytes));

    final p = Principal.selfAuthenticating(spkiDer);
    return p.toText();
  }

  static Uint8List _extractSeed32(Uint8List der) {
    for (int i = 0; i + 34 <= der.length; i++) {
      if (der[i] == 0x04 && der[i + 1] == 0x20) {
        return Uint8List.fromList(der.sublist(i + 2, i + 34));
      }
    }
    throw ArgumentError('Ed25519 seed (32 bytes) not found in PKCS#8');
  }

  static Uint8List _ed25519SpkiFromRaw(Uint8List raw32) {
    return Uint8List.fromList([
      0x30,
      0x2a,
      0x30,
      0x05,
      0x06,
      0x03,
      0x2b,
      0x65,
      0x70,
      0x03,
      0x21,
      0x00,
      ...raw32,
    ]);
  }
}
