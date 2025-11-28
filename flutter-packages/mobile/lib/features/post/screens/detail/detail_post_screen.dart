import 'package:ratel/exports.dart';

class DetailPostScreen extends GetWidget<DetailPostController> {
  const DetailPostScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Layout<DetailPostController>(
      scrollable: false,
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Padding(
            padding: const EdgeInsets.all(20.0),
            child: AppTopBar(onBack: () => Get.back(), title: ""),
          ),
        ],
      ),
    );
  }
}
