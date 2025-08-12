import 'package:ratel/exports.dart';

abstract class Assets {
  static const String introLogo = 'assets/images/logo.png';
  static const String logoLetter = 'assets/images/logo_letter.png';
  static const String back = 'assets/icons/back.svg';
  static const String add = 'assets/icons/add.svg';
  static const String passport = 'assets/icons/passport.svg';

  static const String badge = 'assets/icons/badge.svg';
  static const String bookmark = 'assets/icons/bookmark.svg';
  static const String coin = 'assets/icons/coin.svg';
  static const String edit1 = 'assets/icons/edit_1.svg';
  static const String extra = 'assets/icons/extra.svg';
  static const String feed = 'assets/icons/feed.svg';
  static const String palace = 'assets/icons/palace.svg';
  static const String reward = 'assets/icons/reward.svg';
  static const String roundBubble = 'assets/icons/roundBubble.svg';
  static const String thumbs = 'assets/icons/thumbs.svg';

  static const String filter = 'assets/icons/filter.svg';
  static const String plus = 'assets/icons/plus.svg';
  static const String chat = 'assets/icons/chat.svg';
  static const String home = 'assets/icons/home.svg';
  static const String mail = 'assets/icons/mail.svg';
  static const String notification = 'assets/icons/notification.svg';
  static const String people = 'assets/icons/people.svg';

  static const String logo = 'assets/icons/logo.png';
  static const String favicon = 'assets/icons/favicon.svg';
  static const String home1 = 'assets/images/home_1.svg';
  static const String internet = 'assets/images/internet.svg';
  static const String userGroup = 'assets/images/user_group.svg';
  static const String bell = 'assets/images/bell.svg';
  static const String search = 'assets/images/search.svg';
  static const String google = 'assets/images/google.svg';

  static final addIcon = SvgPicture.asset(add, width: 15, height: 15);
  static final backIcon = SvgPicture.asset(back, width: 16, height: 16);
  static final badgeImage = SvgPicture.asset(badge, width: 20, height: 20);
  static final logoImage = Image.asset(logo, width: 40, height: 40);
  static final googleImage = SvgPicture.asset(google, width: 24, height: 24);
  static final bellImage = SvgPicture.asset(
    bell,
    width: 32,
    height: 32,
    colorFilter: ColorFilter.mode(AppColors.neutral500, BlendMode.srcIn),
  );
  static final searchImage = SvgPicture.asset(
    search,
    width: 32,
    height: 32,
    colorFilter: ColorFilter.mode(AppColors.neutral500, BlendMode.srcIn),
  );
  static final home1ActiveImage = SvgPicture.asset(
    home1,
    width: 32,
    height: 32,
    colorFilter: ColorFilter.mode(Colors.white, BlendMode.srcIn),
  );
  static final home1Image = SvgPicture.asset(
    home1,
    width: 32,
    height: 32,
    colorFilter: ColorFilter.mode(AppColors.neutral500, BlendMode.srcIn),
  );

  static final internetActiveImage = SvgPicture.asset(
    internet,
    width: 32,
    height: 32,
    colorFilter: ColorFilter.mode(Colors.white, BlendMode.srcIn),
  );
  static final internetImage = SvgPicture.asset(
    internet,
    width: 32,
    height: 32,
    colorFilter: ColorFilter.mode(AppColors.neutral500, BlendMode.srcIn),
  );

  static final roundBubbleActiveImage = SvgPicture.asset(
    roundBubble,
    width: 32,
    height: 32,
    colorFilter: ColorFilter.mode(Colors.white, BlendMode.srcIn),
  );
  static final roundBubbleImage = SvgPicture.asset(
    roundBubble,
    width: 32,
    height: 32,
    colorFilter: ColorFilter.mode(AppColors.neutral500, BlendMode.srcIn),
  );

  static final userGroupActiveImage = SvgPicture.asset(
    userGroup,
    width: 32,
    height: 32,
    colorFilter: ColorFilter.mode(Colors.white, BlendMode.srcIn),
  );
  static final userGroupImage = SvgPicture.asset(
    userGroup,
    width: 32,
    height: 32,
    colorFilter: ColorFilter.mode(AppColors.neutral500, BlendMode.srcIn),
  );
}
