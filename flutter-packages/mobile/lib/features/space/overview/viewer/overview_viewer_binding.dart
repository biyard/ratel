import 'package:ratel/exports.dart';

class OverviewViewerBinding extends Bindings {
  @override
  void dependencies() {
    Get.lazyPut<OverviewViewerController>(() => OverviewViewerController());
  }
}
