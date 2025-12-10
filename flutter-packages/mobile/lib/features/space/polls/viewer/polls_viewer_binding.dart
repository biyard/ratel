import 'package:ratel/exports.dart';

class PollsViewerBinding extends Bindings {
  @override
  void dependencies() {
    Get.lazyPut<PollsViewerController>(() => PollsViewerController());
  }
}
