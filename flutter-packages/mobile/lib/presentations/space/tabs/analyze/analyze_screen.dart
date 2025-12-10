import 'package:ratel/exports.dart';

class AnalyzeScreen extends GetWidget<AnalyzeController> {
  const AnalyzeScreen({super.key});

  @override
  Widget build(BuildContext context) {
    final space = controller.space;
    return Layout<AnalyzeController>(
      scrollable: false,
      child: space?.isAdmin ?? false
          ? const AnalyzeCreatorScreen()
          : const SizedBox(),
    );
  }
}
