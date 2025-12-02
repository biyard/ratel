import 'package:ratel/exports.dart';

class OverviewViewerScreen extends GetWidget<OverviewViewerController> {
  const OverviewViewerScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Layout<OverviewViewerController>(
      scrollable: false,
      child: Text("Space Overview Viewer Screen"),
    );
  }
}
