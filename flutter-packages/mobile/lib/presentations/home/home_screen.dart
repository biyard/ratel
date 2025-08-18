import 'package:ratel/exports.dart';

class HomeScreen extends GetWidget<HomeController> {
  const HomeScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Layout<HomeController>(
      child: Padding(
        padding: const EdgeInsets.symmetric(horizontal: 14, vertical: 10),
        child: Obx(
          () => ListView.separated(
            primary: false,
            shrinkWrap: true,
            itemCount: controller.feeds.length,
            separatorBuilder: (_, __) => const SizedBox(height: 10),
            itemBuilder: (_, i) => FeedCard(data: controller.feeds[i]),
          ),
        ),
      ),
    );
  }
}
