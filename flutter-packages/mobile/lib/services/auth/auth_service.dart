import 'dart:convert';

import 'package:crypto/crypto.dart' as crypto;
import 'package:cryptography/cryptography.dart' as cg;
import 'package:ratel/exports.dart';
import 'package:ratel/services/auth/ic_principal.dart';
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

    idToken = token;
    provider = 'google';

    logger.d('idToken token: ${idToken}');

    if (idToken == null && idToken == '') {
      Biyard.error("connect-to-google", "failed to get idToken");
      throw "failed to get idToken";
    }

    await requestToFirebase(accessToken);

    email = user?.email;
    nickname = user?.displayName;
    profileUrl = user?.photoURL;
  }

  Future<String> requestToFirebase(String? accessToken) async {
    final api = DriveApi();

    final files = await api.listFiles(accessToken ?? "");
    final pk = await anonymous.getPrivateKeyBytes();
    final encodedPk = base64Encode(pk);
    logger.d("encodedPk: $encodedPk");

    if (files == null || files.files.isEmpty) {
      logger.e("No files found in Google Drive");
      final file = await api.uploadFile(accessToken ?? "", encodedPk);
      logger.d("Uploaded new file: ${file.id} ${file.name}");

      final p = await IcpPrincipalAgent.fromPkcs8Base64(encodedPk);

      logger.d("identity: ${p}");

      principal = p;
      privateKey = encodedPk;

      return encodedPk; //return Signup Event, private key
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

        final p = await IcpPrincipalAgent.fromPkcs8Base64(contents);
        logger.d('principal(google signed in): $p');

        // logger.d(
        //   "identity: ${identity.address} privateKey: ${identity.privateKey} publicKey: ${identity.publicKey}",
        // );

        principal = p;
        privateKey = contents;
        // privateKey = identity.privateKey;
        // publicKey = identity.publicKey;

        return contents; //return Login Event, contents
      }
    }
  }
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
