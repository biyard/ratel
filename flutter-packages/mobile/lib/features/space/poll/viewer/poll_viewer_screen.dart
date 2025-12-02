import 'package:ratel/exports.dart';

class PollViewerScreen extends GetWidget<PollViewerController> {
  const PollViewerScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Layout<PollViewerController>(
      scrollable: false,
      child: Text("Space Poll Viewer Screen"),
    );
  }
}
