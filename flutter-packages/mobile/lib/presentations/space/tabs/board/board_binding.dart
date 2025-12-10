import 'package:ratel/exports.dart';

class BoardBinding extends Bindings {
  @override
  void dependencies() {
    Get.lazyPut<BoardController>(() => BoardController());
    Get.lazyPut<BoardCreatorController>(() => BoardCreatorController());
    Get.lazyPut<BoardViewerController>(() => BoardViewerController());
  }
}
