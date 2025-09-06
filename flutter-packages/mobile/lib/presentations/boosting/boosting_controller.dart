import 'package:ratel/exports.dart';

class BoostingController extends BaseController {
  Rx<RewardModel> reward = RewardModel(points: 200000).obs;

  RxList<BoostingModel> boostings = <BoostingModel>[
    BoostingModel(
      id: 1,
      updatedAt: 1757453022,
      points: 300,
      ratels: 0,
      exchanged: false,
    ),
    BoostingModel(
      id: 2,
      updatedAt: 1754774622,
      points: 300,
      ratels: 300,
      exchanged: true,
    ),
    BoostingModel(
      id: 3,
      updatedAt: 1752096222,
      points: 300,
      ratels: 0,
      exchanged: false,
    ),
  ].obs;

  void goBack() {
    Get.rootDelegate.offNamed(AppRoutes.mainScreen);
  }
}
