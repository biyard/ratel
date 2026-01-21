import 'package:ratel/exports.dart';

export 'screens/create/create_post_binding.dart';
export 'screens/create/create_post_controller.dart';
export 'screens/create/create_post_screen.dart';

export 'screens/list/post_binding.dart';
export 'screens/list/post_controller.dart';
export 'screens/list/post_screen.dart';

export 'screens/detail/detail_post_binding.dart';
export 'screens/detail/detail_post_controller.dart';
export 'screens/detail/detail_post_screen.dart';

const String createPostScreen = '/posts/new';
const String postScreen = '/post';
const String myPostsScreen = '/my-posts';
const String spaceScreen = '/space';
String postWithPk(String postPk) {
  return '$postScreen/${Uri.encodeComponent(postPk)}';
}

String spaceWithPk(String spacePk) {
  return '$spaceScreen/${Uri.encodeComponent(spacePk)}';
}

List<GetPage> postPages = [
  GetPage(
    name: myPostsScreen,
    page: () => const PostScreen(),
    binding: PostBinding(),
    customTransition: SlideOverTransition(),
    transitionDuration: const Duration(milliseconds: 300),
    opaque: true,
    curve: Curves.easeOutCubic,
  ),
  GetPage(
    name: '/post/:pk',
    page: () => DetailPostScreen(),
    binding: DetailPostBinding(),
    customTransition: SlideOverTransition(),
    transitionDuration: const Duration(milliseconds: 300),
    opaque: true,
    curve: Curves.easeOutCubic,
  ),
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
