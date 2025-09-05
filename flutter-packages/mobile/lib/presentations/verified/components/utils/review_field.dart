import 'package:ratel/exports.dart';

AppTextField field(String v) {
  return AppTextField(
    hint: v,
    controller: TextEditingController(text: v),
    onChanged: (_) {},
    keyboardType: TextInputType.text,
    obscureText: false,
    readOnly: true,
  );
}
