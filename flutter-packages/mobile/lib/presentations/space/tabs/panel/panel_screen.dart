import 'package:ratel/exports.dart';

class PanelScreen extends GetWidget<PanelController> {
  const PanelScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Layout<PanelController>(
      scrollable: false,
      child: Text("Space Panel Screen"),
    );
  }
}
