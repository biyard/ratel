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

  void goBack() {
    Get.rootDelegate.offNamed(AppRoutes.mainScreen);
  }

  RxList<FeedSummary> bookmarkedFeeds = <FeedSummary>[].obs;
}
