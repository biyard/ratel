import 'package:ratel/exports.dart';

class DeliberationSpaceBinding extends Bindings {
  @override
  void dependencies() {
    Get.lazyPut<DeliberationSpaceController>(
      () => DeliberationSpaceController(),
    );
  }
}
