import 'package:ratel/exports.dart';

class NetworkLocalization {
  static final _invitations = s('invitations', 'Invitations', 'Invitations');
  static String get invitations => _invitations.tr;

  static final _reject = s('reject', 'Reject', 'Reject');
  static String get reject => _reject.tr;

  static final _accept = s('accept', 'Accept', 'Accept');
  static String get accept => _accept.tr;

  static final _suggestions = s('suggestions', 'Suggestions', 'Suggestions');
  static String get suggestions => _suggestions.tr;

  static final _follow = s('follow', 'Follow', 'Follow');
  static String get follow => _follow.tr;
}
