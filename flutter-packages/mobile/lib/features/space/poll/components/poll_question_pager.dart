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
  bool _triedNextOnInvalid = false;

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
    if (!_triedNextOnInvalid) return false;
    if (!_isQuestionRequired(_currentQuestion)) return false;
    return !_currentValid;
  }

  void _updateAnswer(Answer answer) {
    if (_readOnly) return;
    setState(() {
      _answers[_index] = answer;
      if (_triedNextOnInvalid && _validateAnswer(_currentQuestion, answer)) {
        _triedNextOnInvalid = false;
      }
    });
  }

  bool get _isLast => _index == widget.poll.questions.length - 1;
  bool get _isFirst => _index == 0;

  bool get _currentValid =>
      _readOnly ? true : _validateAnswer(_currentQuestion, _currentAnswer);

  bool get _canSubmitOrUpdate {
    if (_readOnly) return false;
    if (widget.isFinished) return false;

    final now = DateTime.now().toUtc().millisecondsSinceEpoch;
    if (now < widget.poll.startedAt) return false;
    if (now > widget.poll.endedAt) return false;

    if (widget.poll.myResponse != null && !widget.poll.responseEditable) {
      return false;
    }

    return true;
  }

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

  void _goNext() {
    if (_readOnly) {
      if (!_isLast) {
        setState(() {
          _index++;
          _maxReached = math.max(_maxReached, _index);
          _triedNextOnInvalid = false;
        });
      }
      return;
    }

    if (!_currentValid) {
      setState(() {
        _triedNextOnInvalid = true;
      });
      return;
    }

    setState(() {
      _triedNextOnInvalid = false;
    });

    if (_isLast) {
      if (!_canSubmitOrUpdate) return;

      if (!widget.poll.responseEditable) {
        _showFinalSubmitDialog();
      } else {
        _submit();
      }
    } else {
      setState(() {
        _index++;
        _maxReached = math.max(_maxReached, _index);
        _triedNextOnInvalid = false;
      });
    }
  }

  void _goPrev() {
    if (_isFirst) return;
    setState(() {
      _index--;
      _triedNextOnInvalid = false;
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

    final showPrev = !_isFirst;

    final showRightButton = _readOnly
        ? !_isLast
        : (!_isLast || (_isLast && _canSubmitOrUpdate));

    final isFinalAction = !_readOnly && _isLast && _canSubmitOrUpdate;

    final rightLabel = isFinalAction
        ? (_hasInitialResponse ? 'Update' : 'Submit')
        : 'Next';

    final rightVariant = (isFinalAction && rightLabel == 'Submit')
        ? NavButtonVariant.submit
        : NavButtonVariant.primary;

    final rightStyleEnabled = _readOnly ? true : _currentValid;

    final rightTapEnabled = _readOnly ? !_isLast : true;

    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        if (_isSubmitted) ...[
          SubmittedLabel(),
          20.vgap,
        ] else ...[
          PollProgressBar(
            total: total,
            currentIndex: _index,
            maxReached: _maxReached,
          ),
          40.vgap,
        ],
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
        Row(
          children: [
            if (showPrev)
              NavButton(
                label: 'Prev',
                enabled: true,
                onTap: _goPrev,
                isPrimary: false,
                variant: NavButtonVariant.outline,
              ),
            const Spacer(),
            if (showRightButton)
              NavButton(
                label: rightLabel,
                enabled: rightStyleEnabled,
                tapEnabled: rightTapEnabled,
                onTap: _goNext,
                isPrimary: true,
                variant: rightVariant,
              ),
          ],
        ),
        20.vgap,
      ],
    );
  }
}

class SubmittedLabel extends StatelessWidget {
  const SubmittedLabel({super.key});

  @override
  Widget build(BuildContext context) {
    return RoundContainer(
      radius: 8,
      color: Color(0xFF22C55E).withAlpha(25),
      padding: const EdgeInsets.fromLTRB(12, 10, 12, 10),
      child: Row(
        mainAxisSize: MainAxisSize.min,
        mainAxisAlignment: MainAxisAlignment.center,
        children: [
          Icon(Icons.check, size: 16, color: Color(0xFF22C55E)),
          4.gap,
          Text(
            'Submitted',
            style: TextStyle(
              fontWeight: FontWeight.w700,
              fontSize: 13,
              height: 16 / 13,
              color: Color(0xFF22C55E),
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
