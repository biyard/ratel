import 'package:ratel/exports.dart';

class DraftController extends BaseController {
  final userApi = Get.find<UserApi>();
  final feedsApi = Get.find<FeedsApi>();

  RxList<FeedV2SummaryModel> feeds = <FeedV2SummaryModel>[].obs;
  Rx<bool> isBusy = false.obs;
  String? bookmark;

  @override
  void onInit() {
    super.onInit();
    listFeeds();
  }

  Future<void> listFeeds({bool loadMore = false}) async {
    if (!loadMore) {
      showLoading();
      bookmark = null;
    }

    try {
      final result = await feedsApi.listDraftsV2(bookmark: bookmark);
      bookmark = result.bookmark;

      if (loadMore) {
        feeds.addAll(result.items);
      } else {
        feeds.assignAll(result.items);
      }

      logger.d('draft feeds loaded: ${feeds.length}');
    } finally {
      hideLoading();
    }
  }

  void goBack() {
    Get.rootDelegate.offNamed(AppRoutes.mainScreen);
  }

  void openDraft(String pk) {
    Get.rootDelegate.toNamed(createPostScreen, arguments: {'postPk': pk});
  }

  Future<void> deleteDraft(String pk) async {
    logger.d("delete draft pk: $pk");
    if (isBusy.value) return;
    isBusy.value = true;

    try {
      await feedsApi.deletePostV2(pk);
      Biyard.info("Delete Draft successfully");
      await listFeeds();
    } finally {
      isBusy.value = false;
    }
  }

  bool get hasMore => bookmark != null;
}
