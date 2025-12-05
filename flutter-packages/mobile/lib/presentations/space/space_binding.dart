import 'package:ratel/exports.dart';

class SpaceBinding extends Bindings {
  @override
  void dependencies() {
    Get.lazyPut<SpaceController>(() => SpaceController());
  }
}
