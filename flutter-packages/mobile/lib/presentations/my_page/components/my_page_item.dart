import 'package:ratel/exports.dart';

class MyPageItem extends StatelessWidget {
  const MyPageItem({
    super.key,
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
