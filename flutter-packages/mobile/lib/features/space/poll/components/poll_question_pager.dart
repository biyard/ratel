import 'dart:math' as math;

import 'package:ratel/exports.dart';
import 'package:ratel/features/space/poll/components/poll_question_body.dart';
import 'package:ratel/features/space/poll/components/poll_question_header.dart';
import 'package:ratel/features/space/poll/components/submit_modal.dart';

import 'poll_progress_header.dart';
import 'poll_time_header.dart';

class PollQuestionPager extends StatefulWidget {
  const PollQuestionPager({
    super.key,
    required this.spacePk,
    required this.isFinished,
    required this.poll,
    required this.onSubmit,
  });

  final String spacePk;
  final bool isFinished;
  final PollModel poll;
  final ValueChanged<List<Answer>> onSubmit;

  @override
  State<PollQuestionPager> createState() => _PollQuestionPagerState();
}

class _PollQuestionPagerState extends State<PollQuestionPager> {
  late int _index;
  late int _maxReached;
  late List<Answer?> _answers;
  bool _hasInitialResponse = false;
  late final bool _readOnly;

  bool get _isSubmitted =>
      (widget.poll.myResponse != null && !widget.poll.responseEditable);

  @override
  void initState() {
    super.initState();

    final now = DateTime.now().toUtc().millisecondsSinceEpoch;

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

    _readOnly =
        (widget.poll.myResponse != null && !widget.poll.responseEditable) ||
        widget.isFinished ||
        (now < widget.poll.startedAt) ||
        (now > widget.poll.endedAt);

    final lastAnswered = _answers.lastIndexWhere((a) => a != null);
    final total = widget.poll.questions.length;
    _maxReached = _readOnly
        ? (total - 1)
        : math.max(_index, lastAnswered < 0 ? 0 : lastAnswered);

    if (firstUnanswered == -1 && total > 0) {
      _maxReached = total - 1;
    }
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

  bool get _showMissingRequired {
    if (_readOnly) return false;
    if (!_isQuestionRequired(_currentQuestion)) return false;
    return !_currentValid;
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
    SubmitModal.show(onConfirm: _submit);
  }

  void _goTo(int i) {
    if (i < 0 || i >= widget.poll.questions.length) return;
    if (i > _maxReached) return;
    setState(() => _index = i);
  }

  void _goNext() {
    if (_readOnly) {
      if (!_isLast) {
        setState(() {
          _index++;
          _maxReached = math.max(_maxReached, _index);
        });
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
        _maxReached = math.max(_maxReached, _index);
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

    final isSubmit = lastLabel == 'Submit';

    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        PollProgressBar(
          total: total,
          currentIndex: _index,
          maxReached: _maxReached,
          onTap: _goTo,
        ),
        40.vgap,
        PollQuestionHeader(question: _currentQuestion),
        20.vgap,
        PollTimeHeader(timeZone: 'Asia/Seoul', start: start, end: end),
        10.vgap,
        if (_shouldExpandBody) ...[
          Expanded(
            child: SingleChildScrollView(
              physics: const BouncingScrollPhysics(),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  body,
                  if (_showMissingRequired) ...[
                    10.vgap,
                    const WarningMessage(message: "Missing required fields"),
                  ],
                ],
              ),
            ),
          ),
        ] else ...[
          Column(
            mainAxisSize: MainAxisSize.min,
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              body,
              if (_showMissingRequired) ...[
                10.vgap,
                const WarningMessage(message: "Missing required fields"),
              ],
            ],
          ),
          const Spacer(),
        ],
        24.vgap,
        if (_isSubmitted) ...[
          _SubmittedBar(),
        ] else ...[
          Row(
            mainAxisAlignment: MainAxisAlignment.spaceBetween,
            children: [
              _NavButton(
                label: 'Prev',
                enabled: !_isFirst,
                onTap: _goPrev,
                isPrimary: false,
                variant: _NavButtonVariant.outline,
              ),
              _NavButton(
                label: lastLabel,
                enabled: _readOnly ? !_isLast : _currentValid,
                onTap: _goNext,
                isPrimary: true,
                variant: isSubmit
                    ? _NavButtonVariant.submit
                    : _NavButtonVariant.primary,
              ),
            ],
          ),
        ],
        20.vgap,
      ],
    );
  }
}

class _SubmittedBar extends StatelessWidget {
  const _SubmittedBar();

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      width: double.infinity,
      child: Row(
        mainAxisAlignment: MainAxisAlignment.center,
        children: [
          Icon(Icons.check, size: 24, color: Color(0xFF737373)),
          SizedBox(width: 8),
          Text(
            'Submitted',
            style: TextStyle(
              fontWeight: FontWeight.w700,
              fontSize: 16,
              height: 18 / 16,
              color: Colors.white.withAlpha(125),
            ),
          ),
        ],
      ),
    );
  }
}

DateTime _fromTimestamp(int ts) {
  if (ts == 0) return DateTime.fromMillisecondsSinceEpoch(0);
  if (ts < 1000000000000) return DateTime.fromMillisecondsSinceEpoch(ts * 1000);
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

  if (a == null) return !required;

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
      final text = a is ShortAnswer ? a.answer : (a as SubjectiveAnswer).answer;
      if (!qq.isRequired) return true;
      return text != null && text.trim().isNotEmpty;

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

enum _NavButtonVariant { outline, primary, submit }

class _NavButton extends StatelessWidget {
  const _NavButton({
    required this.label,
    required this.enabled,
    required this.onTap,
    required this.isPrimary,
    this.variant = _NavButtonVariant.primary,
  });

  final String label;
  final bool enabled;
  final VoidCallback onTap;
  final bool isPrimary;
  final _NavButtonVariant variant;

  @override
  Widget build(BuildContext context) {
    final isOutline = variant == _NavButtonVariant.outline;
    final isSubmit = variant == _NavButtonVariant.submit;

    final bgColor = isSubmit
        ? const Color(0xFFFCB300)
        : (isPrimary ? Colors.white : Colors.transparent);

    final textColor = isSubmit
        ? const Color(0xFF0A0A0A)
        : (isPrimary ? const Color(0xFF0A0A0A) : Colors.white);

    final disabledBg = isSubmit
        ? const Color(0xFFFCB300).withAlpha(140)
        : (isPrimary ? Colors.white.withAlpha(125) : Colors.transparent);

    final effectiveBg = enabled ? bgColor : disabledBg;
    final effectiveText = enabled
        ? textColor
        : (isSubmit ? const Color(0xFF0A0A0A) : const Color(0xFF737373));

    final borderColor = isOutline
        ? (enabled ? Colors.white : const Color(0xFF737373))
        : Colors.transparent;

    Widget chevron() {
      if (isSubmit) return const SizedBox.shrink();
      final iconColor = enabled
          ? const Color(0xFF737373)
          : const Color(0xFF404040);
      return Icon(
        isPrimary ? Icons.chevron_right : Icons.chevron_left,
        size: 24,
        color: iconColor,
      );
    }

    return GestureDetector(
      behavior: HitTestBehavior.opaque,
      onTap: enabled ? onTap : null,
      child: Container(
        padding: const EdgeInsets.symmetric(horizontal: 25, vertical: 12),
        decoration: BoxDecoration(
          color: effectiveBg,
          borderRadius: BorderRadius.circular(12),
          border: Border.all(color: borderColor, width: 1),
        ),
        child: Row(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            if (!isPrimary && !isSubmit) ...[chevron(), 8.gap],
            Text(
              label,
              style: TextStyle(
                fontWeight: FontWeight.w700,
                fontSize: 16,
                height: 18 / 16,
                color: effectiveText,
              ),
            ),
            if (isPrimary && !isSubmit) ...[8.gap, chevron()],
          ],
        ),
      ),
    );
  }
}
