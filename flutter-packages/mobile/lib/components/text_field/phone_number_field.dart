import 'package:ratel/exports.dart';

class PhoneNumberField extends StatefulWidget {
  const PhoneNumberField({
    super.key,
    required this.countryCode,
    required this.dialCode,
    required this.controller,
    required this.onTapCountry,
    required this.onChanged,
    required this.onSubmit,
    this.hintText = 'Phone Number',
    this.height = 52,
    this.padding = const EdgeInsets.symmetric(horizontal: 16),
    this.backgroundColor = const Color(0xFF101010),
    this.borderColor = const Color(0xFF2A2A2A),
    this.focusedBorderColor,
    this.borderRadius = 10,
    this.dividerColor = const Color(0xFF262626),
  });

  final String countryCode;
  final String dialCode;

  final TextEditingController controller;
  final VoidCallback onTapCountry;
  final ValueChanged<String> onChanged;
  final VoidCallback onSubmit;

  final String hintText;

  final double height;
  final EdgeInsets padding;

  final Color backgroundColor;
  final Color borderColor;
  final Color? focusedBorderColor;
  final double borderRadius;

  final Color dividerColor;

  @override
  State<PhoneNumberField> createState() => _PhoneNumberFieldState();
}

class _PhoneNumberFieldState extends State<PhoneNumberField> {
  late final FocusNode _focusNode;
  bool _focused = false;

  @override
  void initState() {
    super.initState();
    _focusNode = FocusNode();
    _focusNode.addListener(_onFocusChanged);
  }

  void _onFocusChanged() {
    if (!mounted) return;
    setState(() => _focused = _focusNode.hasFocus);
  }

  @override
  void dispose() {
    _focusNode.removeListener(_onFocusChanged);
    _focusNode.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final effectiveBorder = _focused
        ? (widget.focusedBorderColor ?? widget.borderColor)
        : widget.borderColor;

    return Container(
      height: widget.height,
      padding: widget.padding,
      decoration: BoxDecoration(
        color: widget.backgroundColor,
        borderRadius: BorderRadius.circular(widget.borderRadius),
        border: Border.all(color: effectiveBorder, width: 1),
      ),
      child: Row(
        children: [
          GestureDetector(
            behavior: HitTestBehavior.opaque,
            onTap: widget.onTapCountry,
            child: Row(
              children: [
                Text(
                  '${widget.countryCode.toUpperCase()} +${widget.dialCode}',
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
          Container(width: 1, height: 18, color: widget.dividerColor),
          12.gap,
          Expanded(
            child: TextField(
              focusNode: _focusNode,
              controller: widget.controller,
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
                hintText: widget.hintText,
                hintStyle: AppFonts.textTheme.bodyMedium?.copyWith(
                  color: const Color(0xFF404040),
                  fontSize: 16,
                  fontWeight: FontWeight.w500,
                  height: 22 / 16,
                ),
              ),
              onChanged: widget.onChanged,
              onSubmitted: (_) => widget.onSubmit(),
            ),
          ),
        ],
      ),
    );
  }
}
