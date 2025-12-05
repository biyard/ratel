import 'package:ratel/exports.dart';

class PollsBinding extends Bindings {
  @override
  void dependencies() {
    Get.lazyPut<PollsController>(() => PollsController());
    Get.lazyPut<PollsViewerController>(() => PollsViewerController());
    Get.lazyPut<PollsCreatorController>(() => PollsCreatorController());
  }
}
