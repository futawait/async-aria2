use super::json::{JsonResult, Params, Request, Response, Statistics, Status};
pub struct Http {
    client: reqwest::Client,
    token: String,
}

impl Http {
    pub fn new(token: &str) -> Self {
        Http {
            client: reqwest::Client::new(),
            token: token.to_owned(),
        }
    }
    fn create_request(method: &str, params: Params) -> String {
        let request = Request {
            jsonrpc: "2.0".to_owned(),
            id: "".to_owned(),
            method: format!("aria2.{}", method),
            params,
        };
        serde_json::to_string(&request).unwrap()
    }
    pub async fn call_method(&self, method: &str, params: Params) -> JsonResult {
        let request = Self::create_request(method, params);
        let res = self
            .client
            .post("http://localhost:6800/jsonrpc")
            .body(request)
            .send()
            .await
            .unwrap();
        let res = res.text().await.unwrap();
        let res: Response = serde_json::from_str(&res).unwrap();
        res.result
    }
    pub async fn change_global_option(&self, params: Params) {
        self.call_method("changeGlobalOption", params).await;
    }
    pub async fn add_uri(&self, uri: &str) -> String {
        let result = self
            .call_method(
                "addUri",
                Params::SV(self.token.clone(), vec![uri.to_owned()]),
            )
            .await;
        match result {
            JsonResult::S(s) => s,
            _ => panic!(),
        }
    }
    pub async fn add_torrent(&self, torrent: String) -> String {
        let result = self
            .call_method("addTorrent", Params::V(vec![self.token.clone(), torrent]))
            .await;
        match result {
            JsonResult::S(s) => s,
            _ => panic!(),
        }
    }
    pub async fn tell_status(&self, gid: &str) -> Status {
        let result = self
            .call_method(
                "tellStatus",
                Params::V(vec![self.token.clone(), gid.to_owned()]),
            )
            .await;
        match result {
            JsonResult::Status(status) => status,
            _ => panic!(),
        }
    }
    pub async fn tell_active(&self) -> Vec<Status> {
        let result = self
            .call_method("tellActive", Params::V(vec![self.token.clone()]))
            .await;
        match result {
            JsonResult::StatusList(status_list) => status_list,
            _ => panic!(),
        }
    }
    pub async fn tell_waiting(&self) -> Vec<Status> {
        let result = self
            .call_method("tellWaiting", Params::SUU(self.token.clone(), 0, 20))
            .await;
        match result {
            JsonResult::StatusList(status_list) => status_list,
            _ => panic!(),
        }
    }
    pub async fn tell_stopped(&self) -> Vec<Status> {
        let result = self
            .call_method("tellStopped", Params::SUU(self.token.clone(), 0, 20))
            .await;
        match result {
            JsonResult::StatusList(status_list) => status_list,
            _ => panic!(),
        }
    }
    pub async fn pause(&self, gid: &str) {
        self.call_method("pause", Params::V(vec![self.token.clone(), gid.to_owned()]))
            .await;
    }
    pub async fn unpause(&self, gid: &str) {
        self.call_method(
            "unpause",
            Params::V(vec![self.token.clone(), gid.to_owned()]),
        )
        .await;
    }
    pub async fn pause_all(&self) {
        self.call_method("pauseAll", Params::V(vec![self.token.clone()]))
            .await;
    }
    pub async fn unpause_all(&self) {
        self.call_method("unpauseAll", Params::V(vec![self.token.clone()]))
            .await;
    }
    pub async fn remove(&self, gid: &str) {
        self.call_method(
            "remove",
            Params::V(vec![self.token.clone(), gid.to_owned()]),
        )
        .await;
    }
    pub async fn get_global_stat(&self) -> Statistics {
        let result = self
            .call_method("getGlobalStat", Params::V(vec![self.token.clone()]))
            .await;
        match result {
            JsonResult::Statistics(statistics) => statistics,
            _ => panic!(),
        }
    }
    pub async fn shutdown(&self) {
        self.call_method("shutdown", Params::V(vec![self.token.clone()]))
            .await;
    }
}
