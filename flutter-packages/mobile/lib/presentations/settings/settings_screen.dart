import 'package:ratel/exports.dart';

class SettingsScreen extends GetWidget<SettingsController> {
  const SettingsScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Layout<SettingsController>(
      scrollable: false,
      child: Padding(
        padding: const EdgeInsets.symmetric(horizontal: 20),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            SizedBox(
              height: 75,
              child: Padding(
                padding: EdgeInsets.fromLTRB(0, 20, 0, 20),
                child: Row(
                  children: [
                    InkWell(
                      onTap: controller.goBack,
                      child: SvgPicture.asset(
                        Assets.back,
                        width: 20,
                        height: 20,
                      ),
                    ),
                    10.gap,
                    const Text(
                      'My Profile',
                      style: TextStyle(
                        color: Colors.white,
                        fontWeight: FontWeight.w600,
                        fontSize: 18,
                      ),
                    ),
                  ],
                ),
              ),
            ),

            SettingTile(
              leading: const Icon(Icons.logout, color: Colors.white70),
              title: 'Logout',
              subtitle: 'Sign out of your account',
              onTap: controller.logout,
            ),
            Container(
              width: double.infinity,
              height: 1,
              color: Color(0xff2d2d2d),
            ),
          ],
        ),
      ),
    );
  }
}

class SettingTile extends StatelessWidget {
  const SettingTile({
    super.key,
    required this.leading,
    required this.title,
    this.subtitle,
    this.onTap,
  });

  final Widget leading;
  final String title;
  final String? subtitle;
  final VoidCallback? onTap;

  @override
  Widget build(BuildContext context) {
    return InkWell(
      onTap: onTap,
      child: Padding(
        padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 20),
        child: Row(
          children: [
            leading,
            12.gap,
            Expanded(
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text(
                    title,
                    style: const TextStyle(
                      color: Colors.white,
                      fontWeight: FontWeight.w700,
                      fontSize: 17,
                    ),
                  ),
                  if (subtitle != null) ...[
                    4.vgap,
                    Text(
                      subtitle!,
                      style: TextStyle(
                        color: AppColors.neutral300,
                        fontSize: 13,
                        fontWeight: FontWeight.w600,
                      ),
                    ),
                  ],
                ],
              ),
            ),
            const Icon(
              Icons.chevron_right,
              color: AppColors.neutral500,
              size: 30,
            ),
          ],
        ),
      ),
    );
  }
}
