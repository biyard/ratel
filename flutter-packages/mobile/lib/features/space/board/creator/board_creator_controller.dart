import 'package:ratel/exports.dart';

class BoardCreatorController extends BaseController {
  final reportApi = Get.find<ReportApi>();
  final userService = Get.find<UserService>();
  final SpaceBoardsApi _boardsApi = Get.find<SpaceBoardsApi>();
  late final String spacePk;
  late final String postPk;
  Rx<UserModel> get user => userService.user;
  final post = Rxn<SpacePostModel>();
  final isLoading = false.obs;
  final comments = <SpacePostCommentModel>[].obs;
  final isLoadingComments = false.obs;
  final isLoadingMoreComments = false.obs;
  String? _commentsBookmark;
  bool get hasMoreComments =>
      _commentsBookmark != null && _commentsBookmark!.isNotEmpty;
  @override
  void onInit() {
    super.onInit();
    final sk = Get.parameters['spacePk'];
    final pk = Get.parameters['postPk'];
    if (sk == null || pk == null) {
      logger.e(
        'BoardCreatorController: spacePk/postPk is null. '
        'route: ${Get.currentRoute}',
      );
      Biyard.error(
        'Invalid route',
        'Unable to open this board. Please try again from the space.',
      );
      return;
    }
    spacePk = Uri.decodeComponent(sk);
    postPk = Uri.decodeComponent(pk);
    logger.d(
      'BoardCreatorController initialized '
      'spacePk=$spacePk, postPk=$postPk',
    );
    _loadPost();
    loadComments(reset: true);
  }

  Future<void> _loadPost() async {
    if (isLoading.value) return;
    try {
      isLoading.value = true;
      final res = await _boardsApi.getPost(spacePk, postPk);
      post.value = res;
      logger.d(
        'BoardCreatorController: loaded post '
        'pk=${res.pk}, title=${res.title}',
      );
    } catch (e) {
      logger.e(
        'BoardCreatorController: failed to load post '
        'spacePk=$spacePk postPk=$postPk: $e',
      );
      Biyard.error(
        'Failed to load post',
        'Failed to load this board. Please try again later.',
      );
    } finally {
      isLoading.value = false;
    }
  }

  Future<void> refresh() async {
    await Future.wait([_loadPost(), loadComments(reset: true)]);
  }

  Future<void> loadComments({bool reset = false}) async {
    if (isLoadingComments.value) return;
    try {
      isLoadingComments.value = true;
      if (reset) {
        _commentsBookmark = null;
        comments.clear();
      }
      final res = await _boardsApi.listComments(
        spacePk,
        postPk,
        bookmark: _commentsBookmark,
      );
      _commentsBookmark = res.bookmark;
      if (reset) {
        comments.assignAll(res.items);
      } else {
        comments.addAll(res.items);
      }
      logger.d(
        'BoardCreatorController: loaded comments '
        'count=${res.items.length}, bookmark=$_commentsBookmark',
      );
    } catch (e) {
      logger.e(
        'BoardCreatorController: failed to load comments '
        'spacePk=$spacePk postPk=$postPk: $e',
      );
      Biyard.error(
        'Failed to load comments',
        'Unable to load comments right now. Please try again later.',
      );
    } finally {
      isLoadingComments.value = false;
    }
  }

  Future<void> loadMoreComments() async {
    if (!hasMoreComments || isLoadingMoreComments.value) return;
    try {
      isLoadingMoreComments.value = true;
      final res = await _boardsApi.listComments(
        spacePk,
        postPk,
        bookmark: _commentsBookmark,
      );
      _commentsBookmark = res.bookmark;
      comments.addAll(res.items);
      logger.d(
        'BoardCreatorController: loaded more comments '
        'added=${res.items.length}, bookmark=$_commentsBookmark',
      );
    } catch (e) {
      logger.e(
        'BoardCreatorController: failed to load more comments '
        'spacePk=$spacePk postPk=$postPk: $e',
      );
      Biyard.error(
        'Failed to load more comments',
        'Unable to load more comments. Please try again later.',
      );
    } finally {
      isLoadingMoreComments.value = false;
    }
  }

  Future<void> addComment(String content) async {
    if (content.trim().isEmpty) {
      Biyard.error(
        'Empty comment',
        'Please enter a comment before submitting.',
      );
      return;
    }
    try {
      final res = await _boardsApi.addComment(spacePk, postPk, content);
      comments.insert(0, res);
      logger.d(
        'BoardCreatorController: added comment '
        'sk=${res.sk}, content=${res.content}',
      );
    } catch (e) {
      logger.e(
        'BoardCreatorController: failed to add comment '
        'spacePk=$spacePk postPk=$postPk: $e',
      );
      Biyard.error(
        'Failed to add comment',
        'Could not submit your comment. Please try again.',
      );
    }
  }

  Future<void> toggleLike(SpacePostCommentModel comment) async {
    final targetSk = comment.sk;
    final currentLiked = comment.liked;
    final newLiked = !currentLiked;
    logger.d("current liked: $currentLiked $newLiked");
    final idx = comments.indexWhere((c) => c.sk == targetSk);
    if (idx < 0) return;
    final old = comments[idx];
    final updated = SpacePostCommentModel(
      pk: old.pk,
      sk: old.sk,
      createdAt: old.createdAt,
      updatedAt: old.updatedAt,
      content: old.content,
      likes: newLiked ? old.likes + 1 : (old.likes > 0 ? old.likes - 1 : 0),
      reports: old.reports,
      replies: old.replies,
      parentCommentSk: old.parentCommentSk,
      authorPk: old.authorPk,
      authorDisplayName: old.authorDisplayName,
      authorUsername: old.authorUsername,
      authorProfileUrl: old.authorProfileUrl,
      liked: newLiked,
      isReport: old.isReport,
    );
    comments[idx] = updated;

    try {
      await _boardsApi.likeComment(spacePk, postPk, targetSk, like: newLiked);
      logger.d(
        'BoardCreatorController: toggled like on comment '
        'sk=$targetSk, liked=$newLiked',
      );
    } catch (e) {
      logger.e(
        'BoardCreatorController: failed to toggle like '
        'spacePk=$spacePk postPk=$postPk commentSk=$targetSk liked=$newLiked: $e',
      );
      comments[idx] = old;
      Biyard.error(
        'Failed to update like',
        'Could not update the like status. Please try again.',
      );
    }
  }

  Future<void> deleteComment(SpacePostCommentModel comment) async {
    final targetSk = comment.sk;
    final idx = comments.indexWhere((c) => c.sk == targetSk);
    if (idx < 0) return;
    final backup = List<SpacePostCommentModel>.from(comments);
    comments.removeAt(idx);
    try {
      await _boardsApi.deleteComment(spacePk, postPk, targetSk);
      logger.d('BoardCreatorController: deleted comment sk=$targetSk');
    } catch (e) {
      logger.e(
        'BoardCreatorController: failed to delete comment '
        'spacePk=$spacePk postPk=$postPk commentSk=$targetSk: $e',
      );
      comments.assignAll(backup);
      Biyard.error(
        'Failed to delete comment',
        'Could not delete this comment. Please try again.',
      );
    }
  }

  Future<void> updateComment(
    SpacePostCommentModel comment,
    String newContent,
  ) async {
    final targetSk = comment.sk;
    final trimmed = newContent.trim();
    if (trimmed.isEmpty) {
      Biyard.error('Empty content', 'Comment content cannot be empty.');
      return;
    }
    final idx = comments.indexWhere((c) => c.sk == targetSk);
    if (idx < 0) return;
    final old = comments[idx];
    final now = DateTime.now().millisecondsSinceEpoch ~/ 1000;
    final updated = SpacePostCommentModel(
      pk: old.pk,
      sk: old.sk,
      createdAt: old.createdAt,
      updatedAt: now,
      content: trimmed,
      likes: old.likes,
      reports: old.reports,
      replies: old.replies,
      parentCommentSk: old.parentCommentSk,
      authorPk: old.authorPk,
      authorDisplayName: old.authorDisplayName,
      authorUsername: old.authorUsername,
      authorProfileUrl: old.authorProfileUrl,
      liked: old.liked,
      isReport: old.isReport,
    );
    comments[idx] = updated;
    try {
      await _boardsApi.updateComment(spacePk, postPk, targetSk, trimmed);
      logger.d('BoardCreatorController: updated comment sk=$targetSk');
    } catch (e) {
      logger.e(
        'BoardCreatorController: failed to update comment '
        'spacePk=$spacePk postPk=$postPk commentSk=$targetSk: $e',
      );
      comments[idx] = old;
      Biyard.error(
        'Failed to update comment',
        'Could not update this comment. Please try again.',
      );
    }
  }

  Future<void> reportSpacePost({
    required String spacePk,
    required String spacePostPk,
  }) async {
    try {
      await reportApi.reportSpacePost(
        spacePk: spacePk,
        spacePostPk: spacePostPk,
      );

      Biyard.info('Reported successfully.');
      await _loadPost();
    } catch (e) {
      logger.e('reportSpacePost error: $e');
      Biyard.error('Report Failed', 'Failed to report. Please try again.');
    }
  }

  Future<void> reportSpaceComment({
    required String spacePostPk,
    required String commentSk,
  }) async {
    try {
      await reportApi.reportSpaceComment(
        spacePostPk: spacePostPk,
        commentSk: commentSk,
      );

      Biyard.info('Reported successfully.');
      await loadComments(reset: true);
    } catch (e) {
      logger.e('reportSpacePostComment error: $e');
      Biyard.error('Report Failed', 'Failed to report. Please try again.');
    }
  }
}
