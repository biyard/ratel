import 'package:ratel/exports.dart';

class SelectTopicScreen extends GetWidget<SelectTopicController> {
  const SelectTopicScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Layout<SelectTopicController>(child: Text("Select Topic"));
  }
}
