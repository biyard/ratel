import 'package:ratel/exports.dart';
import 'package:ratel/features/space/poll/components/poll_question_body.dart';
import 'package:ratel/features/space/poll/components/poll_question_header.dart';

class PollQuestionPager extends StatefulWidget {
  const PollQuestionPager({
    super.key,
    required this.poll,
    required this.onSubmit,
  });

  final PollModel poll;
  final ValueChanged<List<Answer>> onSubmit;

  @override
  State<PollQuestionPager> createState() => _PollQuestionPagerState();
}

class _PollQuestionPagerState extends State<PollQuestionPager> {
  late int _index;
  late List<Answer?> _answers;
  bool _hasInitialResponse = false;

  @override
  void initState() {
    super.initState();

    _answers = List<Answer?>.filled(widget.poll.questions.length, null);

    final prev = widget.poll.myResponse;
    if (prev != null && prev.isNotEmpty) {
      _hasInitialResponse = true;
      final len = prev.length < _answers.length ? prev.length : _answers.length;
      for (var i = 0; i < len; i++) {
        _answers[i] = prev[i];
      }
    }

    final firstUnanswered = _answers.indexWhere((a) => a == null);
    _index = firstUnanswered == -1 ? 0 : firstUnanswered;
  }

  QuestionModel get _currentQuestion => widget.poll.questions[_index];
  Answer? get _currentAnswer => _answers[_index];

  bool get _shouldExpandBody {
    switch (_currentQuestion.type) {
      case AnswerType.shortAnswer:
        return false;
      case AnswerType.dropdown:
        return false;
      default:
        return true;
    }
  }

  void _updateAnswer(Answer answer) {
    setState(() {
      _answers[_index] = answer;
    });
  }

  bool get _isLast => _index == widget.poll.questions.length - 1;
  bool get _isFirst => _index == 0;

  bool get _currentValid => _validateAnswer(_currentQuestion, _currentAnswer);

  void _goNext() {
    if (!_currentValid) return;
    if (_isLast) {
      final all = _answers.whereType<Answer>().toList();
      if (all.length != widget.poll.questions.length) return;
      widget.onSubmit(all);
    } else {
      setState(() {
        _index++;
      });
    }
  }

  void _goPrev() {
    if (_isFirst) return;
    setState(() {
      _index--;
    });
  }

  @override
  Widget build(BuildContext context) {
    final total = widget.poll.questions.length;
    final currentNo = _index + 1;

    final body = PollQuestionBody(
      question: _currentQuestion,
      answer: _currentAnswer,
      onChanged: _updateAnswer,
    );

    final lastLabel = _isLast
        ? (_hasInitialResponse ? 'Update' : 'Submit')
        : 'Next';

    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        16.vgap,
        Text(
          '$currentNo / $total',
          style: const TextStyle(
            fontFamily: 'Inter',
            fontSize: 14,
            height: 22.5 / 14,
            color: Colors.white,
          ),
        ),
        5.vgap,
        PollQuestionHeader(question: _currentQuestion),
        15.vgap,
        if (_shouldExpandBody) ...[
          Expanded(child: body),
        ] else ...[
          body,
          const Spacer(),
        ],
        24.vgap,
        Row(
          mainAxisAlignment: MainAxisAlignment.spaceBetween,
          children: [
            _NavButton(label: 'Prev', enabled: !_isFirst, onTap: _goPrev),
            _NavButton(
              label: lastLabel,
              enabled: _currentValid,
              onTap: _goNext,
            ),
          ],
        ),
        20.vgap,
      ],
    );
  }
}

bool _validateAnswer(QuestionModel q, Answer? a) {
  if (a == null) return false;

  switch (q.type) {
    case AnswerType.singleChoice:
      final qq = q as ChoiceQuestionModel;
      final aa = a as SingleChoiceAnswer;
      if (qq.isRequired && aa.answer == null) return false;
      if (aa.answer == null) return true;
      return aa.answer! >= 0 && aa.answer! < qq.options.length;

    case AnswerType.multipleChoice:
      final qq = q as ChoiceQuestionModel;
      final aa = a as MultipleChoiceAnswer;
      final list = aa.answer ?? const [];
      if (qq.isRequired && list.isEmpty) return false;
      if (list.any((v) => v < 0 || v >= qq.options.length)) return false;
      return true;

    case AnswerType.shortAnswer:
    case AnswerType.subjective:
      final qq = q as SubjectiveQuestionModel;
      final aa = a is ShortAnswer
          ? (a as ShortAnswer).answer
          : (a as SubjectiveAnswer).answer;
      if (!qq.isRequired) return true;
      return (aa != null && aa.trim().isNotEmpty);

    case AnswerType.checkbox:
      final qq = q as CheckboxQuestionModel;
      final aa = a as CheckboxAnswer;
      final list = aa.answer ?? const [];
      if (qq.isRequired && list.isEmpty) return false;
      if (!qq.isMulti && list.length > 1) return false;
      if (list.any((v) => v < 0 || v >= qq.options.length)) return false;
      return true;

    case AnswerType.dropdown:
      final qq = q as DropdownQuestionModel;
      final aa = a as DropdownAnswer;
      if (qq.isRequired && aa.answer == null) return false;
      if (aa.answer == null) return true;
      return aa.answer! >= 0 && aa.answer! < qq.options.length;

    case AnswerType.linearScale:
      final qq = q as LinearScaleQuestionModel;
      final aa = a as LinearScaleAnswer;
      if (qq.isRequired && aa.answer == null) return false;
      if (aa.answer == null) return true;
      final v = aa.answer!;
      return v >= qq.minValue && v <= qq.maxValue;
  }
}

class _NavButton extends StatelessWidget {
  const _NavButton({
    required this.label,
    required this.enabled,
    required this.onTap,
  });

  final String label;
  final bool enabled;
  final VoidCallback onTap;

  @override
  Widget build(BuildContext context) {
    return GestureDetector(
      onTap: enabled ? onTap : null,
      child: Container(
        alignment: Alignment.center,
        decoration: BoxDecoration(
          color: enabled ? Colors.white : const Color(0xFF737373),
          borderRadius: BorderRadius.circular(12),
        ),
        child: Padding(
          padding: const EdgeInsets.fromLTRB(20, 10, 20, 10),
          child: Text(
            label,
            style: TextStyle(
              fontFamily: 'Raleway',
              fontWeight: FontWeight.w600,
              fontSize: 14,
              color: enabled
                  ? const Color(0xFF1D1D1D)
                  : const Color(0xFF262626),
            ),
          ),
        ),
      ),
    );
  }
}
