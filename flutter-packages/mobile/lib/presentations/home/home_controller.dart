import 'package:ratel/exports.dart';

class HomeController extends BaseController {
  final feedsApi = Get.find<FeedsApi>();

  @override
  void onInit() {
    super.onInit();
    listFeeds();
  }

  void listFeeds() async {
    showLoading();
    final items = await feedsApi.listFeeds(1, 10);
    feeds.assignAll(items);
    logger.d('feeds loaded: ${feeds.length}');
    hideLoading();
  }

  RxList<FeedModel> feeds = <FeedModel>[].obs;
}
