import 'package:ratel/exports.dart';

class HomeScreen extends GetWidget<HomeController> {
  const HomeScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Layout<HomeController>(
      bottomSheet: DraggableScrollableSheet(
        expand: false,
        initialChildSize: 0.2,
        minChildSize: 0.2,
        maxChildSize: 0.6,
        builder: (context, scrollController) {
          return Container(
            decoration: const BoxDecoration(
              color: AppColors.neutral800,
              borderRadius: BorderRadius.vertical(top: Radius.circular(20)),
            ),
            padding: const EdgeInsets.symmetric(horizontal: 20, vertical: 12),
            child: ListView(
              controller: scrollController,
              children: [
                Center(
                  child: Container(
                    width: 36,
                    height: 5,
                    margin: const EdgeInsets.only(bottom: 10),
                    decoration: BoxDecoration(
                      color: AppColors.neutral600,
                      borderRadius: BorderRadius.circular(8),
                    ),
                  ),
                ),
                Row(
                  children: [
                    Profile(
                      width: 24,
                      height: 24,
                      profile: controller.profile.value.profile,
                    ),
                    SizedBox(width: 10),
                    Text(
                      controller.profile.value.nickname,
                      style: TextStyle(
                        color: Colors.white,
                        fontSize: 18,
                        fontWeight: FontWeight.w700,
                      ),
                    ),
                    SizedBox(width: 4),
                    Assets.badgeImage,
                  ],
                ),
                SizedBox(height: 12),
                Container(
                  width: double.infinity,
                  height: 1,
                  color: AppColors.neutral700,
                ),
              ],
            ),
          );
        },
      ),
      child: const Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: const [Text("Home", style: TextStyle(color: Colors.white))],
      ),
    );
  }
}
