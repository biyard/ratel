import 'package:ratel/exports.dart';

class DraftBinding extends Bindings {
  @override
  void dependencies() {
    Get.lazyPut<DraftController>(() => DraftController());
  }
}
