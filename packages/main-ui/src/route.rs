use crate::pages::*;
use bdk::prelude::*;

#[derive(Clone, Routable, Debug, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(SocialLayout)]
        #[layout(MyPageLayout)]
            #[route("/social")]
            IndexPage {},
        #[end_layout]

        #[nest("/threads")]
            #[nest("/:feed_id")]
                #[nest("/spaces")]
                    #[nest("/:id")]
                        #[layout(DeliberationSettingLayout)]
                            #[route("/deliberations/summary")]
                            DeliberationSummary { feed_id: i64, id: i64 },
                            #[route("/deliberations/deliberation")]
                            Deliberation { feed_id: i64, id: i64 },
                            #[route("/deliberations/final-consensus")]
                            DeliberationFinalConsensus { feed_id: i64, id: i64 },
                            #[route("/deliberations/poll")]
                            DeliberationPoll { feed_id: i64, id: i64 },
                        #[end_layout]

                        #[layout(LegislationSettingLayout)]
                            #[route("/legislations/summary")]
                            LegislationSummary { feed_id: i64, id: i64 },
                        #[end_layout]

                        #[layout(NftSettingLayout)]
                            #[route("/nfts/summary")]
                            NftSummary { feed_id: i64, id: i64 },
                            #[route("/nfts/nft")]
                            Nft { feed_id: i64, id: i64 },
                        #[end_layout]

                        #[layout(PollSettingLayout)]
                            #[route("/polls/summary")]
                            PollSummary { feed_id: i64, id: i64 },
                            #[route("/polls/poll")]
                            Poll { feed_id: i64, id: i64 },
                        #[end_layout]
                    #[end_nest]
                #[end_nest]
            #[end_nest]
        #[end_nest]

        #[route("/threads/:id")]
        ThreadPage { id: i64 },
        #[route("/threads/:feed_id/spaces/:id")]
        SpacePage {feed_id: i64, id: i64},
        #[route("/explore")]
        ExplorePage {},
        #[route("/my-network")]
        MyNetworkPage {},
        #[route("/message")]
        MessagesPage {},
        #[route("/notifications")]
        NotificationsPage {},
        #[route("/my-profile")]
        MyProfilePage {},
        #[route("/politicians")]
        PoliticiansPage {},
        #[route("/politicians/:id")]
        PoliticiansByIdPage { id: i64 },
        #[route("/presidential-election")]
        PresidentialElectionPage {},

        #[nest("/teams/:teamname")]
    #[layout(TeamsByIdLayout)]
    #[route("/")]
    TeamsByIdPage { teamname: String },
    #[end_layout]
    #[end_nest]
    #[end_layout]

    #[layout(LandingLayout)]
        #[route("/")]
        LandingPage {},
        #[route("/privacy-policy")]
        PrivacyPolicyPage {},
        #[route("/preparing")]
        PreparingPage {},
        #[route("/become-sponsor")]
        BecomeSponsorPage {},
        #[route("/quizzes")]
        QuizzesPage {},

        #[route("/politician")]
        PoliticiansPageForLanding {},
        #[route("/presidential-elections")]
        PresidentialElectionPageForLanding {},

        #[route("/advocacy-campaigns/:id")]
        AdvocacyCampaignsByIdPage { id: i64},

        #[route("/quizzes/results/:id")]
        ResultsPage {id: String},
    #[end_layout]

    #[route("/:..route")]
    NotFoundPage { route: Vec<String> },
}
