import 'package:ratel/exports.dart';

class PostController extends BaseController {
  final feedsApi = Get.find<FeedsApi>();

  RxList<FeedV2SummaryModel> feeds = <FeedV2SummaryModel>[].obs;
  RxBool isLoadingMore = false.obs;
  String? bookmark;
  late ScrollController scrollController;

  @override
  void onInit() {
    super.onInit();
    scrollController = ScrollController();
    scrollController.addListener(_onScroll);
    loadInitial();
  }

  Future<void> loadInitial() async {
    bookmark = null;
    await _loadFeeds(reset: true);
  }

  void listFeeds() {
    loadInitial();
  }

  Future<void> _loadFeeds({required bool reset}) async {
    if (reset) showLoading();

    try {
      final result = await feedsApi.listPostsV2(bookmark: bookmark);
      bookmark = result.bookmark;

      if (reset) {
        feeds.assignAll(result.items);
      } else {
        feeds.addAll(result.items);
      }

      logger.d('posts loaded: ${feeds.length}');
    } finally {
      if (reset) hideLoading();
    }
  }

  Future<void> loadMore() async {
    if (bookmark == null) return;
    if (isLoadingMore.value) return;

    isLoadingMore.value = true;
    try {
      await _loadFeeds(reset: false);
    } finally {
      isLoadingMore.value = false;
    }
  }

  bool get hasMore => bookmark != null;

  void _onScroll() {
    if (!scrollController.hasClients) return;
    final position = scrollController.position;
    if (position.pixels >= position.maxScrollExtent - 200) {
      loadMore();
    }
  }

  void goBack() {
    Get.rootDelegate.offNamed(AppRoutes.mainScreen);
  }

  @override
  void onClose() {
    scrollController.dispose();
    super.onClose();
  }
}
