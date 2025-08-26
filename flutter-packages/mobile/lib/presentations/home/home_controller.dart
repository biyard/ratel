import 'package:ratel/exports.dart';

class HomeController extends BaseController {
  final feedsApi = Get.find<FeedsApi>();
  final dashboardsApi = Get.find<DashboardsApi>();

  @override
  void onInit() {
    super.onInit();
    getDashboards();
  }

  void getDashboards() async {
    showLoading();
    final item = await dashboardsApi.getDashboards();
    logger.d(
      "space length: ${item.topSpaces.length} matched feeds length: ${item.matchedFeeds.length} new feeds length: ${item.newFeeds.length}",
    );
    topSpaces(item.topSpaces);
    matchedFeeds(item.matchedFeeds);
    newFeeds(item.newFeeds);
    hideLoading();
  }

  RxList<SpaceSummary> topSpaces = <SpaceSummary>[].obs;
  RxList<FeedSummary> matchedFeeds = <FeedSummary>[].obs;
  RxList<FeedSummary> newFeeds = <FeedSummary>[].obs;
}
