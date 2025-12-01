import 'package:ratel/exports.dart';

class DetailPostController extends BaseController {
  final feedsApi = Get.find<FeedsApi>();

  late final String postPk;

  final feed = Rxn<FeedV2Model>();
  final isLoading = false.obs;

  final replies = <String, List<PostCommentModel>>{}.obs;
  final repliesLoading = <String, bool>{}.obs;

  final isSendingRootComment = false.obs;
  final sendingReplyOf = <String, bool>{}.obs;
  final likingCommentOf = <String, bool>{}.obs;
  final likedOverrideOf = <String, bool>{}.obs;

  final isLikingPost = false.obs;
  bool get isPostLiked => feed.value?.isLiked == true;
  int get postLikes => feed.value?.post.likes ?? 0;

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

    loadFeed();
  }

  bool isCommentLiked(String commentSk, {bool fallback = false}) {
    final override = likedOverrideOf[commentSk];
    if (override != null) {
      return override;
    }

    final current = feed.value;
    if (current != null) {
      for (final c in current.comments) {
        if (c.sk == commentSk) {
          return c.liked == true;
        }
      }
      for (final entry in replies.entries) {
        for (final c in entry.value) {
          if (c.sk == commentSk) {
            return c.liked == true;
          }
        }
      }
    }

    return fallback;
  }

  Future<void> toggleLikeComment({required String commentSk}) async {
    logger.d('Toggling like for comment: $commentSk');
    final current = feed.value;
    if (current == null) return;

    if (likingCommentOf[commentSk] == true) return;

    final currentLiked = isCommentLiked(commentSk, fallback: false);
    final nextLike = !currentLiked;

    likingCommentOf[commentSk] = true;
    likingCommentOf.refresh();

    try {
      final res = await feedsApi.likeComment(
        postPk: current.post.pk,
        commentSk: commentSk,
        like: nextLike,
      );

      if (res == null) return;

      likedOverrideOf[commentSk] = res.liked;
      likedOverrideOf.refresh();

      _applyCommentLikeCountUpdate(
        commentSk: commentSk,
        previousLiked: currentLiked,
        nextLiked: res.liked,
      );
    } catch (e, s) {
      logger.e('Failed to like/unlike comment $commentSk: $e', stackTrace: s);
    } finally {
      likingCommentOf[commentSk] = false;
      likingCommentOf.refresh();
    }
  }

  void _applyCommentLikeCountUpdate({
    required String commentSk,
    required bool previousLiked,
    required bool nextLiked,
  }) {
    final current = feed.value;
    if (current == null) return;

    int delta = 0;
    if (nextLiked && !previousLiked) {
      delta = 1;
    } else if (!nextLiked && previousLiked) {
      delta = -1;
    } else {
      return;
    }

    var updated = false;

    for (final c in current.comments) {
      if (c.sk == commentSk) {
        var newLikes = c.likes + delta;
        if (newLikes < 0) newLikes = 0;
        c.likes = newLikes;
        updated = true;
        break;
      }
    }

    if (!updated) {
      for (final entry in replies.entries) {
        for (final c in entry.value) {
          if (c.sk == commentSk) {
            var newLikes = c.likes + delta;
            if (newLikes < 0) newLikes = 0;
            c.likes = newLikes;
            updated = true;
            break;
          }
        }
        if (updated) break;
      }
    }

    if (updated) {
      feed.refresh();
      replies.refresh();
    }
  }

  Future<void> loadFeed() async {
    try {
      isLoading.value = true;

      final result = await feedsApi.getFeedV2(postPk);
      feed.value = result;
      logger.d("feed results: $result");

      await _loadAllReplies(result);
    } catch (e, s) {
      logger.e('Failed to load feed detail: $e', stackTrace: s);
    } finally {
      isLoading.value = false;
    }
  }

  Future<void> addComment(String text) async {
    final current = feed.value;
    if (current == null) return;
    if (text.trim().isEmpty) return;
    if (isSendingRootComment.value) return;

    isSendingRootComment.value = true;

    final html = _wrapAsDiv(text);

    try {
      final created = await feedsApi.createComment(
        postPk: current.post.pk,
        content: html,
      );
      if (created == null) return;

      feed.value = FeedV2Model(
        post: current.post,
        comments: [created, ...current.comments],
        artworkMetadata: current.artworkMetadata,
        repost: current.repost,
        isLiked: current.isLiked,
        permissions: current.permissions,
      );
    } catch (e, s) {
      logger.e('Failed to add comment: $e', stackTrace: s);
    } finally {
      isSendingRootComment.value = false;
    }
  }

  Future<void> addReply({
    required String parentCommentSk,
    required String text,
  }) async {
    final current = feed.value;
    if (current == null) return;
    if (text.trim().isEmpty) return;

    final html = _wrapAsDiv(text);

    sendingReplyOf[parentCommentSk] = true;
    sendingReplyOf.refresh();

    try {
      final created = await feedsApi.replyToComment(
        postPk: current.post.pk,
        parentCommentSk: parentCommentSk,
        content: html,
      );

      if (created == null) return;

      final currentReplies = replies[parentCommentSk] ?? const [];
      replies[parentCommentSk] = [created, ...currentReplies];
      replies.refresh();
    } catch (e, s) {
      logger.e('Failed to add reply for $parentCommentSk: $e', stackTrace: s);
    } finally {
      sendingReplyOf[parentCommentSk] = false;
      sendingReplyOf.refresh();
    }
  }

  String _wrapAsDiv(String text) {
    final escaped = text
        .replaceAll('&', '&amp;')
        .replaceAll('<', '&lt;')
        .replaceAll('>', '&gt;');
    return '<div>$escaped</div>';
  }

  Future<void> _loadAllReplies(FeedV2Model feed) async {
    final futures = <Future<void>>[];
    final pk = feed.post.pk;

    for (final c in feed.comments) {
      if (c.replies <= 0) continue;
      futures.add(_loadRepliesForComment(postPk: pk, commentSk: c.sk));
    }

    if (futures.isEmpty) return;

    await Future.wait(futures);
  }

  Future<void> _loadRepliesForComment({
    required String postPk,
    required String commentSk,
  }) async {
    try {
      repliesLoading[commentSk] = true;
      final res = await feedsApi.listComments(
        postPk: postPk,
        commentSk: commentSk,
      );
      replies[commentSk] = res.items;
      replies.refresh();
    } catch (e, s) {
      logger.e('Failed to load replies for $commentSk: $e', stackTrace: s);
    } finally {
      repliesLoading[commentSk] = false;
      repliesLoading.refresh();
    }
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
    } catch (e, s) {
      logger.e('Failed to toggle like post: $e', stackTrace: s);
    } finally {
      isLikingPost.value = false;
    }
  }

  List<PostCommentModel> repliesOf(String commentSk) {
    return replies[commentSk] ?? const [];
  }

  bool isRepliesLoadingOf(String commentSk) {
    return repliesLoading[commentSk] ?? false;
  }

  bool isSendingReplyOf(String commentSk) {
    return sendingReplyOf[commentSk] ?? false;
  }

  bool isLikingCommentOf(String commentSk) {
    return likingCommentOf[commentSk] ?? false;
  }
}
