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

  Future<void> addBookmark(int feedId) async {
    logger.d("bookmarked feed id: ${feedId}");
    try {
      final res = await feedsApi.addBookmark(feedId);

      if (res != null) {
        Biyard.info("Bookmarked successfully");
        getDashboards();
      } else {
        Biyard.error(
          "Failed to bookmark.",
          "Bookmarked failed. Please try again later.",
        );
      }
    } finally {}
  }

  Future<void> removebookmark(int feedId) async {
    try {
      final res = await feedsApi.removeBookmark(feedId);

      if (res != null) {
        Biyard.info("Remove Bookmarked successfully");
        getDashboards();
      } else {
        Biyard.error(
          "Failed to remove bookmark.",
          "Remove Bookmarked failed. Please try again later.",
        );
      }
    } finally {}
  }

  RxList<SpaceSummary> topSpaces = <SpaceSummary>[].obs;
  RxList<FeedSummary> matchedFeeds = <FeedSummary>[].obs;
  RxList<FeedSummary> newFeeds = <FeedSummary>[].obs;
}
