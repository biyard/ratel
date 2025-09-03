import 'package:ratel/exports.dart';

class DraftController extends BaseController {
  final userApi = Get.find<UserApi>();
  final feedsApi = Get.find<FeedsApi>();

  @override
  void onInit() {
    super.onInit();
    listFeeds();
  }

  void listFeeds() async {
    showLoading();
    final item = await userApi.getUserInfo();
    final userId = item.id;
    final items = await feedsApi.listFeedsByUserId(1, 10, userId, 1);
    feeds.assignAll(items);
    logger.d('feeds loaded: ${feeds.length}');
    hideLoading();
  }

  void goBack() {
    Get.rootDelegate.offNamed(AppRoutes.mainScreen);
  }

  void openDraft(int feedId) {
    Get.rootDelegate.toNamed(AppRoutes.draftWithId(feedId));
  }

  Future<void> deleteDraft(int feedId) async {
    logger.d("delete draft id: ${feedId}");
    try {
      final res = await feedsApi.deleteFeed(feedId);

      if (res != null) {
        Biyard.info("Delete Draft successfully");
        listFeeds();
      } else {
        Biyard.error(
          "Failed to delete draft.",
          "Delete Draft failed. Please try again later.",
        );
      }
    } finally {}
  }

  RxList<FeedModel> feeds = <FeedModel>[].obs;
  Rx<bool> isBusy = false.obs;
}
