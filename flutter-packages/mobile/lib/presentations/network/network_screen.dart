import 'package:ratel/exports.dart';

class NetworkScreen extends GetWidget<NetworkController> {
  const NetworkScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Layout<NetworkController>(
      child: Column(
        mainAxisAlignment: MainAxisAlignment.start,
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [Text("My Network", style: TextStyle(color: Colors.white))],
      ),
    );
  }
}
