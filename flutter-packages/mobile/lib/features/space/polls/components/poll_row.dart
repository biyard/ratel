import 'package:ratel/exports.dart';

class PollRow extends StatelessWidget {
  final PollModel poll;
  final String spacePk;
  final VoidCallback onTap;

  const PollRow({
    super.key,
    required this.poll,
    required this.spacePk,
    required this.onTap,
  });

  String _buttonLabel() {
    switch (poll.status) {
      case PollStatus.notStarted:
        return 'Upcoming';
      case PollStatus.inProgress:
        return 'Start';
      case PollStatus.finish:
        return 'Closed';
      case PollStatus.unknown:
        return 'Unknown';
    }
  }

  bool get _canEnter => poll.status == PollStatus.inProgress;

  String _titleText() {
    String extractId(String v) {
      final i = v.indexOf('#');
      if (i < 0 || i + 1 >= v.length) return v;
      return v.substring(i + 1);
    }

    final spaceId = extractId(spacePk);
    final pollId = extractId(poll.sk);

    if (spaceId == pollId) {
      return 'Pre-poll Survey';
    } else {
      return 'Final survey';
    }
  }

  @override
  Widget build(BuildContext context) {
    final questionCount = poll.questions.length;
    final title = _titleText();

    return SizedBox(
      height: 44,
      child: Row(
        children: [
          Expanded(
            child: SizedBox(
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                mainAxisAlignment: MainAxisAlignment.center,
                children: [
                  Row(
                    mainAxisSize: MainAxisSize.min,
                    children: [
                      Text(
                        'SURVEY',
                        style: TextStyle(
                          fontFamily: 'Raleway',
                          fontWeight: FontWeight.w600,
                          fontSize: 13,
                          height: 20 / 13,
                          color: const Color(0xFF737373),
                        ),
                      ),
                      5.gap,
                      Text(
                        '$questionCount QUESTIONS',
                        style: TextStyle(
                          fontFamily: 'Raleway',
                          fontWeight: FontWeight.w600,
                          fontSize: 13,
                          height: 20 / 13,
                          color: const Color(0xFFD4D4D4),
                        ),
                      ),
                    ],
                  ),
                  0.vgap,
                  Text(
                    title,
                    maxLines: 1,
                    overflow: TextOverflow.ellipsis,
                    style: TextStyle(
                      fontFamily: 'Raleway',
                      fontWeight: FontWeight.w600,
                      fontSize: 16,
                      height: 24 / 16,
                      color: Colors.white,
                    ),
                  ),
                ],
              ),
            ),
          ),
          20.gap,
          SizedBox(
            height: 30,
            child: GestureDetector(
              onTap: _canEnter ? onTap : null,
              child: Container(
                decoration: BoxDecoration(
                  color: _canEnter ? AppColors.primary : AppColors.neutral700,
                  borderRadius: BorderRadius.circular(8),
                ),
                padding: const EdgeInsets.symmetric(
                  horizontal: 30,
                  vertical: 5,
                ),
                alignment: Alignment.center,
                child: Text(
                  _buttonLabel(),
                  style: TextStyle(
                    fontFamily: 'Raleway',
                    fontWeight: FontWeight.w600,
                    fontSize: 13,
                    height: 20 / 13,
                    color: _canEnter
                        ? const Color(0xFF1D1D1D)
                        : const Color(0xFFBDBDBD),
                  ),
                ),
              ),
            ),
          ),
        ],
      ),
    );
  }
}
