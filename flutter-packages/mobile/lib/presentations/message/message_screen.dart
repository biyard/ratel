import 'package:ratel/exports.dart';

class MessageScreen extends GetWidget<MessageController> {
  const MessageScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Layout<MessageController>(
      child: Column(
        mainAxisAlignment: MainAxisAlignment.start,
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [Text("Message", style: TextStyle(color: Colors.white))],
      ),
    );
  }
}
