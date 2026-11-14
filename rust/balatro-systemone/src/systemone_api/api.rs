use std::collections::HashMap;
use std::{env, fmt};
use serde_json::{json, Value};

/* This will be passed in to specify which api to use */
pub enum ApiType {
    Jev,
}

impl ApiType {
    pub fn get_url(&self) -> &'static str {
        match self {
            ApiType::Jev => "https://api.typesafe.ai/v1/systemone",
        }
    }

    pub fn get_api_key(&self) -> String {
        match self {
            ApiType::Jev => env::var("JEV_API_KEY").unwrap(),
        }
    }
}

struct ApiChoice {
    name: String,
    desc: String,
}

impl ApiChoice {
    pub fn new(name: String, desc: String) -> Self {
        ApiChoice { name, desc }
    }
}

impl fmt::Display for ApiChoice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let n = &self.name;
        let d = &self.name;
        write!(f, "\"{n}\": \"{d}\"")
    }
}

pub enum ApiQuestionCriteria {
    Choice(Vec<ApiChoice>),
    Noul(Option<(String, String)>),
    Score(Vec<String>),
}

struct ApiQuestion {
    name: String,
    instructions: String,
    criteria: ApiQuestionCriteria,
}

impl fmt::Display for ApiQuestion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let type_str = match &self.criteria {
            ApiQuestionCriteria::Choice(_) => "choice",
            ApiQuestionCriteria::Noul(_) => "noul",
            ApiQuestionCriteria::Score(_) => "score",
        };

        let contents = match &self.criteria {
            ApiQuestionCriteria::Choice(v) => {
                String::from(&v.iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(",\n\t"))
                    + "\n}"
            }
            ApiQuestionCriteria::Noul(v) => match v {
                Some((x, y)) => format!("{{\n\t\t\"true\": {x}\n\t\t\"false\": {y}\n\t}}"),
                None => String::new(),
            },
            ApiQuestionCriteria::Score(v) => {
                String::from(&v.iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(",\n\t"))
                    + "\n}"
            }
        };

        let n = &self.name;
        let i = &self.instructions;

        write!(f, "{n} {{\n\t\"type\": {type_str}\n\t\"instructions\": \"{i}\"\n\t\"criteria\": {contents}\n}}")
    }
}

struct ApiRequest {
    state: String,
    model: String,
    questions: Vec<ApiQuestion>
}

impl ApiRequest {
    pub fn as_json(&self) -> Value{
        json!(&self.to_string())
    }

    pub async fn get_response(&self, t: ApiType) {
        let url = t.get_url();
        let api_key = t.get_api_key();
        let payload = &self.as_json();
        
        let client = reqwest::Client::new();
        let res = client.post(url).header("Authorization", format!("Bearer {}", api_key)).json(&payload).send().await;
        
        println!("status {:?}", res);
    }
}

impl fmt::Display for ApiRequest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = &self.state;
        let m = &self.model;
        let qstr = (String::from(&self.questions.iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(",\n\t"))
                    + "\n}").replace("\n", "\n\t");

        write!(f, "{{\n\t\"state\": \"{s}\"\n\t\"model\": \"{m}\"\n\t\"questions\
         {{{qstr}\n}}")
    }
}



struct ApiChoiceAnswer {
    choice: String,
    confidence: f32,
    probabilities: HashMap<String, f32>,
}

pub enum ApiAnswer {
    Choice(ApiChoiceAnswer),
    Noul(f32),
    Score(f32),
}
