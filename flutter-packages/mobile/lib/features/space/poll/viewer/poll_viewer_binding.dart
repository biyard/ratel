import 'package:ratel/exports.dart';

class PollViewerBinding extends Bindings {
  @override
  void dependencies() {
    Get.lazyPut<PollViewerController>(() => PollViewerController());
  }
}
