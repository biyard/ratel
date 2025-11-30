import 'package:ratel/exports.dart';

export 'screens/create/create_post_binding.dart';
export 'screens/create/create_post_controller.dart';
export 'screens/create/create_post_screen.dart';

const String createPostScreen = '/posts/new';

List<GetPage> postPages = [
  GetPage(
    name: createPostScreen,
    page: () => const CreatePostScreen(),
    binding: CreatePostBinding(),
    customTransition: SlideOverTransition(),
    transitionDuration: const Duration(milliseconds: 300),
    opaque: true,
    curve: Curves.easeOutCubic,
  ),
];
