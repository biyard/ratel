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
    return PhoneNumberField(
      countryCode: countryCode,
      dialCode: dialCode,
      controller: phoneController,
      onTapCountry: onTapCountry,
      onChanged: onPhoneChanged,
      onSubmit: onSubmit,
      hintText: 'Phone Number',
      height: 52,
      padding: const EdgeInsets.symmetric(horizontal: 16),
      backgroundColor: const Color(0xFF101010),
      borderColor: const Color(0xFF2A2A2A),
      focusedBorderColor: const Color(0xFF2A2A2A),
      borderRadius: 10,
      dividerColor: const Color(0xFF262626),
    );
  }
}
