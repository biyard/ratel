import 'package:ratel/exports.dart';

class AnalyzesScreen extends GetWidget<AnalyzesController> {
  const AnalyzesScreen({super.key});

  @override
  Widget build(BuildContext context) {
    final space = controller.space;
    return Layout<AnalyzesController>(
      scrollable: false,
      child: space?.isAdmin ?? false
          ? const AnalyzesCreatorScreen()
          : const SizedBox(),
    );
  }
}
