import 'package:ratel/exports.dart';

class BoardsBinding extends Bindings {
  @override
  void dependencies() {
    Get.lazyPut<BoardsController>(() => BoardsController());
    Get.lazyPut<BoardsCreatorController>(() => BoardsCreatorController());
    Get.lazyPut<BoardsViewerController>(() => BoardsViewerController());
  }
}
