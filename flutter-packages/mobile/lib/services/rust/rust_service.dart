import 'package:get/get.dart';
import 'package:ratel/src/rust/api/icp/identity.dart' as identity;

class RustService extends GetxService {
  static void init() {
    Get.put<RustService>(RustService());
  }

  Future<identity.IdentityResponse> createIdentity(String privateKey) async {
    final identityResponse = await identity.createIdentity(
      privateKey: privateKey,
    );

    return identityResponse;
  }
}
