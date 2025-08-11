import 'package:ratel/exports.dart';

class SelectTopicController extends BaseController {
  //FIXME: fix to query api
  final industries = [
    "Crypto",
    'AI',
    'Mobility',
    'Bio & Healthcare',
    'Semiconductor',
    'Defense Tech',
    'Smart City',
    'Content & Entertainment',
    'Fintech',
    'Green Tech',
    'Security',
  ];

  final query = ''.obs;
  final selected = <String>{}.obs;

  List<String> get filtered => industries
      .where((e) => e.toLowerCase().contains(query.value.toLowerCase()))
      .toList();

  void onSearchChanged(String v) => query.value = v;
  void toggle(String tag) =>
      selected.contains(tag) ? selected.remove(tag) : selected.add(tag);

  void goBack() => Get.rootDelegate.offNamed(AppRoutes.setupProfileScreen);
  void skip() => Get.rootDelegate.offNamed(AppRoutes.setupProfileScreen);
  void next() => {
    logger.d("selected: $selected"),
    Get.rootDelegate.offNamed(AppRoutes.connectionScreen),
  };
}
