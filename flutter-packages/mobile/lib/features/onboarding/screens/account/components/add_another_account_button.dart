import 'package:ratel/exports.dart';

class AddAnotherAccountButton extends StatelessWidget {
  final VoidCallback onTap;
  const AddAnotherAccountButton({super.key, required this.onTap});

  @override
  Widget build(BuildContext context) {
    return Material(
      color: Colors.transparent,
      child: InkWell(
        onTap: onTap,
        borderRadius: BorderRadius.circular(100),
        child: Container(
          height: 58,
          width: double.infinity,
          padding: const EdgeInsets.symmetric(horizontal: 30, vertical: 10),
          decoration: BoxDecoration(
            color: const Color(0xFFFFFFFF),
            borderRadius: BorderRadius.circular(100),
            border: Border.all(color: const Color(0xFF464646), width: 0.5),
          ),
          child: Row(
            children: [
              SvgPicture.asset(
                'assets/icons/add.svg',
                width: 24,
                height: 24,
                fit: BoxFit.contain,
                colorFilter: const ColorFilter.mode(
                  Color(0xFF737373),
                  BlendMode.srcIn,
                ),
              ),
              10.gap,
              Expanded(
                child: Text(
                  'Add another account',
                  maxLines: 1,
                  overflow: TextOverflow.ellipsis,
                  style: TextStyle(
                    fontSize: 15,
                    height: 23 / 15,
                    fontWeight: FontWeight.w600,
                    color: Color(0xFF171717),
                  ),
                ),
              ),
              20.gap,
              Icon(Icons.chevron_right, size: 20, color: Color(0xFF737373)),
            ],
          ),
        ),
      ),
    );
  }
}
