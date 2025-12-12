import 'package:ratel/exports.dart';

class OverviewCreatorBinding extends Bindings {
  @override
  void dependencies() {
    Get.lazyPut<OverviewCreatorController>(() => OverviewCreatorController());
  }
}
