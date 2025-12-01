import 'package:ratel/exports.dart';
import 'package:ratel/features/post/screens/detail/components/comment_item.dart';

class DetailCommentBar extends StatelessWidget {
  const DetailCommentBar({
    super.key,
    required this.bottomInset,
    required this.comments,
    required this.onSendRootComment,
    required this.onSendReply,
    required this.repliesOf,
    required this.isRepliesLoadingOf,
    required this.isLikingCommentOf,
    required this.isCommentLiked,
    required this.onToggleLikeComment,
  });

  final double bottomInset;
  final List<PostCommentModel> comments;

  final Future<PostCommentModel?> Function(String text) onSendRootComment;
  final Future<void> Function(String parentCommentSk, String text) onSendReply;

  final List<PostCommentModel> Function(String commentSk) repliesOf;
  final bool Function(String commentSk) isRepliesLoadingOf;

  final bool Function(String commentSk) isLikingCommentOf;
  final bool Function(String commentSk, {bool fallback}) isCommentLiked;
  final Future<void> Function(String commentSk) onToggleLikeComment;

  void _openCommentSheet(BuildContext context) {
    showModalBottomSheet(
      context: context,
      isScrollControlled: true,
      enableDrag: true,
      backgroundColor: Colors.transparent,
      builder: (_) {
        return FractionallySizedBox(
          heightFactor: 0.7,
          child: _CommentBottomSheet(
            comments: comments,
            onSendRootComment: onSendRootComment,
            onSendReply: onSendReply,
            repliesOf: repliesOf,
            isRepliesLoadingOf: isRepliesLoadingOf,
            isLikingCommentOf: isLikingCommentOf,
            isCommentLiked: isCommentLiked,
            onToggleLikeComment: onToggleLikeComment,
          ),
        );
      },
    );
  }

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: EdgeInsets.fromLTRB(15, 0, 15, 30 + bottomInset),
      child: GestureDetector(
        onTap: () => _openCommentSheet(context),
        child: RoundContainer(
          radius: 100,
          width: double.infinity,
          color: const Color(0xFF101010),
          padding: const EdgeInsets.fromLTRB(14, 12, 14, 12),
          child: Row(
            children: [
              SvgPicture.asset(
                Assets.add,
                width: 24,
                height: 24,
                colorFilter: const ColorFilter.mode(
                  Colors.white,
                  BlendMode.srcIn,
                ),
              ),
              const SizedBox(width: 10),
              Text(
                'Add a comment',
                style: Theme.of(context).textTheme.bodyMedium?.copyWith(
                  fontSize: 16,
                  fontWeight: FontWeight.w500,
                  color: const Color(0xFF404040),
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}

class _CommentBottomSheet extends StatefulWidget {
  const _CommentBottomSheet({
    required this.comments,
    required this.onSendRootComment,
    required this.onSendReply,
    required this.repliesOf,
    required this.isRepliesLoadingOf,
    required this.isLikingCommentOf,
    required this.isCommentLiked,
    required this.onToggleLikeComment,
  });

  final List<PostCommentModel> comments;

  final Future<PostCommentModel?> Function(String text) onSendRootComment;
  final Future<void> Function(String parentCommentSk, String text) onSendReply;

  final List<PostCommentModel> Function(String commentSk) repliesOf;
  final bool Function(String commentSk) isRepliesLoadingOf;

  final bool Function(String commentSk) isLikingCommentOf;
  final bool Function(String commentSk, {bool fallback}) isCommentLiked;
  final Future<void> Function(String commentSk) onToggleLikeComment;

  @override
  State<_CommentBottomSheet> createState() => _CommentBottomSheetState();
}

class _CommentBottomSheetState extends State<_CommentBottomSheet> {
  final _controller = TextEditingController();
  final _focusNode = FocusNode();

  PostCommentModel? _replyTarget;
  bool _sending = false;

  late List<PostCommentModel> _comments;

  @override
  void initState() {
    super.initState();
    _comments = List<PostCommentModel>.from(widget.comments);
    WidgetsBinding.instance.addPostFrameCallback((_) {
      _focusNode.requestFocus();
    });
  }

  @override
  void dispose() {
    _controller.dispose();
    _focusNode.dispose();
    super.dispose();
  }

  String _plainContent(String raw) {
    final noTags = raw.replaceAll(RegExp(r'<[^>]*>'), '');
    return noTags.trim();
  }

  Future<void> _handleSend() async {
    final text = _controller.text.trim();
    if (text.isEmpty || _sending) return;

    _sending = true;
    setState(() {});

    try {
      if (_replyTarget == null) {
        final created = await widget.onSendRootComment(text);
        if (created != null) {
          setState(() {
            _comments.insert(0, created);
          });
        }
      } else {
        await widget.onSendReply(_replyTarget!.sk, text);
      }

      _controller.clear();
      setState(() {
        _replyTarget = null;
      });
    } catch (e, s) {
      logger.e('Failed to send comment: $e', stackTrace: s);
    } finally {
      _sending = false;
      setState(() {});
    }
  }

  void _setReplyTarget(PostCommentModel c) {
    setState(() {
      _replyTarget = c;
    });
  }

  @override
  Widget build(BuildContext context) {
    final bottomInset = MediaQuery.of(context).viewInsets.bottom;

    return Container(
      padding: EdgeInsets.only(bottom: bottomInset),
      decoration: const BoxDecoration(
        color: Color(0xFF29292F),
        borderRadius: BorderRadius.vertical(top: Radius.circular(20)),
        border: Border(top: BorderSide(color: Color(0xFF3E3E4A))),
        boxShadow: [
          BoxShadow(
            offset: Offset(0, -4),
            blurRadius: 20,
            spreadRadius: 10,
            color: Color.fromARGB(128, 10, 10, 10),
          ),
        ],
      ),
      child: SafeArea(
        top: false,
        child: Column(
          children: [
            const SizedBox(height: 8),
            Container(
              width: 50,
              height: 5,
              decoration: BoxDecoration(
                color: const Color(0xFF6B6B6D),
                borderRadius: BorderRadius.circular(10),
              ),
            ),
            const SizedBox(height: 16),
            Padding(
              padding: const EdgeInsets.symmetric(horizontal: 20),
              child: Align(
                alignment: Alignment.centerLeft,
                child: Text(
                  'Comments',
                  style: Theme.of(context).textTheme.titleSmall?.copyWith(
                    fontWeight: FontWeight.w700,
                    fontSize: 14,
                    color: Colors.white,
                  ),
                ),
              ),
            ),
            const SizedBox(height: 16),
            Expanded(
              child: ListView.builder(
                padding: const EdgeInsets.symmetric(horizontal: 20),
                itemCount: _comments.length,
                itemBuilder: (context, index) {
                  final c = _comments[index];
                  return Column(
                    children: [
                      CommentItem(
                        comment: c,
                        onReply: _setReplyTarget,
                        repliesOf: widget.repliesOf,
                        isRepliesLoadingOf: widget.isRepliesLoadingOf,
                        isLikingCommentOf: widget.isLikingCommentOf,
                        isCommentLiked: widget.isCommentLiked,
                        onToggleLikeComment: widget.onToggleLikeComment,
                      ),
                      30.vgap,
                    ],
                  );
                },
              ),
            ),
            const SizedBox(height: 10),
            if (_replyTarget != null) ...[
              Padding(
                padding: const EdgeInsets.symmetric(horizontal: 20),
                child: RoundContainer(
                  radius: 10,
                  width: double.infinity,
                  color: const Color(0xFF2563EB),
                  padding: const EdgeInsets.symmetric(
                    horizontal: 12,
                    vertical: 8,
                  ),
                  child: Row(
                    children: [
                      const Icon(
                        Icons.subdirectory_arrow_left_rounded,
                        color: Colors.white,
                        size: 18,
                      ),
                      8.gap,
                      RoundContainer(
                        width: 20,
                        height: 20,
                        radius: 118.5,
                        imageUrl: _replyTarget!.authorProfileUrl.isNotEmpty
                            ? _replyTarget!.authorProfileUrl
                            : defaultProfileImage,
                        color: null,
                        alignment: Alignment.center,
                        child: null,
                      ),
                      8.gap,
                      Expanded(
                        child: Text(
                          _plainContent(_replyTarget!.content),
                          maxLines: 1,
                          overflow: TextOverflow.ellipsis,
                          style: Theme.of(context).textTheme.bodySmall
                              ?.copyWith(
                                fontSize: 14,
                                fontWeight: FontWeight.w500,
                                color: Colors.white,
                              ),
                        ),
                      ),
                      GestureDetector(
                        onTap: () {
                          setState(() {
                            _replyTarget = null;
                          });
                        },
                        child: const Icon(
                          Icons.close_rounded,
                          color: Colors.white,
                          size: 18,
                        ),
                      ),
                    ],
                  ),
                ),
              ),
            ],
            Padding(
              padding: const EdgeInsets.symmetric(horizontal: 20, vertical: 16),
              child: RoundContainer(
                radius: 100,
                width: double.infinity,
                color: const Color(0xFF101010),
                padding: const EdgeInsets.symmetric(
                  horizontal: 14,
                  vertical: 12,
                ),
                border: Border.all(color: const Color(0xFF3E3E4A)),
                child: Row(
                  children: [
                    SvgPicture.asset(
                      Assets.add,
                      width: 24,
                      height: 24,
                      colorFilter: const ColorFilter.mode(
                        Color(0xFFD4D4D4),
                        BlendMode.srcIn,
                      ),
                    ),
                    const SizedBox(width: 10),
                    Expanded(
                      child: TextField(
                        controller: _controller,
                        focusNode: _focusNode,
                        style: Theme.of(context).textTheme.bodyMedium?.copyWith(
                          fontSize: 16,
                          fontWeight: FontWeight.w500,
                          color: Colors.white,
                        ),
                        decoration: const InputDecoration(
                          isDense: true,
                          border: InputBorder.none,
                          hintText: 'Add a comment',
                          hintStyle: TextStyle(
                            fontSize: 16,
                            fontWeight: FontWeight.w500,
                            color: Color(0xFF6B6B6D),
                          ),
                        ),
                      ),
                    ),
                    const SizedBox(width: 10),
                    GestureDetector(
                      onTap: _handleSend,
                      child: SvgPicture.asset(
                        Assets.uploadComment,
                        width: 24,
                        height: 24,
                      ),
                    ),
                  ],
                ),
              ),
            ),
          ],
        ),
      ),
    );
  }
}
