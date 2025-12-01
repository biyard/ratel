import 'package:ratel/exports.dart';

class OverviewScreen extends GetWidget<OverviewController> {
  const OverviewScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Layout<OverviewController>(
      scrollable: false,
      child: Text("Space Overview Screen"),
    );
  }
}
