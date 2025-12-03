import 'package:ratel/exports.dart';
import 'package:intl/intl.dart';
import 'package:ratel/features/space/poll/components/poll_question_body.dart';
import 'package:ratel/features/space/poll/components/poll_question_header.dart';

class PollQuestionPager extends StatefulWidget {
  const PollQuestionPager({
    super.key,
    required this.spacePk,
    required this.poll,
    required this.onSubmit,
  });

  final String spacePk;
  final PollModel poll;
  final ValueChanged<List<Answer>> onSubmit;

  @override
  State<PollQuestionPager> createState() => _PollQuestionPagerState();
}

class _PollQuestionPagerState extends State<PollQuestionPager> {
  late int _index;
  late List<Answer?> _answers;
  bool _hasInitialResponse = false;
  late final bool _readOnly;

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

    _readOnly = widget.poll.myResponse != null && !widget.poll.responseEditable;
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
    if (_readOnly) return;
    setState(() {
      _answers[_index] = answer;
    });
  }

  bool get _isLast => _index == widget.poll.questions.length - 1;
  bool get _isFirst => _index == 0;

  bool get _currentValid =>
      _readOnly ? true : _validateAnswer(_currentQuestion, _currentAnswer);

  void _submit() {
    final all = <Answer>[];
    for (var i = 0; i < widget.poll.questions.length; i++) {
      final q = widget.poll.questions[i];
      final a = _answers[i];
      all.add(a ?? _buildEmptyAnswer(q));
    }
    widget.onSubmit(all);
  }

  void _showFinalSubmitDialog() {
    Get.dialog(
      Center(
        child: Container(
          margin: const EdgeInsets.symmetric(horizontal: 24),
          padding: const EdgeInsets.fromLTRB(24, 24, 24, 16),
          decoration: BoxDecoration(
            color: const Color(0xFF111111),
            borderRadius: BorderRadius.circular(20),
          ),
          child: Column(
            mainAxisSize: MainAxisSize.min,
            children: [
              Row(
                children: [
                  const Expanded(
                    child: Text(
                      'Submit Survey',
                      textAlign: TextAlign.center,
                      style: TextStyle(
                        fontFamily: 'Raleway',
                        fontSize: 16,
                        fontWeight: FontWeight.w700,
                        decoration: TextDecoration.none,
                        color: Colors.white,
                      ),
                    ),
                  ),
                  GestureDetector(
                    onTap: () => Get.back(),
                    child: const Icon(
                      Icons.close,
                      size: 18,
                      color: Colors.white70,
                    ),
                  ),
                ],
              ),
              16.vgap,
              const Align(
                alignment: Alignment.centerLeft,
                child: Text(
                  'Once you submit your response, it cannot be changed.\nPlease double-check before submitting.',
                  style: TextStyle(
                    fontFamily: 'Inter',
                    fontSize: 16,
                    fontWeight: FontWeight.w500,
                    height: 1.5,
                    decoration: TextDecoration.none,
                    color: Color(0xFFA1A1A1),
                  ),
                ),
              ),
              24.vgap,
              Row(
                mainAxisAlignment: MainAxisAlignment.end,
                children: [
                  GestureDetector(
                    onTap: () => Get.back(),
                    child: Container(
                      padding: const EdgeInsets.symmetric(
                        horizontal: 20,
                        vertical: 10,
                      ),
                      decoration: BoxDecoration(
                        color: Colors.white,
                        borderRadius: BorderRadius.circular(12),
                      ),
                      child: const Text(
                        'Cancel',
                        style: TextStyle(
                          fontFamily: 'Raleway',
                          fontWeight: FontWeight.w600,
                          fontSize: 14,
                          decoration: TextDecoration.none,
                          color: Colors.black,
                        ),
                      ),
                    ),
                  ),
                  12.gap,
                  GestureDetector(
                    onTap: () {
                      Get.back();
                      _submit();
                    },
                    child: Container(
                      padding: const EdgeInsets.symmetric(
                        horizontal: 20,
                        vertical: 10,
                      ),
                      decoration: BoxDecoration(
                        color: AppColors.primary,
                        borderRadius: BorderRadius.circular(12),
                      ),
                      child: const Text(
                        'Submit',
                        style: TextStyle(
                          fontFamily: 'Raleway',
                          fontWeight: FontWeight.w600,
                          fontSize: 14,
                          decoration: TextDecoration.none,
                          color: Colors.black,
                        ),
                      ),
                    ),
                  ),
                ],
              ),
            ],
          ),
        ),
      ),
      barrierDismissible: true,
    );
  }

  void _goNext() {
    if (_readOnly) {
      if (!_isLast) {
        setState(() => _index++);
      }
      return;
    }

    if (!_currentValid) return;

    if (_isLast) {
      if (!widget.poll.responseEditable) {
        _showFinalSubmitDialog();
      } else {
        _submit();
      }
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

    final start = _fromTimestamp(widget.poll.startedAt);
    final end = _fromTimestamp(widget.poll.endedAt);

    final body = PollQuestionBody(
      question: _currentQuestion,
      answer: _currentAnswer,
      onChanged: _updateAnswer,
      readOnly: _readOnly,
    );

    final lastLabel = _isLast
        ? (_hasInitialResponse ? 'Update' : 'Submit')
        : 'Next';

    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        _PollTimeHeader(timeZone: 'Asia/Seoul', start: start, end: end),
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
              enabled: _readOnly ? !_isLast : _currentValid,
              onTap: _goNext,
            ),
          ],
        ),
        20.vgap,
      ],
    );
  }
}

class _PollTimeHeader extends StatelessWidget {
  const _PollTimeHeader({
    required this.timeZone,
    required this.start,
    required this.end,
  });

  final String timeZone;
  final DateTime start;
  final DateTime end;

  @override
  Widget build(BuildContext context) {
    final fmt = DateFormat('MMM d, yyyy, hh:mm a');

    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Text(
          timeZone,
          style: const TextStyle(
            fontFamily: 'Inter',
            fontSize: 12,
            color: Color(0xFF9CA3AF),
          ),
        ),
        4.vgap,
        Row(
          children: [
            Text(
              fmt.format(start),
              style: const TextStyle(
                fontFamily: 'Inter',
                fontSize: 13,
                color: Colors.white,
              ),
            ),
            6.gap,
            const Text(
              '->',
              style: TextStyle(
                fontFamily: 'Inter',
                fontSize: 13,
                color: Color(0xFF9CA3AF),
              ),
            ),
            6.gap,
            Text(
              fmt.format(end),
              style: const TextStyle(
                fontFamily: 'Inter',
                fontSize: 13,
                color: Colors.white,
              ),
            ),
          ],
        ),
      ],
    );
  }
}

DateTime _fromTimestamp(int ts) {
  if (ts == 0) {
    return DateTime.fromMillisecondsSinceEpoch(0);
  }
  if (ts < 1000000000000) {
    return DateTime.fromMillisecondsSinceEpoch(ts * 1000);
  }
  return DateTime.fromMillisecondsSinceEpoch(ts);
}

bool _isQuestionRequired(QuestionModel q) {
  switch (q.type) {
    case AnswerType.singleChoice:
    case AnswerType.multipleChoice:
      return (q as ChoiceQuestionModel).isRequired;
    case AnswerType.shortAnswer:
    case AnswerType.subjective:
      return (q as SubjectiveQuestionModel).isRequired;
    case AnswerType.checkbox:
      return (q as CheckboxQuestionModel).isRequired;
    case AnswerType.dropdown:
      return (q as DropdownQuestionModel).isRequired;
    case AnswerType.linearScale:
      return (q as LinearScaleQuestionModel).isRequired;
  }
}

Answer _buildEmptyAnswer(QuestionModel q) {
  switch (q.type) {
    case AnswerType.singleChoice:
      return SingleChoiceAnswer(null, null);
    case AnswerType.multipleChoice:
      return MultipleChoiceAnswer(const [], null);
    case AnswerType.shortAnswer:
      return ShortAnswer(null);
    case AnswerType.subjective:
      return SubjectiveAnswer(null);
    case AnswerType.checkbox:
      return CheckboxAnswer(const []);
    case AnswerType.dropdown:
      return DropdownAnswer(null);
    case AnswerType.linearScale:
      return LinearScaleAnswer(null);
  }
}

bool _validateAnswer(QuestionModel q, Answer? a) {
  final required = _isQuestionRequired(q);

  if (a == null) {
    return !required;
  }

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
