class FeedData {
  final int feedId;
  final List<int> spaceIds;
  final String feedType;
  final String? image;
  final String title;
  final String description;
  final int authorId;
  final String authorUrl;
  final int createdAt;

  final int? rewards;
  final int likes;
  final int comments;
  final int reposts;

  const FeedData({
    required this.feedId,
    required this.spaceIds,
    required this.feedType,
    required this.image,
    required this.title,
    required this.description,
    required this.authorId,
    required this.authorUrl,
    required this.createdAt,

    required this.rewards,
    required this.likes,
    required this.comments,
    required this.reposts,
  });
}
