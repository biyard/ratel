import 'package:ratel/exports.dart';

class NotFoundSpaceBinding extends Bindings {
  @override
  void dependencies() {
    Get.lazyPut<NotFoundSpaceController>(() => NotFoundSpaceController());
  }
}
