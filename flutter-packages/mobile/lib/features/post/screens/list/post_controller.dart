import 'package:ratel/exports.dart';

class PostController extends BaseController {
  final feedsService = Get.find<FeedsService>();

  RxList<FeedV2SummaryModel> get feeds => feedsService.summaries;

  RxBool isInitialLoading = false.obs;
  RxBool isLoadingMore = false.obs;
  late ScrollController scrollController;

  @override
  void onInit() {
    super.onInit();
    scrollController = ScrollController();
    scrollController.addListener(_onScroll);
    loadInitial();
  }

  Future<void> loadInitial() async {
    isInitialLoading.value = true;
    try {
      await feedsService.loadInitial();
    } finally {
      isInitialLoading.value = false;
    }
  }

  Future<void> loadMore() async {
    if (!feedsService.hasMore) return;
    if (isLoadingMore.value) return;

    isLoadingMore.value = true;
    try {
      await feedsService.loadMore();
    } finally {
      isLoadingMore.value = false;
    }
  }

  bool get hasMore => feedsService.hasMore;

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
