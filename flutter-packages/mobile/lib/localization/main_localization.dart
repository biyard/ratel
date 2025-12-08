import 'package:ratel/exports.dart';

class MainLocalization {
  // Information
  static final _appName = s('appName', 'Ratel', 'Ratel');

  static String get appName => _appName.tr;

  static final _home = s('home', 'Home', 'Home');
  static String get home => _home.tr;

  static final _network = s('network', 'Network', 'Network');
  static String get network => _network.tr;

  static final _spaces = s('spaces', 'Spaces', 'Spaces');
  static String get spaces => _spaces.tr;

  static final _notification = s(
    'notification',
    'Notification',
    'Notification',
  );
  static String get notification => _notification.tr;

  static final _my = s('my', 'My', 'My');
  static String get my => _my.tr;

  static final _messages = s('messages', 'Messages', 'Messages');
  static String get messages => _messages.tr;

  static final _points = s('points', 'Points', 'Points');
  static String get points => _points.tr;

  static final _following = s('following', 'Following', 'Following');
  static String get following => _following.tr;

  static final _followers = s('followers', 'Followers', 'Followers');
  static String get followers => _followers.tr;

  static final _drafts = s('drafts', 'Drafts', 'Drafts');
  static String get drafts => _drafts.tr;

  static final _posts = s('posts', 'Posts', 'Posts');
  static String get posts => _posts.tr;

  static final _bookmarks = s('bookmarks', 'Bookmarks', 'Bookmarks');
  static String get bookmarks => _bookmarks.tr;

  static final _verifiedCredential = s(
    'verifiedCredential',
    'Verified Credential',
    'Verified Credential',
  );
  static String get verifiedCredential => _verifiedCredential.tr;

  static final _myRewards = s('myRewards', 'My Rewards', 'My Rewards');
  static String get myRewards => _myRewards.tr;

  static final _settings = s('settings', 'Settings', 'Settings');
  static String get settings => _settings.tr;

  static final _theme = s('theme', 'Theme', 'Theme');
  static String get theme => _theme.tr;

  static final _dark = s('dark', 'Dark', 'Dark');
  static String get dark => _dark.tr;

  static final _light = s('light', 'Light', 'Light');
  static String get light => _light.tr;

  static final _systemWideSetting = s(
    'systemWideSetting',
    'System-wide setting',
    'System-wide setting',
  );
  static String get systemWideSetting => _systemWideSetting.tr;

  static final _accounts = s('accounts', 'Accounts', 'Accounts');
  static String get accounts => _accounts.tr;

  static final _createAccount = s(
    'createAccount',
    'Create a new Account',
    'Create a new Account',
  );
  static String get createAccount => _createAccount.tr;

  static final _addExistingAccount = s(
    'addExistingAccount',
    'Add an existing account',
    'Add an existing account',
  );
  static String get addExistingAccount => _addExistingAccount.tr;
}
