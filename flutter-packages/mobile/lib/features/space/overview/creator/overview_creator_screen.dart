import 'package:ratel/exports.dart';

class OverviewCreatorScreen extends GetWidget<OverviewCreatorController> {
  const OverviewCreatorScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Layout<OverviewCreatorController>(
      scrollable: false,
      child: Text("Space Overview Creator Screen"),
    );
  }
}
