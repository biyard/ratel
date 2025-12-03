import 'package:ratel/exports.dart';

class BoardsViewerBinding extends Bindings {
  @override
  void dependencies() {
    Get.lazyPut<BoardsViewerController>(() => BoardsViewerController());
  }
}
