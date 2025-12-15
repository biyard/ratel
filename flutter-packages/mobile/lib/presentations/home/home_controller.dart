import 'package:ratel/exports.dart';

class HomeController extends BaseController {
  final feedsService = Get.find<FeedsService>();

  RxList<FeedSummaryModel> get feeds => feedsService.homeFeeds;

  final RxBool isLoading = false.obs;
  final RxBool isLoadingMore = false.obs;
  final RxBool hasMore = false.obs;

  late final ScrollController scrollController;

  @override
  void onInit() {
    super.onInit();
    scrollController = ScrollController()..addListener(_onScroll);
    loadInitial();
  }

  void _onScroll() {
    if (!scrollController.hasClients) return;
    if (!hasMore.value || isLoadingMore.value) return;

    final max = scrollController.position.maxScrollExtent;
    final offset = scrollController.offset;

    if (offset >= max - 200) {
      loadMore();
    }
  }

  Future<void> loadInitial() async {
    isLoading.value = true;
    showLoading();

    try {
      await feedsService.loadHomeInitial();
      hasMore.value = feedsService.hasMoreHome;
    } finally {
      hideLoading();
      isLoading.value = false;
    }
  }

  Future<void> loadMore() async {
    if (!hasMore.value || isLoadingMore.value) return;

    isLoadingMore.value = true;
    try {
      await feedsService.loadHomeMore();
      hasMore.value = feedsService.hasMoreHome;
    } finally {
      isLoadingMore.value = false;
    }
  }

  @override
  void onClose() {
    scrollController.dispose();
    super.onClose();
  }
}
