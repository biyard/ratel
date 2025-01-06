use reqwest::Error;
use std::collections::HashMap;
use serde_json::Value;

pub struct OpenAPI {
    pub active_member_url: String,
}

impl OpenAPI {
    pub fn new() -> Self {
        Self {
            active_member_url: "https://open.assembly.go.kr/portal/openapi/nwvrqwxyaytdsfvhu".to_string(),
        }
    }

    pub async fn get_active_members(
        &self, 
        p_index: Option<String>, // 페이지번호 default: 1
        p_size: Option<String>, // 페이지당 요청 건수 default: 10
        name: Option<String>, // 국회의원명
        party: Option<String>, // 정당명
        district: Option<String>, // 선거구명
        committee: Option<String>, // 소속위원회 명
        code: Option<String>, // 국회의원코드
    ) -> Result<Value, Error> {
        let config = crate::config::get();
        let mut params = HashMap::new();
        params.insert("KEY", config.openapi_key.to_string());
        params.insert("type", "json".to_string());
        params.insert("pIndex", p_index.unwrap_or_else(|| "1".to_string()));
        params.insert("pSize", p_size.unwrap_or_else(|| "300".to_string()));

        if let Some(hg_nm) = name {
            params.insert("HG_NM", hg_nm);
        }
        if let Some(poly_nm) = party {
            params.insert("POLY_NM", poly_nm);
        }
        if let Some(orig_nm) = district {
            params.insert("ORIG_NM", orig_nm);
        }
        if let Some(cmits) = committee {
            params.insert("CMITS", cmits);
        }
        if let Some(mona_cd) = code {
            params.insert("MONA_CD", mona_cd);
        }

        let client = reqwest::Client::new();
        let response = client
            .get(&self.active_member_url)
            .query(&params)
            .header(reqwest::header::USER_AGENT, "biyard") // 필수
            .send()
            .await?
            .text()
            .await?;

        if let Ok(json) = serde_json::from_str::<Value>(&response) {
            let response = json["nwvrqwxyaytdsfvhu"].clone();
            return Ok(response[1].clone());
        }

        Ok(Value::Null)
    }
}