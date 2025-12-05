import 'package:ratel/exports.dart';

class NotificationHeader extends StatelessWidget {
  const NotificationHeader({super.key, required this.onMarkAllRead});

  final VoidCallback onMarkAllRead;

  @override
  Widget build(BuildContext context) {
    return Container(
      padding: const EdgeInsets.fromLTRB(16, 8, 16, 8),
      child: Row(
        children: [
          const Spacer(),
          InkWell(
            onTap: onMarkAllRead,
            borderRadius: BorderRadius.circular(24),
            child: Container(
              padding: const EdgeInsets.symmetric(horizontal: 14, vertical: 8),
              decoration: BoxDecoration(
                borderRadius: BorderRadius.circular(24),
                color: AppColors.primary,
              ),
              child: Row(
                children: [
                  const Icon(Icons.check_circle, size: 12, color: Colors.black),
                  6.gap,
                  Text(
                    'Mark all as read',
                    style: const TextStyle(
                      color: Colors.black,
                      fontSize: 11,
                      fontWeight: FontWeight.w500,
                      height: 1.1,
                    ),
                  ),
                ],
              ),
            ),
          ),
        ],
      ),
    );
  }
}
