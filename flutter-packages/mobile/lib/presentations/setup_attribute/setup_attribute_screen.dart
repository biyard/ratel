import 'package:ratel/exports.dart';

class SetupAttributeScreen extends GetWidget<SetupAttributeController> {
  const SetupAttributeScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Layout<SetupAttributeController>(
      child: Column(
        mainAxisAlignment: MainAxisAlignment.start,
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text("Setup Attribute", style: TextStyle(color: Colors.white)),
        ],
      ),
    );
  }
}
