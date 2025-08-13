import 'package:ratel/exports.dart';

class SelectTopicController extends BaseController {
  @override
  void onInit() {
    listIndustries();
    super.onInit();
  }

  void listIndustries() async {
    final items = await industryApi.getIndustries();
    industries.assignAll(items);
    logger.d('industries loaded: ${industries.length}');
  }

  final industryApi = Get.find<IndustryApi>();
  final int minRequired = 2;
  RxList<IndustryModel> industries = <IndustryModel>[].obs;

  final query = ''.obs;
  final selected = <IndustryModel>{}.obs;

  List<IndustryModel> get filtered => industries
      .where((e) => e.label.toLowerCase().contains(query.value.toLowerCase()))
      .toList();

  void onSearchChanged(String v) => query.value = v;
  void toggle(String tag) {
    final q = tag.toLowerCase();

    final exists = selected.any((e) => e.label.toLowerCase().contains(q));

    if (exists) {
      selected.removeWhere((e) => e.label.toLowerCase().contains(q));
    } else {
      final matches = industries
          .where((e) => e.label.toLowerCase().contains(q))
          .toList();
      if (matches.isNotEmpty) {
        selected.add(matches.first);
      }
    }
  }

  void goBack() => Get.rootDelegate.offNamed(AppRoutes.setupProfileScreen);
  void skip() => Get.rootDelegate.offNamed(AppRoutes.connectionScreen);

  Future<void> next() async {
    final industry = IndustryApi();
    logger.d("selected: $selected");
    List<int> selectedIds = selected.map((e) => e.id).toList();

    try {
      final res = await industry.selectTopics(selectedIds);

      if (res != null) {
        Get.rootDelegate.offNamed(AppRoutes.connectionScreen);
      } else {
        Biyard.error(
          "Failed to subscribe topic.",
          "Subscribe topic is failed. Please try again later.",
        );
      }
    } finally {}
  }
}
