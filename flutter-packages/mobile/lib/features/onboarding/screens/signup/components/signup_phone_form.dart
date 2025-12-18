import 'package:ratel/exports.dart';

class SignupPhoneForm extends StatelessWidget {
  const SignupPhoneForm({
    super.key,
    required this.countryCode,
    required this.dialCode,
    required this.phoneController,
    required this.onTapCountry,
    required this.onPhoneChanged,
    required this.onSubmit,
  });

  final String countryCode;
  final String dialCode;

  final TextEditingController phoneController;
  final VoidCallback onTapCountry;
  final ValueChanged<String> onPhoneChanged;
  final VoidCallback onSubmit;

  @override
  Widget build(BuildContext context) {
    return Container(
      height: 52,
      padding: const EdgeInsets.symmetric(horizontal: 16),
      decoration: BoxDecoration(
        color: const Color(0xFF101010),
        borderRadius: BorderRadius.circular(10),
        border: Border.all(color: const Color(0xFF2A2A2A), width: 1),
      ),
      child: Row(
        children: [
          GestureDetector(
            behavior: HitTestBehavior.opaque,
            onTap: onTapCountry,
            child: Row(
              children: [
                Text(
                  '${countryCode.toUpperCase()} +$dialCode',
                  style: AppFonts.textTheme.bodyMedium?.copyWith(
                    color: Colors.white,
                    fontSize: 16,
                    fontWeight: FontWeight.w500,
                    height: 22 / 16,
                  ),
                ),
                6.gap,
                const Icon(
                  Icons.keyboard_arrow_down,
                  size: 18,
                  color: Color(0xFF737373),
                ),
              ],
            ),
          ),
          12.gap,
          Container(width: 1, height: 18, color: const Color(0xFF262626)),
          12.gap,
          Expanded(
            child: TextField(
              controller: phoneController,
              keyboardType: TextInputType.number,
              textInputAction: TextInputAction.done,
              inputFormatters: [FilteringTextInputFormatter.digitsOnly],
              style: AppFonts.textTheme.bodyMedium?.copyWith(
                color: Colors.white,
                fontSize: 16,
                fontWeight: FontWeight.w500,
                height: 22 / 16,
              ),
              decoration: InputDecoration(
                isCollapsed: true,
                border: InputBorder.none,
                hintText: 'Phone Number',
                hintStyle: AppFonts.textTheme.bodyMedium?.copyWith(
                  color: const Color(0xFFBABABA),
                  fontSize: 16,
                  fontWeight: FontWeight.w500,
                  height: 22 / 16,
                ),
              ),
              onChanged: onPhoneChanged,
              onSubmitted: (_) => onSubmit(),
            ),
          ),
        ],
      ),
    );
  }
}
