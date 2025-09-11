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
          logger.d("model: ${index} ${m.questions[0].title}");
          return _SurveyTile(
            model: m,
            userResponses: space.userResponses,
            isAI: (index == 0) && (surveys.length != 1),
          );
        },
      ),
    );
  }
}

class _SurveyTile extends StatelessWidget {
  const _SurveyTile({
    required this.model,
    required this.userResponses,
    required this.isAI,
  });
  final SurveyModel model;
  final List<SurveyResponse> userResponses;
  final bool isAI;

  String get _statusText {
    if (isAI && userResponses.isNotEmpty) return 'Voted';

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
    if (isAI && userResponses.isNotEmpty) return AppColors.btnPDisabledText;

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

  @override
  Widget build(BuildContext context) {
    const title = 'Final survey';
    final questionsCnt = model.questions.length;
    final spaceController = Get.find<DeliberationSpaceController>();

    return InkWell(
      onTap: _tappable
          ? () {
              if (_statusText != "Start") return;

              spaceController.isSurvey(true);
              spaceController.questions(model.questions);
              spaceController.surveyId(model.id);
            }
          : null,
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

                  if (isAI) ...[
                    Row(
                      mainAxisAlignment: MainAxisAlignment.spaceBetween,
                      children: [
                        Row(
                          mainAxisAlignment: MainAxisAlignment.center,
                          crossAxisAlignment: CrossAxisAlignment.center,
                          children: [
                            SvgPicture.asset(Assets.ai, width: 12, height: 12),
                            5.gap,
                            Text(
                              "Ratel AI",
                              style: TextStyle(
                                fontSize: 14,
                                fontWeight: FontWeight.w600,
                                color: Colors.white,
                              ),
                            ),
                          ],
                        ),
                      ],
                    ),
                  ],
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
