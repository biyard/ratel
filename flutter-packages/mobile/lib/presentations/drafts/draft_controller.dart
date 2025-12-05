import 'package:ratel/exports.dart';

class DraftController extends BaseController {
  final feedsService = Get.find<FeedsService>();

  RxList<FeedV2SummaryModel> get feeds => feedsService.drafts;
  RxBool isBusy = false.obs;

  @override
  void onInit() {
    super.onInit();
    listFeeds();
  }

  Future<void> listFeeds({bool loadMore = false}) async {
    if (!loadMore) {
      showLoading();
    }

    try {
      if (loadMore) {
        await feedsService.loadDraftsMore();
      } else {
        await feedsService.loadDraftsInitial();
      }
    } finally {
      if (!loadMore) hideLoading();
    }
  }

  bool get hasMore => feedsService.hasMoreDrafts;

  void goBack() {
    Get.rootDelegate.offNamed(AppRoutes.mainScreen);
  }

  void openDraft(String pk) {
    Get.rootDelegate.toNamed(createPostScreen, arguments: {'postPk': pk});
  }

  Future<void> deleteDraft(String pk) async {
    if (isBusy.value) return;
    isBusy.value = true;

    try {
      final ok = await feedsService.deleteDraft(pk);
      if (ok) {
        Biyard.info('Delete Draft successfully');
      } else {
        Biyard.error('Failed to delete draft.', 'Please try again later.');
      }
    } finally {
      isBusy.value = false;
    }
  }
}
