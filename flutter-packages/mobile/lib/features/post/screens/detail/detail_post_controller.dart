import 'package:ratel/exports.dart';

class DetailPostController extends BaseController {
  final reportApi = Get.find<ReportApi>();
  final userService = Get.find<UserService>();
  final feedsApi = Get.find<FeedsApi>();
  final feedsService = Get.find<FeedsService>();

  late final String postPk;

  final feed = Rxn<FeedModel>();
  final isLoading = false.obs;

  final isSendingRootComment = false.obs;
  final likingCommentOf = <String, bool>{}.obs;

  final isLikingPost = false.obs;
  bool get isPostLiked => feed.value?.isLiked == true;
  int get postLikes => feed.value?.post.likes ?? 0;

  Rx<UserModel> get user => userService.user;

  late final Worker _detailSubscription;

  @override
  void onInit() {
    super.onInit();

    final raw = Get.parameters['pk'];
    if (raw == null) {
      logger.e('post pk is null. route: ${Get.currentRoute}');
      return;
    }

    postPk = Uri.decodeComponent(raw);
    logger.d('DetailPostController postPk = $postPk');

    _detailSubscription = ever<Map<String, FeedModel>>(feedsService.details, (
      map,
    ) {
      final updated = map[postPk];
      if (updated != null) {
        feed.value = updated;
      }
    });

    loadFeed();
  }

  @override
  void onClose() {
    _detailSubscription.dispose();
    super.onClose();
  }

  Future<void> loadFeed({bool forceRefresh = false}) async {
    try {
      isLoading.value = true;
      final result = await feedsService.fetchDetail(
        postPk,
        forceRefresh: forceRefresh,
      );

      final spacePk = result.post.spacePk;
      if (spacePk != null && spacePk.isNotEmpty) {
        Get.rootDelegate.offNamed(spaceWithPk(spacePk));
        return;
      }

      feed.value = result;
    } catch (e, s) {
      logger.e('Failed to load feed detail: $e', stackTrace: s);
    } finally {
      isLoading.value = false;
    }
  }

  Future<void> deletePost({required String postPk}) async {
    final ok = await feedsService.deletePost(postPk);
    if (!ok) {
      Biyard.error(
        'Delete post failed.',
        'Failed to delete post. Please try again later.',
      );
      return;
    }

    Get.back();
    Biyard.info("Success to delete post");
  }

  bool isCommentLiked(String commentSk, {bool fallback = false}) {
    final current = feed.value;
    if (current == null) return fallback;

    for (final c in current.comments) {
      if (c.sk == commentSk) {
        return c.liked == true;
      }
    }

    return fallback;
  }

  Future<void> toggleLikeComment({required String commentSk}) async {
    final current = feed.value;
    if (current == null) return;

    if (likingCommentOf[commentSk] == true) return;

    final idx = current.comments.indexWhere((c) => c.sk == commentSk);
    if (idx < 0) return;

    final target = current.comments[idx];
    final previousLiked = target.liked == true;
    final nextLike = !previousLiked;

    likingCommentOf[commentSk] = true;
    likingCommentOf.refresh();

    try {
      final res = await feedsApi.likeComment(
        postPk: current.post.pk,
        commentSk: commentSk,
        like: nextLike,
      );

      if (res == null) return;

      final actualLiked = res.liked;

      var likes = target.likes;
      if (actualLiked && !previousLiked) {
        likes = likes + 1;
      } else if (!actualLiked && previousLiked && likes > 0) {
        likes = likes - 1;
      }

      target.likes = likes;
      target.liked = actualLiked;

      if (nextLike) {
        Biyard.info("Success to like post");
      } else {
        Biyard.info("Success to unlike post");
      }

      feed.refresh();
      feedsService.updateDetail(current);
    } catch (e, s) {
      logger.e('Failed to like/unlike comment $commentSk: $e', stackTrace: s);

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
      likingCommentOf[commentSk] = false;
      likingCommentOf.refresh();
    }
  }

  Future<PostCommentModel?> addComment(String text) async {
    final current = feed.value;
    if (current == null) return null;
    if (text.trim().isEmpty) return null;
    if (isSendingRootComment.value) return null;

    isSendingRootComment.value = true;

    final html = _wrapAsDiv(text);

    try {
      final created = await feedsApi.createComment(
        postPk: current.post.pk,
        content: html,
      );
      if (created == null) return null;

      current.post.comments = current.post.comments + 1;

      feed.value = FeedModel(
        post: current.post,
        comments: [created, ...current.comments],
        artworkMetadata: current.artworkMetadata,
        repost: current.repost,
        isLiked: current.isLiked,
        isReport: current.isReport,
        permissions: current.permissions,
      );

      Biyard.info("Success to create comment");
      feedsService.updateDetail(feed.value!);

      return created;
    } catch (e, s) {
      logger.e('Failed to add comment: $e', stackTrace: s);
      Biyard.info("Failed to create comment. please try again later.");
      return null;
    } finally {
      isSendingRootComment.value = false;
    }
  }

  String _wrapAsDiv(String text) {
    final escaped = text
        .replaceAll('&', '&amp;')
        .replaceAll('<', '&lt;')
        .replaceAll('>', '&gt;');
    return '<div>$escaped</div>';
  }

  Future<void> toggleLikePost() async {
    final current = feed.value;
    if (current == null) return;
    if (isLikingPost.value) return;

    final alreadyLiked = current.isLiked == true;
    final nextLike = !alreadyLiked;

    isLikingPost.value = true;

    try {
      final res = await feedsApi.likePost(
        postPk: current.post.pk,
        like: nextLike,
      );

      if (res == null) {
        return;
      }

      final oldLikes = current.post.likes;
      int newLikes = oldLikes;

      if (nextLike && !alreadyLiked) {
        newLikes = oldLikes + 1;
      } else if (!nextLike && alreadyLiked && oldLikes > 0) {
        newLikes = oldLikes - 1;
      }

      current.post.likes = newLikes;

      feed.value = FeedModel(
        post: current.post,
        comments: current.comments,
        artworkMetadata: current.artworkMetadata,
        repost: current.repost,
        isLiked: res.like,
        isReport: current.isReport,
        permissions: current.permissions,
      );

      if (nextLike) {
        Biyard.info("Success to like post");
      } else {
        Biyard.info("Success to unlike post");
      }

      feedsService.updateDetail(feed.value!);
    } catch (e, s) {
      logger.e('Failed to toggle like post: $e', stackTrace: s);

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

  Future<void> reportPost({required String postPk}) async {
    try {
      await reportApi.reportPost(postPk: postPk);
      Biyard.info('Reported successfully.');

      final detail = await feedsService.fetchDetail(postPk, forceRefresh: true);
      feed.value = detail;
    } catch (e) {
      logger.e('reportPost error: $e');
      Biyard.error('Report Failed', 'Failed to report. Please try again.');
    }
  }

  Future<void> reportPostComment({
    required String postPk,
    required String commentSk,
  }) async {
    try {
      await reportApi.reportPostComment(postPk: postPk, commentSk: commentSk);
      Biyard.info('Reported successfully.');

      final detail = await feedsService.fetchDetail(postPk, forceRefresh: true);
      feed.value = detail;
    } catch (e) {
      logger.e('reportPost error: $e');
      Biyard.error('Report Failed', 'Failed to report. Please try again.');
    }
  }

  bool isLikingCommentOf(String commentSk) {
    return likingCommentOf[commentSk] ?? false;
  }
}
