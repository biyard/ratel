import 'package:ratel/exports.dart';

class FileViewerBinding extends Bindings {
  @override
  void dependencies() {
    Get.lazyPut<FileViewerController>(() => FileViewerController());
  }
}
