use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "UPPERCASE")] // for parsing
pub struct Member {
    hg_nm: String,
    hj_nm: String,
    eng_nm: String,
    bth_gbn_nm: String,
    bth_date: String,
    job_res_nm: String,
    poly_nm: String,
    orig_nm: String,
    elect_gbn_nm: String,
    cmit_nm: String,
    cmits: String,
    reele_gbn_nm: String,
    units: String,
    sex_gbn_nm: String,
    tel_no: String,
    e_mail: String,
    homepage: Option<String>,
    staff: String,
    secretary: String,
    secretary2: String,
    mona_cd: String,
    mem_title: String,
    assem_addr: String,
}

impl Serialize for Member {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // custom serialization
        let mut state = serializer.serialize_struct("Member", 23)?;
        state.serialize_field("name", &self.hg_nm)?;
        state.serialize_field("hj_nm", &self.hj_nm)?;
        state.serialize_field("eng_name", &self.eng_nm)?;
        state.serialize_field("bth_gbn_nm", &self.bth_gbn_nm)?;
        state.serialize_field("bth_date", &self.bth_date)?;
        state.serialize_field("job_res_nm", &self.job_res_nm)?;
        state.serialize_field("poly_nm", &self.poly_nm)?;
        state.serialize_field("orig_nm", &self.orig_nm)?;
        state.serialize_field("elect_gbn_nm", &self.elect_gbn_nm)?;
        state.serialize_field("cmit_nm", &self.cmit_nm)?;
        state.serialize_field("cmits", &self.cmits)?;
        state.serialize_field("reele_gbn_nm", &self.reele_gbn_nm)?;
        state.serialize_field("units", &self.units)?;
        state.serialize_field("sex_gbn_nm", &self.sex_gbn_nm)?;
        state.serialize_field("tel_no", &self.tel_no)?;
        state.serialize_field("e_mail", &self.e_mail)?;
        state.serialize_field("homepage", &self.homepage)?;
        state.serialize_field("staff", &self.staff)?;
        state.serialize_field("secretary", &self.secretary)?;
        state.serialize_field("secretary2", &self.secretary2)?;
        state.serialize_field("mona_cd", &self.mona_cd)?;
        state.serialize_field("mem_title", &self.mem_title)?;
        state.serialize_field("assem_addr", &self.assem_addr)?;
        state.end()
    }
}