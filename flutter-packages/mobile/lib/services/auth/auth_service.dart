import 'dart:convert';

import 'package:ratel/exports.dart';
import 'package:ratel/services/rust/rust_service.dart';

class AuthService extends GetxService {
  RustService rust = Get.find<RustService>();
  AnonymousService anonymous = Get.find<AnonymousService>();
  ByFirebase firebase = Get.find<ByFirebase>();
  String? idToken;
  String provider = '';

  String? principal;
  String? privateKey;
  String? publicKey;
  String? pkcs8;

  String? email;
  String? nickname;
  String? profileUrl;

  static void init() {
    Get.put<AuthService>(AuthService());
    Get.put<AuthApi>(AuthApi());
  }

  Future connectToGoogle(String redirectUri) async {
    logger.d('connectToGoogle');
    final user = await firebase.signIn();

    final accessToken = firebase.credential?.credential?.accessToken;
    logger.d('user $email $nickname $profileUrl');
    logger.d('accessToken $accessToken');

    final token = await firebase.idToken();
    logger.d('idToken token');
    idToken = token;
    provider = 'google';

    if (idToken == null && idToken == '') {
      Biyard.error("connect-to-google", "failed to get idToken");
      throw "failed to get idToken";
    }

    await requestToFirebase(accessToken);

    email = user?.email;
    nickname = user?.displayName;
    profileUrl = user?.photoURL;
  }

  Future<dynamic> requestToFirebase(String? accessToken) async {
    final api = DriveApi();

    final files = await api.listFiles(accessToken ?? "");
    final pk = await anonymous.getPrivateKeyBytes();
    final encodedPk = base64Encode(pk);
    logger.d("encodedPk: $encodedPk");

    if (files == null || files.files.isEmpty) {
      logger.e("No files found in Google Drive");
      final file = await api.uploadFile(accessToken ?? "", encodedPk);
      logger.d("Uploaded new file: ${file.id} ${file.name}");

      final identity = await rust.createIdentity(encodedPk);

      logger.d(
        "identity: ${identity.address} privateKey: ${identity.privateKey} publicKey: ${identity.publicKey}",
      );

      principal = identity.address;
      privateKey = identity.privateKey;
      publicKey = identity.publicKey;

      return; //return Signup Event, private key
    } else {
      final file = files.files.firstWhereOrNull((f) => f.name == Config.env);
      if (file == null) {
        throw Exception("Failed to get file");
      } else {
        final contents = await api.getFile(accessToken ?? "", file.id);
        if (contents == null) {
          throw Exception("Failed to get file contents");
        }
        logger.d("Found existing file: ${file.id} $contents");

        final identity = await rust.createIdentity(contents);

        logger.d(
          "identity: ${identity.address} privateKey: ${identity.privateKey} publicKey: ${identity.publicKey}",
        );

        principal = identity.address;
        privateKey = identity.privateKey;
        publicKey = identity.publicKey;

        return; //return Login Event, contents
      }
    }
  }

  // Future<String?> trySetupFromPrivateKey(String base64Pkcs8) async {
  //   try {
  //     final pkcs8Bytes = base64.decode(base64Pkcs8);
  //     logger.d("bytes decoded: ${pkcs8Bytes.length}");

  //     final keyPair = await _keyPairFromPkcs8Bytes(pkcs8Bytes);
  //     if (keyPair == null) {
  //       throw Exception("Failed to parse PKCS#8 or construct key pair");
  //     }

  //     privateKey = base64Pkcs8;
  //     final pubKey = await keyPair.extractPublicKey();
  //     publicKey = base64.encode(pubKey.bytes);
  //     logger.d("Public key encoded: $publicKey");

  //     if (publicKey != null) {
  //       principal = getPrincipal();
  //       logger.d("Principal: $principal");
  //     }

  //     return principal;
  //   } catch (e, st) {
  //     logger.e("trySetupFromPrivateKey error: $e\n$st");
  //     return null;
  //   }
  // }

  // Future<SimpleKeyPairData?> _keyPairFromPkcs8Bytes(Uint8List bytes) async {
  //   try {
  //     final parser = ASN1Parser(bytes);
  //     final seq = parser.nextObject() as ASN1Sequence;

  //     final outerOctet = seq.elements![2] as ASN1OctetString;

  //     final innerParser = ASN1Parser(outerOctet.valueBytes!);
  //     final innerOctet = innerParser.nextObject() as ASN1OctetString;
  //     final seed = innerOctet.valueBytes!;

  //     if (seed.length != 32) {
  //       throw Exception("Seed must be exactly 32 bytes. Got ${seed.length}");
  //     }

  //     final algorithm = Ed25519();
  //     final keyPair = await algorithm.newKeyPairFromSeed(seed);
  //     return keyPair as SimpleKeyPairData;
  //   } catch (e, st) {
  //     logger.e("Failed to parse PKCS#8: $e\n$st");
  //     return null;
  //   }
  // }

  // String getPrincipal() {
  //   final decoded = base64.decode(publicKey!);

  //   final hash = sha224.convert(decoded).bytes;
  //   final principalBytes = Uint8List.fromList([0x02, ...hash]);

  //   final checksum = getCrc32(principalBytes);
  //   final checksumBytes = Uint8List(4)
  //     ..buffer.asByteData().setUint32(0, checksum, Endian.little);

  //   final combined = Uint8List.fromList([...checksumBytes, ...principalBytes]);
  //   final text = base32.encode(combined).toLowerCase();

  //   final buffer = StringBuffer();
  //   for (int i = 0; i < text.length; i++) {
  //     if (i != 0 && i % 5 == 0) buffer.write('-');
  //     buffer.write(text[i]);
  //   }

  //   return buffer.toString();
  // }

  // Future<SimpleKeyPair?> initOrGetIdentity(Uint8List? inputPkcs8) async {
  //   if (pkcs8 == null && inputPkcs8 != null) {
  //     String encoded = base64.encode(inputPkcs8);
  //     pkcs8 = encoded;
  //   }
  //   return await getIdentity();
  // }

  // Future<SimpleKeyPair?> getIdentity() async {
  //   if (pkcs8 != null) {
  //     try {
  //       final decoded = base64.decode(pkcs8!);
  //       final seed = decoded.sublist(0, 32);
  //       final keyPair = await Ed25519().newKeyPairFromSeed(seed);
  //       return keyPair;
  //     } catch (e) {
  //       logger.d("Could not read the key pair: $e");
  //       return null;
  //     }
  //   } else {
  //     return null;
  //   }
  // }
}

// class Crc32 {
//   static const _crcTable = [0x00000000, 0x77073096, 0xee0e612c, 0x990951ba];

//   static int compute(Uint8List data) {
//     int crc = 0xffffffff;

//     for (final byte in data) {
//       crc = (_crcTable[(crc ^ byte) & 0xff] ^ (crc >> 8)) & 0xffffffff;
//     }

//     return crc ^ 0xffffffff;
//   }
// }
