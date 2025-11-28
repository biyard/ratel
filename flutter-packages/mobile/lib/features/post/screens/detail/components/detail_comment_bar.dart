import 'package:ratel/exports.dart';

class DetailCommentBar extends StatelessWidget {
  const DetailCommentBar({
    super.key,
    required this.bottomInset,
    required this.comments,
  });

  final double bottomInset;
  final List<PostCommentModel> comments;

  void _openCommentSheet(BuildContext context) {
    showModalBottomSheet(
      context: context,
      isScrollControlled: true,
      enableDrag: true,
      backgroundColor: Colors.transparent,
      builder: (_) {
        return FractionallySizedBox(
          heightFactor: 0.7,
          child: _CommentBottomSheet(comments: comments),
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
  const _CommentBottomSheet({required this.comments});

  final List<PostCommentModel> comments;

  @override
  State<_CommentBottomSheet> createState() => _CommentBottomSheetState();
}

class _CommentBottomSheetState extends State<_CommentBottomSheet> {
  final _controller = TextEditingController();
  final _focusNode = FocusNode();

  @override
  void initState() {
    super.initState();
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
                itemCount: widget.comments.length,
                itemBuilder: (context, index) {
                  final c = widget.comments[index];
                  return Column(
                    children: [
                      _CommentItem(comment: c),
                      30.vgap,
                    ],
                  );
                },
              ),
            ),

            const SizedBox(height: 10),

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
                          hintText: 'Hi...',
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
                      onTap: () {
                        final text = _controller.text.trim();
                        if (text.isEmpty) return;

                        logger.d('send comment: $text');

                        _controller.clear();
                      },
                      child: const Icon(
                        Icons.arrow_upward_rounded,
                        color: Color(0xFFD4D4D4),
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

class _CommentItem extends StatelessWidget {
  const _CommentItem({required this.comment});

  final PostCommentModel comment;

  String _relativeTime(int millis) {
    final dt = DateTime.fromMillisecondsSinceEpoch(
      millis * 1000,
      isUtc: true,
    ).toLocal();
    final now = DateTime.now();
    final diff = now.difference(dt);

    if (diff.inDays >= 7) {
      final w = (diff.inDays / 7).floor();
      return '${w}w ago';
    }
    if (diff.inDays >= 1) return '${diff.inDays}d ago';
    if (diff.inHours >= 1) return '${diff.inHours}h ago';
    if (diff.inMinutes >= 1) return '${diff.inMinutes}m ago';
    return 'now';
  }

  String _plainContent(String raw) {
    final noTags = raw.replaceAll(RegExp(r'<[^>]*>'), '');
    return noTags.trim();
  }

  @override
  Widget build(BuildContext context) {
    final textTheme = Theme.of(context).textTheme;
    final content = _plainContent(comment.content);
    final timeText = _relativeTime(comment.updatedAt);

    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Row(
          crossAxisAlignment: CrossAxisAlignment.center,
          children: [
            RoundContainer(
              width: 24,
              height: 24,
              radius: 118.5,
              imageUrl: comment.authorProfileUrl.isNotEmpty
                  ? comment.authorProfileUrl
                  : defaultProfileImage,
              color: null,
              alignment: Alignment.center,
              child: null,
            ),
            10.gap,
            Expanded(
              child: Row(
                children: [
                  Row(
                    children: [
                      Text(
                        comment.authorDisplayName,
                        style: textTheme.bodyMedium?.copyWith(
                          fontWeight: FontWeight.w500,
                          fontSize: 16,
                          height: 24 / 16,
                          letterSpacing: 0.5,
                          color: Colors.white,
                        ),
                      ),
                      4.gap,
                      SvgPicture.asset(Assets.badge, width: 20, height: 20),
                    ],
                  ),
                  const Spacer(),
                  Text(
                    timeText,
                    style: textTheme.bodySmall?.copyWith(
                      fontSize: 12,
                      color: const Color(0xFF737373),
                    ),
                  ),
                ],
              ),
            ),
          ],
        ),
        4.vgap,
        Padding(
          padding: const EdgeInsets.only(left: 34), // 24 avatar + 10 gap
          child: Text(
            content,
            style: textTheme.bodyMedium?.copyWith(
              fontSize: 15,
              height: 24 / 15,
              letterSpacing: 0.5,
              color: Colors.white,
            ),
          ),
        ),
        10.vgap,
        Padding(
          padding: const EdgeInsets.only(left: 34),
          child: Row(
            children: [
              Row(
                children: [
                  SvgPicture.asset(
                    Assets.thumbs,
                    width: 20,
                    height: 20,
                    colorFilter: const ColorFilter.mode(
                      Color(0xFF737373),
                      BlendMode.srcIn,
                    ),
                  ),
                  5.gap,
                  Text(
                    comment.likes.toString(),
                    style: textTheme.bodySmall?.copyWith(
                      fontSize: 15,
                      color: Colors.white,
                    ),
                  ),
                ],
              ),
              20.gap,
              Row(
                children: [
                  SvgPicture.asset(
                    Assets.roundBubble,
                    width: 20,
                    height: 20,
                    colorFilter: const ColorFilter.mode(
                      Color(0xFF737373),
                      BlendMode.srcIn,
                    ),
                  ),
                  5.gap,
                  Text(
                    comment.replies.toString(),
                    style: textTheme.bodySmall?.copyWith(
                      fontSize: 15,
                      color: Colors.white,
                    ),
                  ),
                ],
              ),
            ],
          ),
        ),
      ],
    );
  }
}
