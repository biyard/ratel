import 'package:ratel/exports.dart';

class PollBinding extends Bindings {
  @override
  void dependencies() {
    Get.lazyPut<PollController>(() => PollController());
    Get.lazyPut<PollViewerController>(() => PollViewerController());
    Get.lazyPut<PollCreatorController>(() => PollCreatorController());
  }
}
