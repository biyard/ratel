import 'package:ratel/exports.dart';

class NotificationScreen extends GetWidget<NotificationController> {
  const NotificationScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Layout<NotificationController>(
      child: Column(
        mainAxisAlignment: MainAxisAlignment.start,
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [Text("Notification", style: TextStyle(color: Colors.white))],
      ),
    );
  }
}
