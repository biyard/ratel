import 'dart:math';

import 'package:flutter/rendering.dart';
import 'package:ratel/exports.dart';
import 'package:ratel/features/space/analyze/components/analyze_header.dart';
import 'package:ratel/features/space/analyze/components/objective_answers_view.dart';
import 'package:ratel/features/space/analyze/components/subjective_answers_view.dart';

class AnalyzeCreatorScreen extends GetWidget<AnalyzeCreatorController> {
  const AnalyzeCreatorScreen({super.key});

  @override
  Widget build(BuildContext context) {
    final space = Get.find<SpaceController>();

    return Layout<AnalyzeCreatorController>(
      scrollable: false,
      child: Obx(() {
        if (controller.isLoading.value &&
            (controller.poll.value == null ||
                controller.pollResult.value == null)) {
          return const Center(
            child: SizedBox(
              width: 24,
              height: 24,
              child: CircularProgressIndicator(strokeWidth: 2),
            ),
          );
        }

        final poll = controller.poll.value;
        final result = controller.pollResult.value;

        if (poll == null || result == null) {
          return const SizedBox.shrink();
        }

        final questionCount = min(
          poll.questions.length,
          result.summaries.length,
        );

        return NotificationListener<ScrollNotification>(
          onNotification: (n) {
            space.markScrollActivity();

            if (n.metrics.axis != Axis.vertical) return false;
            if (n is! ScrollUpdateNotification &&
                n is! UserScrollNotification) {
              return false;
            }

            final pixels = n.metrics.pixels;
            final collapsedNow = space.isHeaderCollapsed.value;

            if (!collapsedNow && pixels > SpaceController.collapseThresholdPx) {
              space.isHeaderCollapsed.value = true;
              return false;
            }

            if (collapsedNow) {
              final minExtent = n.metrics.minScrollExtent;
              final atTop = pixels <= (minExtent + 0.5);

              final bool userPullingDown =
                  (n is UserScrollNotification &&
                      n.direction == ScrollDirection.forward) ||
                  (n is ScrollUpdateNotification &&
                      n.dragDetails != null &&
                      (n.scrollDelta ?? 0) < 0);

              if (atTop && userPullingDown) {
                space.isHeaderCollapsed.value = false;
              }
            }

            return false;
          },
          child: ListView.builder(
            physics: const BouncingScrollPhysics(
              parent: AlwaysScrollableScrollPhysics(),
            ),
            itemCount: questionCount + 1,
            itemBuilder: (context, index) {
              if (index == 0) {
                return AnalyzeHeader(poll: poll, result: result);
              }

              final q = poll.questions[index - 1];
              final s = result.summaries[index - 1];

              return _QuestionResultCard(
                index: index - 1,
                question: q,
                summary: s,
              );
            },
          ),
        );
      }),
    );
  }
}

class _QuestionResultCard extends StatelessWidget {
  final int index;
  final QuestionModel question;
  final PollSummary summary;

  const _QuestionResultCard({
    required this.index,
    required this.question,
    required this.summary,
  });

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final totalCount = summary.totalCount;

    return Container(
      margin: const EdgeInsets.symmetric(horizontal: 0, vertical: 8),
      padding: const EdgeInsets.all(16),
      decoration: BoxDecoration(
        borderRadius: BorderRadius.circular(12),
        color: Colors.transparent,
        border: Border.all(color: AppColors.neutral500, width: 1),
      ),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(
            mainAxisAlignment: MainAxisAlignment.spaceBetween,
            children: [
              Expanded(
                child: Text(
                  question.title,
                  style: theme.textTheme.titleMedium?.copyWith(
                    fontWeight: FontWeight.w600,
                    color: AppColors.neutral400,
                  ),
                  overflow: TextOverflow.ellipsis,
                ),
              ),
              8.gap,
              Text(
                '$totalCount responses',
                style: theme.textTheme.bodySmall?.copyWith(
                  color: AppColors.neutral400,
                ),
              ),
            ],
          ),
          10.vgap,
          if (_isChoiceSummary(summary))
            ObjectiveAnswersView(question: question, summary: summary)
          else
            SubjectiveAnswersView(summary: summary),
        ],
      ),
    );
  }

  bool _isChoiceSummary(PollSummary s) {
    return s is SingleChoiceSummary ||
        s is MultipleChoiceSummary ||
        s is CheckboxSummary ||
        s is DropdownSummary ||
        s is LinearScaleSummary;
  }
}
