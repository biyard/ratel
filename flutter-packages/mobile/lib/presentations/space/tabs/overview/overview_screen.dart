import 'package:ratel/exports.dart';

class OverviewScreen extends GetWidget<OverviewController> {
  const OverviewScreen({super.key});

  @override
  Widget build(BuildContext context) {
    final space = controller.space;
    return Layout<OverviewController>(
      scrollable: false,
      child: space?.isAdmin ?? false
          ? const OverviewCreatorScreen()
          : const OverviewViewerScreen(),
    );
  }
}
