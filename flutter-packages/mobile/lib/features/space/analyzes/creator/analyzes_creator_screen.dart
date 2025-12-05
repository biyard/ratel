import 'package:ratel/exports.dart';
import 'package:ratel/features/space/analyzes/components/analyze_row.dart';

class AnalyzesCreatorScreen extends GetWidget<AnalyzesCreatorController> {
  const AnalyzesCreatorScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Layout<AnalyzesCreatorController>(
      scrollable: false,
      child: Obx(() {
        if (controller.isLoading.value && controller.polls.isEmpty) {
          return const Center(
            child: SizedBox(
              width: 24,
              height: 24,
              child: CircularProgressIndicator(strokeWidth: 2),
            ),
          );
        }

        if (controller.polls.isEmpty) {
          return const Center(
            child: Text(
              'No analyzes yet.',
              style: TextStyle(color: Colors.white70, fontSize: 14),
            ),
          );
        }

        return Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            for (int i = 0; i < controller.polls.length; i++) ...[
              AnalyzeRow(
                poll: controller.polls[i],
                spacePk: controller.spacePk,
                onTap: () => controller.onPollTap(controller.polls[i]),
              ),
              if (i != controller.polls.length - 1) 20.vgap,
            ],
          ],
        );
      }),
    );
  }
}
