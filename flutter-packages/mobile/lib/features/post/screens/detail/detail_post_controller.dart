import 'package:ratel/exports.dart';

class DetailPostController extends BaseController {
  final userService = Get.find<UserService>();
  final feedsApi = Get.find<FeedsApi>();
  final feedsService = Get.find<FeedsService>();

  late final String postPk;

  final feed = Rxn<FeedV2Model>();
  final isLoading = false.obs;

  final isSendingRootComment = false.obs;
  final likingCommentOf = <String, bool>{}.obs;

  final isLikingPost = false.obs;
  bool get isPostLiked => feed.value?.isLiked == true;
  int get postLikes => feed.value?.post.likes ?? 0;

  final Rx<UserV2Model> user = UserV2Model(
    pk: '',
    email: '',
    nickname: '',
    profileUrl: '',
    description: '',
    userType: 0,
    username: '',
    followersCount: 0,
    followingsCount: 0,
    theme: 0,
    point: 0,
    referralCode: null,
    phoneNumber: null,
    principal: null,
    evmAddress: null,
    teams: const [],
  ).obs;

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

    _detailSubscription = ever<Map<String, FeedV2Model>>(feedsService.details, (
      map,
    ) {
      final updated = map[postPk];
      if (updated != null) {
        feed.value = updated;
      }
    });

    loadFeed();
    user(userService.user.value);
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
      Biyard.error('Failed to delete post.', 'Please try again later.');
      return;
    }
    Get.back();
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

      feed.refresh();
      feedsService.updateDetail(current);
    } catch (e, s) {
      logger.e('Failed to like/unlike comment $commentSk: $e', stackTrace: s);
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

      feed.value = FeedV2Model(
        post: current.post,
        comments: [created, ...current.comments],
        artworkMetadata: current.artworkMetadata,
        repost: current.repost,
        isLiked: current.isLiked,
        permissions: current.permissions,
      );

      feedsService.updateDetail(feed.value!);
      return created;
    } catch (e, s) {
      logger.e('Failed to add comment: $e', stackTrace: s);
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

      feed.value = FeedV2Model(
        post: current.post,
        comments: current.comments,
        artworkMetadata: current.artworkMetadata,
        repost: current.repost,
        isLiked: res.like,
        permissions: current.permissions,
      );

      feedsService.updateDetail(feed.value!);
    } catch (e, s) {
      logger.e('Failed to toggle like post: $e', stackTrace: s);
    } finally {
      isLikingPost.value = false;
    }
  }

  bool isLikingCommentOf(String commentSk) {
    return likingCommentOf[commentSk] ?? false;
  }
}
