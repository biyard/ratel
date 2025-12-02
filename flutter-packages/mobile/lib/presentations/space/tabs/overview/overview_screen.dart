import 'package:ratel/exports.dart';

class OverviewScreen extends GetWidget<OverviewController> {
  const OverviewScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Layout<OverviewController>(
      scrollable: false,
      child: Obx(() {
        final space = controller.space;

        if (space == null) {
          return const Center(
            child: SizedBox(
              width: 24,
              height: 24,
              child: CircularProgressIndicator(strokeWidth: 2),
            ),
          );
        }

        return space.isAdmin
            ? const OverviewCreatorScreen()
            : const OverviewViewerScreen();
      }),
    );
  }
}
