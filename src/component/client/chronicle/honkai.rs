use reqwest::Response;

use crate::component::client::base::InnerClient;
use crate::component::client::chronicle::client::Chronicle;
use crate::model::ModelBase;
use crate::model::honkai;
use crate::types;
use crate::types::{Game, IDOr};
use crate::util::kwargs::Kwargs;
use crate::util::uid::recognize_honkai_server;

#[derive(Debug, Clone)]
pub(crate) struct HonkaiClient(pub(crate) InnerClient<'static>);


impl HonkaiClient {
    async fn inner_get_honkai_record(
        &self, endpoint: &str, uid: u32, method: Option<&str>, lang: Option<types::Languages>, payload: Option<Kwargs>, _cache: Option<bool>
    ) -> anyhow::Result<Response> {
        let mut payload = payload.unwrap_or_else(|| Kwargs::new());
        payload.set("role_id", uid);
        payload.set("server", recognize_honkai_server(&uid).unwrap());

        let mut kwargs = Kwargs::new();
        let method =  method.unwrap_or("GET");

        if method.eq("GET") {
            kwargs.set("params", payload);
        } else {
            kwargs.set("data", payload);
        };

        let data = self.0.request_game_record(
            endpoint,
            method,
            lang,
            Some(types::Region::OVERSEAS),
            Some(Game::HONKAI),
            Some(kwargs)
        )
            .await
            .unwrap();
        Ok(data)
    }

    pub(crate) async fn get_notes(&self, uid: Option<u32>, lang: Option<&str>, auto_auth: Option<bool>) -> anyhow::Result<()> {
        todo!()
    }

    pub(crate) async fn get_user(&self, uid: Option<u32>, lang: Option<types::Languages>) -> anyhow::Result<()> {
        let result = self.inner_get_honkai_record("index", uid.unwrap(), None, lang, None, None)
            .await
            .unwrap();

        let model = match result.json::<ModelBase<honkai::user::Test>>().await {
            Ok(success) => Ok(success),
            Err(_) => {
                self.0.update_settings(IDOr::Int(2), true, Some(Game::HONKAI)).await.unwrap();
                let string = self.inner_get_honkai_record("index", uid.unwrap(), None, lang, None, None)
                    .await.unwrap().text().await.unwrap();
                dbg!(string);


                Err(())
            }
        };

        dbg!(model.unwrap());

        Ok(())
    }

    pub(crate) async fn get_characters(&self, uid: Option<u32>, lang: Option<&str>) -> anyhow::Result<()> {
        todo!()
    }

    pub(crate) async fn get_challenge(&self, uid: Option<u32>, previous: Option<bool>, lang: Option<&str>) -> anyhow::Result<()> {
        todo!()
    }

    pub(crate) async fn get_rouge(&self, uid: Option<u32>, schedule_type: Option<&str>, lang: Option<&str>) -> anyhow::Result<()> {
        todo!()
    }
}

// FUCKFUCKFUCKFUCK

impl Chronicle<HonkaiClient> {
    pub(crate) fn new() -> Self {
        Chronicle(HonkaiClient(InnerClient::default()))
    }

}

