import 'package:ratel/exports.dart';

class HomeController extends BaseController {
  final feedsApi = Get.find<FeedsApi>();
  final RxList<FeedV2SummaryModel> feeds = <FeedV2SummaryModel>[].obs;

  final RxBool isLoading = false.obs;
  final RxBool isLoadingMore = false.obs;
  final RxBool hasMore = true.obs;
  String? _bookmark;

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
    feeds.clear();
    _bookmark = null;
    hasMore.value = true;

    try {
      final result = await feedsApi.listFeedsV2(bookmark: null);
      feeds.assignAll(result.items);
      logger.d("feeds data: ${result}");

      _bookmark = result.bookmark;
      hasMore.value = _bookmark != null && _bookmark!.isNotEmpty;
    } finally {
      hideLoading();
      isLoading.value = false;
    }
  }

  Future<void> loadMore() async {
    if (!hasMore.value || isLoadingMore.value) return;

    isLoadingMore.value = true;
    try {
      final result = await feedsApi.listFeedsV2(bookmark: _bookmark);
      feeds.addAll(result.items);
      _bookmark = result.bookmark;
      hasMore.value = _bookmark != null && _bookmark!.isNotEmpty;
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
