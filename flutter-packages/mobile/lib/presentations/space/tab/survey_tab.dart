import 'package:ratel/exports.dart';

class Survey extends StatefulWidget {
  const Survey({super.key, this.onSubmit});
  final void Function(List<Answer>)? onSubmit;

  @override
  State<Survey> createState() => _SurveyState();
}

class _SurveyState extends State<Survey> {
  late final SpaceController controller;
  late final List<ChoiceQuestionModel> singleChoiceQs;
  late final List<SingleChoiceAnswer> answers;

  @override
  void initState() {
    super.initState();
    controller = Get.find<SpaceController>();
    final all = controller.questions.value;
    singleChoiceQs = all
        .where((q) => q.type == AnswerType.singleChoice)
        .cast<ChoiceQuestionModel>()
        .toList();
    answers = List.generate(
      singleChoiceQs.length,
      (_) => const SingleChoiceAnswer(null),
    );
  }

  void _setAnswer(int qIndex, int optionIndex) {
    setState(() => answers[qIndex] = SingleChoiceAnswer(optionIndex));
  }

  bool get _isAllAnswered =>
      answers.isNotEmpty && answers.every((a) => a.answer != null);

  void _submit() {
    final payload = List<Answer>.from(answers);
    widget.onSubmit?.call(payload);
    controller.sendAnswer(payload);
    logger.d(payload.map((e) => e.toJson()).toList());
  }

  @override
  Widget build(BuildContext context) {
    final title = controller.space.value.title;

    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Padding(
          padding: const EdgeInsets.all(20),
          child: Row(
            children: [
              InkWell(
                onTap: () => controller.goBack(),
                child: RoundContainer(
                  color: Colors.white.withAlpha(50),
                  radius: 100,
                  child: Padding(
                    padding: const EdgeInsets.all(5.0),
                    child: SvgPicture.asset(Assets.back, width: 20, height: 20),
                  ),
                ),
              ),
              20.gap,
              Expanded(
                child: Text(
                  title.isEmpty ? '' : title,
                  maxLines: 1,
                  overflow: TextOverflow.ellipsis,
                  style: const TextStyle(
                    color: Colors.white,
                    fontSize: 14,
                    fontWeight: FontWeight.w600,
                    height: 1.1,
                  ),
                ),
              ),
            ],
          ),
        ),

        Expanded(
          child: ListView.separated(
            padding: const EdgeInsets.fromLTRB(20, 0, 20, 24),
            itemCount: singleChoiceQs.length,
            separatorBuilder: (_, __) => const SizedBox(height: 14),
            itemBuilder: (context, i) {
              final q = singleChoiceQs[i];
              final selected = answers[i].answer;
              return _SingleChoiceQuestionCard(
                question: q,
                selectedIndex: selected,
                onChanged: (opt) => _setAnswer(i, opt),
              );
            },
          ),
        ),

        Padding(
          padding: const EdgeInsets.fromLTRB(20, 0, 20, 150),
          child: SizedBox(
            width: double.infinity,
            child: ElevatedButton(
              onPressed: _isAllAnswered ? _submit : null,
              style: ElevatedButton.styleFrom(
                backgroundColor: _isAllAnswered
                    ? AppColors.primary
                    : AppColors.neutral700,
                foregroundColor: Colors.black,
                padding: const EdgeInsets.symmetric(vertical: 14.5),
                shape: RoundedRectangleBorder(
                  borderRadius: BorderRadius.circular(10),
                ),
              ),
              child: const Text(
                'Submit',
                style: TextStyle(
                  color: AppColors.bg,
                  fontSize: 16,
                  fontWeight: FontWeight.w700,
                ),
              ),
            ),
          ),
        ),
      ],
    );
  }
}

class _SingleChoiceQuestionCard extends StatelessWidget {
  const _SingleChoiceQuestionCard({
    required this.question,
    required this.selectedIndex,
    required this.onChanged,
  });

  final ChoiceQuestionModel question;
  final int? selectedIndex;
  final ValueChanged<int> onChanged;

  @override
  Widget build(BuildContext context) {
    final q = question;

    return Container(
      decoration: BoxDecoration(
        color: const Color(0xFF151515),
        borderRadius: BorderRadius.circular(8),
        border: Border.all(color: AppColors.neutral700, width: 1),
      ),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Padding(
            padding: const EdgeInsets.fromLTRB(12, 10, 12, 8),
            child: Text(
              q.title,
              style: const TextStyle(
                color: AppColors.neutral300,
                fontSize: 12,
                fontWeight: FontWeight.w500,
              ),
            ),
          ),
          const Divider(height: 1, color: AppColors.neutral700),
          ...List.generate(q.options.length, (i) {
            final text = q.options[i];
            final selected = selectedIndex == i;
            return InkWell(
              onTap: () => onChanged(i),
              child: Padding(
                padding: const EdgeInsets.symmetric(
                  horizontal: 12,
                  vertical: 10,
                ),
                child: Row(
                  children: [
                    _CheckBox(selected: selected),
                    const SizedBox(width: 10),
                    Expanded(
                      child: Text(
                        text,
                        style: TextStyle(
                          color: selected ? Colors.white : AppColors.neutral300,
                          fontSize: 14,
                          fontWeight: selected
                              ? FontWeight.w600
                              : FontWeight.w500,
                        ),
                      ),
                    ),
                  ],
                ),
              ),
            );
          }),
        ],
      ),
    );
  }
}

class _CheckBox extends StatelessWidget {
  const _CheckBox({required this.selected});
  final bool selected;

  @override
  Widget build(BuildContext context) {
    return Container(
      width: 18,
      height: 18,
      decoration: BoxDecoration(
        borderRadius: BorderRadius.circular(4),
        border: Border.all(
          color: selected ? AppColors.primary : AppColors.neutral600,
          width: 1.4,
        ),
        color: selected ? const Color(0xFF2A2A2A) : Colors.transparent,
      ),
      child: selected
          ? const Center(
              child: Icon(
                Icons.check_rounded,
                size: 14,
                color: AppColors.primary,
              ),
            )
          : const SizedBox.shrink(),
    );
  }
}
