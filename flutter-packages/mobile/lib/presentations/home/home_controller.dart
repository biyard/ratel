import 'package:ratel/exports.dart';

class HomeController extends BaseController {
  final feedsService = Get.find<FeedsService>();
  final feedsApi = Get.find<FeedsApi>();

  RxList<FeedSummaryModel> get feeds => feedsService.homeFeeds;

  final RxBool isLoading = false.obs;
  final RxBool isLoadingMore = false.obs;
  final RxBool hasMore = false.obs;

  final RxBool isLikingPost = false.obs;

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

  Future<void> toggleLikePost(FeedSummaryModel target) async {
    if (isLikingPost.value) return;

    final index = feeds.indexWhere((f) => f.pk == target.pk);
    if (index == -1) return;

    final original = feeds[index];

    final alreadyLiked = original.liked == true;
    final nextLike = !alreadyLiked;

    final originalLikes = original.likes;
    var newLikes = originalLikes;

    if (nextLike && !alreadyLiked) {
      newLikes = originalLikes + 1;
    } else if (!nextLike && alreadyLiked && originalLikes > 0) {
      newLikes = originalLikes - 1;
    }

    feeds[index].liked = nextLike;
    feeds[index].likes = newLikes;
    feeds.refresh();

    isLikingPost.value = true;
    try {
      final res = await feedsApi.likePost(postPk: original.pk, like: nextLike);

      if (res == null || res.like != nextLike) {
        feeds[index].liked = alreadyLiked;
        feeds[index].likes = originalLikes;
        feeds.refresh();
        return;
      }

      feedsService.patchDetailFromSummary(feeds[index]);

      if (nextLike) {
        Biyard.info("Success to like post");
      } else {
        Biyard.info("Success to unlike post");
      }
    } catch (e, s) {
      logger.e('Failed to toggle like from home: $e', stackTrace: s);
      feeds[index].liked = alreadyLiked;
      feeds[index].likes = originalLikes;
      feeds.refresh();

      if (nextLike) {
        Biyard.error(
          "Like Failed",
          "Failed to like post. Please try again later.",
        );
      } else {
        Biyard.error(
          "Unlike Failed",
          "Failed to unlike post. Please try again later.",
        );
      }
    } finally {
      isLikingPost.value = false;
    }
  }

  @override
  void onClose() {
    scrollController.dispose();
    super.onClose();
  }
}
