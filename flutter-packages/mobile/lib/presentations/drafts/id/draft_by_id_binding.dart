import 'package:ratel/exports.dart';

class DraftByIdBinding extends Bindings {
  @override
  void dependencies() {
    Get.lazyPut<DraftByIdController>(() => DraftByIdController());
  }
}
