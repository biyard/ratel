import 'package:intl/intl.dart';
import 'package:ratel/exports.dart';

class PollTab extends StatelessWidget {
  const PollTab({
    super.key,
    required this.space,
    required this.sheetBottom,
    required this.scrollBottomPadding,
  });

  final SpaceModel space;
  final double sheetBottom;
  final double scrollBottomPadding;

  @override
  Widget build(BuildContext context) {
    final surveys = [...space.surveys]
      ..sort((a, b) => a.startedAt.compareTo(b.startedAt));

    if (surveys.isEmpty) {
      return Padding(
        padding: EdgeInsets.only(bottom: scrollBottomPadding),
        child: const Center(
          child: Text(
            'No surveys yet.',
            style: TextStyle(color: Colors.white70),
          ),
        ),
      );
    }

    return MediaQuery.removePadding(
      context: context,
      removeTop: true,
      child: ListView.separated(
        padding: EdgeInsets.fromLTRB(10, 0, 10, scrollBottomPadding),
        itemCount: surveys.length,
        separatorBuilder: (_, __) => Padding(
          padding: const EdgeInsets.symmetric(vertical: 10),
          child: Container(),
        ),
        itemBuilder: (context, index) {
          final m = surveys[index];
          return _SurveyTile(model: m);
        },
      ),
    );
  }
}

class _SurveyTile extends StatelessWidget {
  const _SurveyTile({required this.model});
  final SurveyModel model;

  String get _statusText {
    switch (model.status) {
      case ProjectStatus.ready:
        return 'Ready';
      case ProjectStatus.inProgress:
        return 'Start';
      case ProjectStatus.finish:
        return 'Ended';
    }
  }

  Color get _statusColor {
    switch (model.status) {
      case ProjectStatus.ready:
        return Colors.white;
      case ProjectStatus.inProgress:
        return Colors.white;
      case ProjectStatus.finish:
        return AppColors.btnPDisabledText;
    }
  }

  bool get _tappable => model.status == ProjectStatus.inProgress;

  String _fmtDue(int sec) => DateFormat(
    'MMM d',
  ).format(DateTime.fromMillisecondsSinceEpoch(sec * 1000));

  @override
  Widget build(BuildContext context) {
    const title = 'Final survey';
    final questionsCnt = model.questions.length;

    return InkWell(
      onTap: _tappable ? () {} : null,
      borderRadius: BorderRadius.circular(8),
      child: Padding(
        padding: const EdgeInsets.symmetric(vertical: 6),
        child: Row(
          crossAxisAlignment: CrossAxisAlignment.center,
          children: [
            Expanded(
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Row(
                    children: [
                      const Text(
                        'SURVEY',
                        style: TextStyle(
                          color: AppColors.btnPDisabled,
                          fontWeight: FontWeight.w500,
                          fontSize: 11,
                          height: 1.3,
                        ),
                      ),
                      5.gap,
                      Text(
                        '$questionsCnt QUESTIONS',
                        style: const TextStyle(
                          color: Color(0xffd4d4d4),
                          fontWeight: FontWeight.w700,
                          fontSize: 11,
                          height: 1.3,
                        ),
                      ),
                    ],
                  ),
                  5.vgap,

                  const Text(
                    title,
                    maxLines: 1,
                    overflow: TextOverflow.ellipsis,
                    style: TextStyle(
                      color: Colors.white,
                      fontWeight: FontWeight.w600,
                      fontSize: 14,
                      height: 1.2,
                    ),
                  ),

                  // if (model.endedAt > 0) ...[
                  //   const SizedBox(height: 2),
                  //   Text(
                  //     'by ${_fmtDue(model.endedAt)}',
                  //     style: const TextStyle(
                  //       color: AppColors.neutral500,
                  //       fontSize: 11,
                  //       height: 1.3,
                  //     ),
                  //   ),
                  // ],
                ],
              ),
            ),

            10.gap,

            SizedBox(
              width: 100,
              child: Center(
                child: Text(
                  _statusText,
                  style: TextStyle(
                    color: _statusColor,
                    fontWeight: FontWeight.w600,
                    fontSize: 14,
                    height: 1.2,
                  ),
                ),
              ),
            ),
          ],
        ),
      ),
    );
  }
}
