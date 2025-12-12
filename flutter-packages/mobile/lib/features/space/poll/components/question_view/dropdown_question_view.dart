import 'package:ratel/exports.dart';

class DropdownQuestionView extends StatelessWidget {
  const DropdownQuestionView({
    super.key,
    required this.question,
    required this.answer,
    required this.onChanged,
    required this.readOnly,
  });

  final DropdownQuestionModel question;
  final DropdownAnswer? answer;
  final ValueChanged<Answer> onChanged;
  final bool readOnly;

  @override
  Widget build(BuildContext context) {
    final current = answer?.answer;

    return SizedBox(
      width: 260,
      child: DropdownButtonFormField<int>(
        value: current,
        icon: const Icon(Icons.arrow_drop_down, color: Colors.white),
        dropdownColor: const Color(0xFF111111),
        style: const TextStyle(
          fontFamily: 'Inter',
          fontSize: 14,
          color: Colors.white,
        ),
        decoration: InputDecoration(
          hintText: 'Select an option',
          hintStyle: const TextStyle(
            fontFamily: 'Inter',
            fontSize: 14,
            color: Color(0xFF6B6B6B),
          ),
          enabledBorder: OutlineInputBorder(
            borderRadius: BorderRadius.circular(8),
            borderSide: const BorderSide(color: Color(0xFF404040)),
          ),
          focusedBorder: OutlineInputBorder(
            borderRadius: BorderRadius.circular(8),
            borderSide: const BorderSide(color: Color(0xFFFCB300)),
          ),
          contentPadding: const EdgeInsets.symmetric(
            horizontal: 12,
            vertical: 10,
          ),
        ),
        items: [
          for (int i = 0; i < question.options.length; i++)
            DropdownMenuItem<int>(value: i, child: Text(question.options[i])),
        ],
        onChanged: readOnly
            ? null
            : (v) {
                onChanged(DropdownAnswer(v));
              },
      ),
    );
  }
}
