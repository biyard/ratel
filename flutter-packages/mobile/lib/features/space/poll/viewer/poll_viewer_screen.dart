import 'package:ratel/exports.dart';
import 'package:ratel/features/space/poll/components/poll_question_pager.dart';

class PollViewerScreen extends GetWidget<PollViewerController> {
  const PollViewerScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Layout<PollViewerController>(
      scrollable: false,
      child: Obx(() {
        if (controller.isLoading.value) {
          return const Center(
            child: SizedBox(
              width: 24,
              height: 24,
              child: CircularProgressIndicator(strokeWidth: 2),
            ),
          );
        }

        final poll = controller.poll.value;
        if (poll == null) {
          return const Center(
            child: Text(
              'Poll not found.',
              style: TextStyle(color: Colors.white70),
            ),
          );
        }

        return PollQuestionPager(
          spacePk: controller.spacePk,
          poll: poll,
          isFinished: controller.space?.isFinished ?? false,
          onSubmit: (answers) {
            logger.d("answer: ${answers}");
            controller.respondAnswers(answers);
          },
        );
      }),
    );
  }
}
