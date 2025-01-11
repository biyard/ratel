#[derive(Debug, Clone, Copy)]
pub struct Controller {
    pub politicians: Resource<CommonQueryResponse<Politician>>,
}

impl Controller {
    pub fn new() -> Result<Self, RenderError> {
        let politician_api: PoliticianService = use_context();

        let politicians = use_server_future(move || async move {
            match politician_api.list_politicians(10).await {
                Ok(res) => res,
                Err(e) => {
                    tracing::error!("list politicians error: {:?}", e);
                    CommonQueryResponse::<Politician>::default()
                }
            }
        })?;

        let ctrl = Self { politicians };
        use_context_provider(|| ctrl);

        Ok(ctrl)
    }
}