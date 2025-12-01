import 'package:ratel/exports.dart';

class OverviewBinding extends Bindings {
  @override
  void dependencies() {
    Get.lazyPut<OverviewController>(() => OverviewController());
  }
}
