import 'package:ratel/exports.dart';

class BookmarkController extends BaseController {
  final feedsApi = Get.find<FeedsApi>();

  @override
  void onInit() {
    super.onInit();
    getBookmarks();
  }

  void getBookmarks() async {
    showLoading();
    final item = await feedsApi.listBookmarkedFeeds();
    logger.d("feeds length: ${item.length}");
    bookmarkedFeeds(item);
    hideLoading();
  }

  Future<void> removebookmark(int feedId) async {
    try {
      final res = await feedsApi.removeBookmark(feedId);

      if (res != null) {
        Biyard.info("Remove Bookmarked successfully");
        getBookmarks();
      } else {
        Biyard.error(
          "Failed to remove bookmark.",
          "Remove Bookmarked failed. Please try again later.",
        );
      }
    } finally {}
  }

  void goBack() {
    final controller = Get.find<HomeController>();
    controller.getDashboards();
    Get.rootDelegate.offNamed(AppRoutes.mainScreen);
  }

  RxList<FeedSummary> bookmarkedFeeds = <FeedSummary>[].obs;
}
