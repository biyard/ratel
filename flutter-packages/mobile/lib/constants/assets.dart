import 'package:ratel/exports.dart';

abstract class Assets {
  static const String introLogo = 'assets/images/logo.png';
  static const String logoLetter = 'assets/images/logo_letter.png';
  static const String logoSquare = 'assets/images/logo_square.png';
  static const String back = 'assets/icons/back.svg';
  static const String add = 'assets/icons/add.svg';
  static const String passport = 'assets/icons/passport.svg';

  static const String badge = 'assets/icons/badge.svg';
  static const String bookmark = 'assets/icons/bookmark.svg';
  static const String bookmarkFilled = 'assets/icons/bookmark_filled.svg';
  static const String coin = 'assets/icons/coin.svg';
  static const String coin2 = 'assets/icons/coin_2.svg';
  static const String edit1 = 'assets/icons/edit_1.svg';
  static const String extra = 'assets/icons/extra.svg';
  static const String feed = 'assets/icons/feed.svg';
  static const String palace = 'assets/icons/palace.svg';
  static const String reward = 'assets/icons/reward.svg';
  static const String roundBubble = 'assets/icons/round_bubble.svg';
  static const String thumbs = 'assets/icons/thumbs.svg';
  static const String exchange2 = 'assets/icons/exchange2.svg';
  static const String rewardCoin = 'assets/icons/reward_coin.svg';
  static const String botCoin = 'assets/icons/botcoin_coin.svg';
  static const String bot = 'assets/icons/bot.svg';
  static const String file = 'assets/icons/file.svg';
  static const String setting = 'assets/icons/setting.svg';

  static const String editContent = 'assets/icons/edit_content.svg';
  static const String folder = 'assets/icons/folder.svg';
  static const String option = 'assets/icons/option.svg';
  static const String star = 'assets/icons/star.svg';
  static const String verification = 'assets/icons/verification.svg';
  static const String dark = 'assets/icons/dark.svg';

  static const String filter = 'assets/icons/filter.svg';
  static const String plus = 'assets/icons/plus.svg';
  static const String chat = 'assets/icons/chat.svg';
  static const String home = 'assets/icons/home.svg';
  static const String mail = 'assets/icons/mail.svg';
  static const String notification = 'assets/icons/notification.svg';
  static const String people = 'assets/icons/people.svg';
  static const String exchange = 'assets/icons/exchange.svg';
  static const String send = 'assets/icons/send.svg';
  static const String verified = 'assets/icons/verified.svg';
  static const String roundedPlus = 'assets/icons/rounded_plus.svg';
  static const String user = 'assets/icons/user.svg';
  static const String credentialBadge = 'assets/icons/credential_badge.svg';
  static const String solarStar = 'assets/icons/solar_star.svg';

  static const String logo = 'assets/icons/logo.png';
  static const String favicon = 'assets/icons/favicon.svg';
  static const String home1 = 'assets/images/home_1.svg';
  static const String internet = 'assets/images/internet.svg';
  static const String userGroup = 'assets/images/user_group.svg';
  static const String bell = 'assets/images/bell.svg';
  static const String search = 'assets/images/search.svg';
  static const String google = 'assets/images/google.svg';
  static const String apple = 'assets/images/apple.svg';
  static const String email = 'assets/images/email.svg';

  static const String upload = 'assets/icons/upload.svg';
  static const String ai = 'assets/icons/ai.svg';

  static const String docx = 'assets/icons/docx.svg';
  static const String jpg = 'assets/icons/jpg.svg';
  static const String mov = 'assets/icons/mov.svg';
  static const String mp4 = 'assets/icons/mp4.svg';
  static const String pdf = 'assets/icons/pdf.svg';
  static const String png = 'assets/icons/png.svg';
  static const String pptx = 'assets/icons/pptx.svg';
  static const String xlsx = 'assets/icons/xlsx.svg';
  static const String zip = 'assets/icons/zip.svg';

  static const String record = 'assets/icons/record.svg';
  static const String play = 'assets/icons/play.svg';
  static const String delete2 = 'assets/icons/delete_2.svg';
  static const String warning = 'assets/icons/warning.svg';

  static const String bold = 'assets/icons/bold.svg';
  static const String bottomLine = 'assets/icons/bottom_line.svg';
  static const String bullet = 'assets/icons/bullet.svg';
  static const String h1 = 'assets/icons/h1.svg';
  static const String h2 = 'assets/icons/h2.svg';
  static const String h3 = 'assets/icons/h3.svg';
  static const String italic = 'assets/icons/italic.svg';
  static const String keyboard = 'assets/icons/keyboard.svg';

  static final addIcon = SvgPicture.asset(add, width: 15, height: 15);
  static final backIcon = SvgPicture.asset(back, width: 16, height: 16);
  static final badgeImage = SvgPicture.asset(badge, width: 20, height: 20);
  static final logoImage = Image.asset(logo, width: 40, height: 40);
  static final googleImage = SvgPicture.asset(google, width: 24, height: 24);
  static final appleImage = SvgPicture.asset(apple, width: 24, height: 24);
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
