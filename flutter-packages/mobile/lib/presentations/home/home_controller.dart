import 'package:ratel/exports.dart';

class HomeController extends BaseController {
  final feedsApi = Get.find<FeedsApi>();
  final dashboardsApi = Get.find<DashboardsApi>();

  @override
  void onInit() {
    super.onInit();
    listFeeds();
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

  void listFeeds() async {
    final items = await feedsApi.listFeeds(1, 10);
    feeds.assignAll(items);
    logger.d('feeds loaded: ${feeds.length}');
  }

  RxList<FeedModel> feeds = <FeedModel>[].obs;

  RxList<SpaceSummary> topSpaces = <SpaceSummary>[].obs;
  RxList<FeedSummary> matchedFeeds = <FeedSummary>[].obs;
  RxList<FeedSummary> newFeeds = <FeedSummary>[].obs;
}
