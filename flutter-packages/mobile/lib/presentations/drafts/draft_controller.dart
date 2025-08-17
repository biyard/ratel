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

  RxList<FeedModel> feeds = <FeedModel>[].obs;
}
