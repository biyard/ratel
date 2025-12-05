import 'package:ratel/exports.dart';

class Header extends StatelessWidget {
  const Header({super.key, required this.title});
  final String title;

  @override
  Widget build(BuildContext context) {
    const double avatar = 36;
    const double sidePad = 16;

    return Container(
      color: const Color(0xff1d1d1d),
      width: double.infinity,
      padding: const EdgeInsets.symmetric(horizontal: sidePad),
      alignment: Alignment.center,
      child: Column(
        children: [
          30.vgap,
          Row(
            mainAxisAlignment: MainAxisAlignment.start,
            children: [
              _Avatar(imageUrl: defaultProfileImage, size: avatar),
              10.gap,
              Text(
                title,
                style: TextStyle(
                  fontSize: 15,
                  fontWeight: FontWeight.w700,
                  color: Colors.white,
                ),
              ),
            ],
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
