import 'package:ratel/exports.dart';

class MyPageScreen extends GetWidget<MyPageController> {
  const MyPageScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Layout<MyPageController>(
      scrollable: false,
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Expanded(
            child: ListView(
              padding: const EdgeInsets.symmetric(horizontal: 0, vertical: 8),
              children: [
                const Header(title: 'My Pages'),
                15.vgap,
                Padding(
                  padding: const EdgeInsets.symmetric(
                    horizontal: 16,
                    vertical: 0,
                  ),
                  child: Column(
                    children: [
                      _MyPageItem(
                        icon: SvgPicture.asset(
                          Assets.editContent,
                          width: 20,
                          height: 20,
                          colorFilter: const ColorFilter.mode(
                            AppColors.neutral500,
                            BlendMode.srcIn,
                          ),
                        ),
                        title: MainLocalization.drafts,
                        subtitle: 'View and edit your drafts',
                        onTap: () {
                          Get.rootDelegate.toNamed(AppRoutes.draftScreen);
                        },
                      ),
                      _MyPageItem(
                        icon: SvgPicture.asset(
                          Assets.file,
                          width: 20,
                          height: 20,
                          colorFilter: const ColorFilter.mode(
                            AppColors.neutral500,
                            BlendMode.srcIn,
                          ),
                        ),
                        title: MainLocalization.posts,
                        subtitle: 'Check all your posts',
                        onTap: () {
                          Get.rootDelegate.toNamed(postScreen);
                        },
                      ),
                      _MyPageItem(
                        icon: SvgPicture.asset(
                          Assets.verification,
                          width: 20,
                          height: 20,
                          colorFilter: const ColorFilter.mode(
                            AppColors.neutral500,
                            BlendMode.srcIn,
                          ),
                        ),
                        title: MainLocalization.verifiedCredential,
                        subtitle: 'Manage your credentials',
                        onTap: () {
                          Get.rootDelegate.toNamed(AppRoutes.verifiedScreen);
                        },
                      ),
                      _MyPageItem(
                        icon: SvgPicture.asset(
                          Assets.setting,
                          width: 20,
                          height: 20,
                          colorFilter: const ColorFilter.mode(
                            AppColors.neutral500,
                            BlendMode.srcIn,
                          ),
                        ),
                        title: MainLocalization.settings,
                        subtitle: 'App and account settings',
                        onTap: () {
                          Get.rootDelegate.toNamed(AppRoutes.settingScreen);
                        },
                      ),
                    ],
                  ),
                ),

                const SizedBox(height: 24),
              ],
            ),
          ),
        ],
      ),
    );
  }
}

class _MyPageItem extends StatelessWidget {
  const _MyPageItem({
    required this.icon,
    required this.title,
    required this.subtitle,
    this.onTap,
  });

  final Widget icon;
  final String title;
  final String subtitle;
  final VoidCallback? onTap;

  @override
  Widget build(BuildContext context) {
    return ListTile(
      onTap: onTap,
      contentPadding: EdgeInsets.zero,
      leading: icon,
      title: Text(
        title,
        style: const TextStyle(
          color: Colors.white,
          fontSize: 16,
          fontWeight: FontWeight.w600,
          height: 1.2,
        ),
      ),
      subtitle: Text(
        subtitle,
        style: const TextStyle(
          color: AppColors.neutral400,
          fontSize: 13,
          height: 1.2,
        ),
      ),
      trailing: const Icon(
        Icons.chevron_right,
        color: AppColors.neutral500,
        size: 20,
      ),
      minVerticalPadding: 10,
    );
  }
}
