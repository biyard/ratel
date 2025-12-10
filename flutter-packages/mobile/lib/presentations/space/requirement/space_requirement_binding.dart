import 'package:ratel/exports.dart';

class SpaceRequirementBinding extends Bindings {
  @override
  void dependencies() {
    Get.lazyPut<SpaceRequirementController>(() => SpaceRequirementController());
  }
}
