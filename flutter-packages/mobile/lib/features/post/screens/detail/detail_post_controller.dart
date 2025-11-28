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

  Future<void> toggleLikeComment({
    required String commentSk,
    required bool like,
  }) async {
    final current = feed.value;
    if (current == null) return;

    if (likingCommentOf[commentSk] == true) return;
    likingCommentOf[commentSk] = true;
    likingCommentOf.refresh();

    try {
      await feedsApi.likeComment(
        postPk: current.post.pk,
        commentSk: commentSk,
        like: like,
      );
    } catch (e, s) {
      logger.e('Failed to like/unlike comment $commentSk: $e', stackTrace: s);
    } finally {
      likingCommentOf[commentSk] = false;
      likingCommentOf.refresh();
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
