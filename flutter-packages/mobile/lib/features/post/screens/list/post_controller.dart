import 'package:ratel/exports.dart';

class PostController extends BaseController {
  final feedsService = Get.find<FeedsService>();
  final feedsApi = Get.find<FeedsApi>();

  RxList<FeedSummaryModel> get feeds => feedsService.summaries;

  RxBool isInitialLoading = false.obs;
  RxBool isLoadingMore = false.obs;
  final RxBool isLikingPost = false.obs;
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
      logger.e('Failed to toggle like from posts: $e', stackTrace: s);
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
