/*
#[derive(Serialize)]
struct WebParams {
    pub command: TelegramWebCommand,
}

#[derive(Serialize)]
pub enum TelegramWebCommand {
    OpenSpacePage { space_id: i64 },
}
*/
export interface TgWebParams {
  command: TelegramWebCommand;
}

export type TelegramWebCommand = {
  OpenSpacePage: {
    space_id: number;
  };
};
