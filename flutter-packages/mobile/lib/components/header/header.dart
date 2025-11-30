import 'package:ratel/exports.dart';
import 'package:flutter_svg/flutter_svg.dart';

class Header extends StatelessWidget {
  const Header({
    super.key,
    this.profileImage,
    this.onTapPlus,
    this.onTapAvatar,
  });

  final String? profileImage;
  final VoidCallback? onTapPlus;
  final VoidCallback? onTapAvatar;

  @override
  Widget build(BuildContext context) {
    const double h = 80;
    const double avatar = 36;
    const double sidePad = 16;

    return Container(
      color: const Color(0xff1d1d1d),
      height: h,
      width: double.infinity,
      padding: const EdgeInsets.symmetric(horizontal: sidePad),
      alignment: Alignment.center,
      child: Row(
        mainAxisAlignment: MainAxisAlignment.spaceBetween,
        children: [
          InkWell(
            onTap: onTapAvatar,
            child: _Avatar(imageUrl: profileImage, size: avatar),
          ),
          _IconButtonBox(
            onTap: onTapPlus,
            child: SvgPicture.asset(
              Assets.plus,
              width: 28,
              height: 28,
              colorFilter: const ColorFilter.mode(
                AppColors.neutral300,
                BlendMode.srcIn,
              ),
            ),
          ),
        ],
      ),
    );
  }
}

class _Avatar extends StatelessWidget {
  const _Avatar({required this.imageUrl, required this.size});
  final String? imageUrl;
  final double size;

  @override
  Widget build(BuildContext context) {
    return ClipRRect(
      borderRadius: BorderRadius.circular(size / 2),
      child: imageUrl == null || imageUrl!.isEmpty
          ? Container(width: size, height: size, color: AppColors.neutral700)
          : Image.network(
              imageUrl!,
              width: size,
              height: size,
              fit: BoxFit.cover,
            ),
    );
  }
}

class _IconButtonBox extends StatelessWidget {
  const _IconButtonBox({required this.child, this.onTap});
  final Widget child;
  final VoidCallback? onTap;

  @override
  Widget build(BuildContext context) {
    return InkWell(
      onTap: onTap,
      borderRadius: BorderRadius.circular(8),
      child: Container(
        width: 32,
        height: 32,
        decoration: BoxDecoration(
          color: const Color(0xFF262626),
          borderRadius: BorderRadius.circular(8),
        ),
        alignment: Alignment.center,
        child: child,
      ),
    );
  }
}
