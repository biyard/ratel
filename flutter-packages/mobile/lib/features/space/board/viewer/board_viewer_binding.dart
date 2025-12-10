import 'package:ratel/exports.dart';

class BoardViewerBinding extends Bindings {
  @override
  void dependencies() {
    Get.lazyPut<BoardViewerController>(() => BoardViewerController());
  }
}
