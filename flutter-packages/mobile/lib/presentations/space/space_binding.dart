import 'package:ratel/exports.dart';

class SpaceBinding extends Bindings {
  @override
  void dependencies() {
    Get.lazyPut<DeliberationSpaceController>(
      () => DeliberationSpaceController(),
    );
    Get.lazyPut<NotFoundSpaceController>(() => NotFoundSpaceController());
    Get.lazyPut<NoticeSpaceController>(() => NoticeSpaceController());
  }
}
