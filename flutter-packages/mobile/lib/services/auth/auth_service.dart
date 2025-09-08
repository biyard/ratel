import 'dart:convert';
import 'dart:typed_data';

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

  bool neededSignup = false; //check needed to social signup

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

    if (files == null || files.files.isEmpty) {
      final pair = await cg.Ed25519().newKeyPair();
      final cg.SimpleKeyPairData data = await pair.extract();

      final Uint8List priv = Uint8List.fromList(
        await data.extractPrivateKeyBytes(),
      );

      final Uint8List seed32 = priv.length >= 32
          ? Uint8List.fromList(priv.sublist(0, 32))
          : priv;

      final cg.SimplePublicKey pub = await pair.extractPublicKey();

      final der = _encodePkcs8WithPublic(seed32, Uint8List.fromList(pub.bytes));
      final pkcs8B64 = base64Encode(der);

      neededSignup = true;

      final file = await api.uploadFile(accessToken ?? "", pkcs8B64);
      logger.d("Uploaded new file: ${file.id} ${file.name}");

      final p = await IcpPrincipalAgent.fromPkcs8Base64(pkcs8B64);
      principal = p;
      privateKey = pkcs8B64;

      logger.d("principal: ${principal} privateKey: ${privateKey}");

      return pkcs8B64;
    }

    final file = files.files.firstWhereOrNull((f) => f.name == Config.env);
    if (file == null) {
      neededSignup = true;
      throw Exception("Failed to get file");
    } else {
      neededSignup = false;
      final contents = await api.getFile(accessToken ?? "", file.id);
      if (contents == null) throw Exception("Failed to get file contents");
      logger.d("Found existing file: ${file.id} $contents");

      final p = await IcpPrincipalAgent.fromPkcs8Base64(contents);
      principal = p;
      privateKey = contents;
      return contents;
    }
  }

  Uint8List _encodePkcs8WithPublic(Uint8List seed32, Uint8List pub32) {
    Uint8List _len(int n) {
      if (n < 128) return Uint8List.fromList([n]);
      final out = <int>[];
      var v = n;
      while (v > 0) {
        out.insert(0, v & 0xFF);
        v >>= 8;
      }
      return Uint8List.fromList([0x80 | out.length, ...out]);
    }

    final body = BytesBuilder()
      ..add([0x02, 0x01, 0x01])
      ..add([0x30, 0x05, 0x06, 0x03, 0x2B, 0x65, 0x70])
      ..add([0x04])
      ..add(_len(0x22))
      ..add([0x04, 0x20])
      ..add(seed32)
      ..add([0xA1, 0x23])
      ..add([0x03, 0x21, 0x00])
      ..add(pub32);

    final bodyBytes = body.toBytes();
    return Uint8List.fromList([0x30, ..._len(bodyBytes.length), ...bodyBytes]);
  }
}
