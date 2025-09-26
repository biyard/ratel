import 'package:ratel/exports.dart';

class NoticeSpaceBinding extends Bindings {
  @override
  void dependencies() {
    Get.lazyPut<NoticeSpaceController>(() => NoticeSpaceController());
  }
}
