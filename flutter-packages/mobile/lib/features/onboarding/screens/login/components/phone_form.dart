import 'package:ratel/exports.dart';

class PhoneForm extends StatelessWidget {
  const PhoneForm({
    super.key,
    required this.countryName,
    required this.dialCode,
    required this.phoneController,
    required this.onPhoneChanged,
    required this.showWarning,
    required this.onTapCountry,
  });

  final String countryName;
  final String dialCode;

  final TextEditingController phoneController;
  final ValueChanged<String> onPhoneChanged;

  final bool showWarning;
  final VoidCallback onTapCountry;

  @override
  Widget build(BuildContext context) {
    return Column(
      mainAxisAlignment: MainAxisAlignment.start,
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        _PhoneCountrySelectBox(
          countryName: countryName,
          dialCode: dialCode,
          onTap: onTapCountry,
        ),
        10.vgap,
        _PhoneNumberField(
          controller: phoneController,
          onChanged: onPhoneChanged,
        ),
        if (showWarning) ...[
          const Padding(
            padding: EdgeInsets.only(top: 10),
            child: WarningMessage(message: 'Missing required fields'),
          ),
        ],
      ],
    );
  }
}

class _PhoneCountrySelectBox extends StatelessWidget {
  const _PhoneCountrySelectBox({
    required this.countryName,
    required this.dialCode,
    required this.onTap,
  });

  final String countryName;
  final String dialCode;
  final VoidCallback onTap;

  @override
  Widget build(BuildContext context) {
    return GestureDetector(
      behavior: HitTestBehavior.opaque,
      onTap: onTap,
      child: Container(
        height: 52,
        padding: const EdgeInsets.symmetric(horizontal: 20),
        decoration: BoxDecoration(
          color: const Color(0xFF101010),
          borderRadius: BorderRadius.circular(10),
          border: Border.all(color: const Color(0xFF2A2A2A), width: 1),
        ),
        child: Row(
          children: [
            Expanded(
              child: Text(
                countryName,
                maxLines: 1,
                overflow: TextOverflow.ellipsis,
                softWrap: false,
                style: AppFonts.textTheme.bodyMedium?.copyWith(
                  color: Colors.white,
                  fontSize: 16,
                  fontWeight: FontWeight.w500,
                  height: 22 / 16,
                ),
              ),
            ),
            8.gap,
            Text(
              '+$dialCode',
              maxLines: 1,
              overflow: TextOverflow.clip,
              style: AppFonts.textTheme.bodyMedium?.copyWith(
                color: Colors.white,
                fontSize: 16,
                fontWeight: FontWeight.w500,
                height: 22 / 16,
              ),
            ),
            8.gap,
            const Icon(Icons.keyboard_arrow_down, color: Color(0xFF737373)),
          ],
        ),
      ),
    );
  }
}

class _PhoneNumberField extends StatelessWidget {
  const _PhoneNumberField({required this.controller, required this.onChanged});

  final TextEditingController controller;
  final ValueChanged<String> onChanged;

  @override
  Widget build(BuildContext context) {
    return TextFormField(
      controller: controller,
      keyboardType: TextInputType.number,
      inputFormatters: [
        FilteringTextInputFormatter.digitsOnly,
        LengthLimitingTextInputFormatter(20),
      ],
      style: AppFonts.textTheme.bodyMedium?.copyWith(
        color: Colors.white,
        fontSize: 16,
        fontWeight: FontWeight.w500,
        height: 22 / 16,
      ),
      onChanged: onChanged,
      decoration: InputDecoration(
        hintText: 'Phone Number',
        hintStyle: AppFonts.textTheme.bodyMedium?.copyWith(
          color: const Color(0xFF404040),
          fontSize: 16,
          fontWeight: FontWeight.w500,
          height: 22 / 16,
        ),
        filled: true,
        fillColor: const Color(0xFF101010),
        contentPadding: const EdgeInsets.symmetric(
          horizontal: 20,
          vertical: 14,
        ),
        enabledBorder: OutlineInputBorder(
          borderRadius: BorderRadius.circular(10),
          borderSide: const BorderSide(color: Color(0xFF2A2A2A), width: 1),
        ),
        focusedBorder: OutlineInputBorder(
          borderRadius: BorderRadius.circular(10),
          borderSide: const BorderSide(color: AppColors.primary, width: 1),
        ),
      ),
    );
  }
}
