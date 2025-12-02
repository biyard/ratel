import 'package:ratel/exports.dart';

class AnalyzeScreen extends GetWidget<AnalyzeController> {
  const AnalyzeScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Layout<AnalyzeController>(
      scrollable: false,
      child: Text("Space Analyze Screen"),
    );
  }
}
