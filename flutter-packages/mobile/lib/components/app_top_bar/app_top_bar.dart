import 'package:ratel/exports.dart';

class AppTopBar extends StatelessWidget {
  final VoidCallback? onBack;
  final Widget? backIcon;
  final String title;
  final TextStyle? titleStyle;
  final String? rightLabel;
  final TextStyle? rightStyle;
  final VoidCallback? onRight;
  final bool? enableBack;
  final EdgeInsetsGeometry padding;

  const AppTopBar({
    super.key,
    this.onBack,
    this.backIcon,
    required this.title,
    this.titleStyle,
    this.rightLabel,
    this.rightStyle,
    this.enableBack = true,
    this.onRight,
    this.padding = const EdgeInsets.fromLTRB(0, 5, 0, 2),
  });

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: padding,
      child: Row(
        children: [
          InkWell(
            onTap: () {
              if (enableBack ?? false) {
                onBack!();
              }
            },
            child: Row(
              mainAxisSize: MainAxisSize.min,
              children: [
                if (enableBack == true) backIcon ?? Assets.backIcon,
                10.gap,
                Text(
                  title,
                  style:
                      titleStyle ??
                      const TextStyle(
                        color: Colors.white,
                        fontWeight: FontWeight.w600,
                        fontSize: 18,
                        height: 1.2,
                      ),
                ),
              ],
            ),
          ),

          const Spacer(),
          if (rightLabel != null)
            GestureDetector(
              onTap: onRight,
              child: Text(
                rightLabel!,
                style:
                    rightStyle ??
                    const TextStyle(
                      color: AppColors.primary,
                      fontWeight: FontWeight.w600,
                      fontSize: 18,
                      height: 1.2,
                    ),
              ),
            ),
        ],
      ),
    );
  }
}
