import 'package:ratel/exports.dart';
import 'package:flutter_svg/flutter_svg.dart';

class Header extends StatelessWidget {
  const Header({
    super.key,
    this.profileImage,
    this.hint = 'Search for a country',
    this.onSearchChanged,
    this.onTapFilter,
    this.onTapPlus,
    this.controller,
    this.onTapAvatar,
  });

  final String? profileImage;
  final String hint;
  final ValueChanged<String>? onSearchChanged;
  final VoidCallback? onTapFilter;
  final VoidCallback? onTapPlus;
  final TextEditingController? controller;
  final VoidCallback? onTapAvatar;

  @override
  Widget build(BuildContext context) {
    const double h = 80;
    const double avatar = 36;
    const double iconPad = 12;
    const double sidePad = 16;
    const double iconBox = 28;

    final double w = MediaQuery.of(context).size.width;
    final double fieldWidth =
        w -
        (sidePad * 2 +
            avatar +
            iconPad +
            iconBox +
            iconPad +
            iconBox +
            iconPad);

    return Container(
      color: AppColors.neutral800,
      height: h,
      width: double.infinity,
      padding: const EdgeInsets.symmetric(horizontal: sidePad),
      alignment: Alignment.center,
      child: Row(
        children: [
          InkWell(
            onTap: onTapAvatar,
            child: _Avatar(imageUrl: profileImage, size: avatar),
          ),
          const SizedBox(width: iconPad),
          SizedBox(
            width: fieldWidth.clamp(160, double.infinity),
            child: AppTextField(
              hint: hint,
              controller: controller,
              rounded: 28,
              suffixIcon: const Padding(
                padding: EdgeInsets.only(right: 8),
                child: Icon(Icons.search, color: AppColors.neutral600),
              ),
              onChanged: onSearchChanged,
              border: OutlineInputBorder(
                borderRadius: BorderRadius.circular(28),
                borderSide: const BorderSide(
                  color: AppColors.borderPrimary,
                  width: 1,
                ),
              ),
              bgColor: Colors.transparent,
            ),
          ),
          const SizedBox(width: iconPad),
          _IconButtonBox(
            onTap: onTapFilter,
            child: SvgPicture.asset(
              width: 24,
              height: 24,
              colorFilter: const ColorFilter.mode(
                AppColors.neutral500,
                BlendMode.srcIn,
              ),
              Assets.filter,
            ),
          ),
          const SizedBox(width: iconPad),
          _IconButtonBox(
            onTap: onTapPlus,
            child: SvgPicture.asset(
              Assets.plus,
              width: 24,
              height: 24,
              colorFilter: const ColorFilter.mode(
                AppColors.neutral500,
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
      child: Container(
        width: 24,
        height: 24,
        alignment: Alignment.center,
        child: child,
      ),
    );
  }
}
