use serde_json::{Value, json};
use std::collections::HashMap;
use std::fmt::write;
use std::{env, fmt};

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

pub struct ApiChoice {
    name: String,
    desc: String,
}

impl ApiChoice {
    pub fn new(name: &str, desc: &str) -> Self {
        ApiChoice {
            name: String::from(name),
            desc: String::from(desc),
        }
    }
}

impl fmt::Display for ApiChoice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let n = &self.name;
        let d = &self.desc;
        write!(f, "\"{n}\": \"{d}\"")
    }
}

pub enum ApiQuestionCriteria {
    Choice(Vec<ApiChoice>),
    Noul(Option<(String, String)>),
    Score(Vec<String>),
}

pub struct ApiQuestion {
    name: String,
    instructions: String,
    criteria: ApiQuestionCriteria,
}

impl ApiQuestion {
    pub fn new(name: &str, instructions: &str, criteria: ApiQuestionCriteria) -> Self {
        ApiQuestion {
            name: String::from(name),
            instructions: String::from(instructions),
            criteria,
        }
    }
}

impl fmt::Display for ApiQuestion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let type_str = match &self.criteria {
            ApiQuestionCriteria::Choice(_) => "choice",
            ApiQuestionCriteria::Noul(_) => "noul",
            ApiQuestionCriteria::Score(_) => "score",
        };

        let contents = match &self.criteria {
            ApiQuestionCriteria::Choice(v) => (String::from("{\n\t")
                + &v.iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(",\n\t")
                + "\n}")
                .replace("\n", "\n\t"),
            ApiQuestionCriteria::Noul(v) => match v {
                Some((x, y)) => format!("{{\n\t\t\"true\": {x}\n\t\t\"false\": {y}\n\t}}"),
                None => String::new(),
            },
            ApiQuestionCriteria::Score(v) => {
                String::from(
                    &v.iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>()
                        .join(",\n\t"),
                ) + "\n}"
            }
        };

        let n = &self.name;
        let i = &self.instructions;

        write!(
            f,
            "\"{n}\": {{\n\t\"type\": \"{type_str}\",\
                   \n\t\"instructions\": \"{i}\",\
                   \n\t\"criteria\": {contents}\n}}"
        )
    }
}

pub struct ApiRequest {
    state: String,
    model: String,
    questions: Vec<ApiQuestion>,
}

impl ApiRequest {
    pub fn new(state: &str, model: &str, questions: Vec<ApiQuestion>) -> Self {
        ApiRequest {
            state: String::from(state),
            model: String::from(model),
            questions,
        }
    }

    pub fn as_json(&self) -> Value {
        json!(&self.to_string())
    }

    pub async fn get_response(&self, t: ApiType) -> Vec<ApiAnswer> {
        let url = t.get_url();
        let api_key = t.get_api_key();

        let client = reqwest::Client::new();
        println!("REQUEST\n{self}");

        let res = client
            .post(url)
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .body(self.to_string())
            .send()
            .await
            .unwrap();

        if res.status() != 200 {
            panic!(
                "{}",
                format!("Invalid request\n\nREQUEST\n{self}\nRESP\n{res:?}")
            );
        }
        let body = res.text().await.unwrap();
        println!("BODY\n{body}\n\n");
        let mut answers: Vec<ApiAnswer> = Vec::new();
        let body_v: Value = serde_json::from_str(&body).expect("Failed to parse Json body response");
        let ans_v = &body_v["answers"];
        for q in &self.questions {
            let qn = &q.name;
            let qv = &ans_v[qn];
            answers.push(ApiAnswer::from_json(qv));
        }
        println!("answers:");
        for a in &answers {
            println!("{}", a);
        }

        answers
    }
}

impl fmt::Display for ApiRequest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = &self.state;
        let m = &self.model;
        let qstr = (String::from("\n")
            + &self
                .questions
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(",\n\t"))
            .replace("\n", "\n\t\t")
            + "\n\t}";

        write!(
            f,
            "{{\
            \n\t\"state\": \"{s}\",\
            \n\t\"model\": \"{m}\",\
            \n\t\"questions\": {{{qstr}\
            \n}}"
        )
    }
}

pub enum ApiAnswer {
    Choice((String, f64)),
    Noul(f64),
    Score(f64),
}

impl ApiAnswer {
    pub fn from_json(v: &Value) -> Self {
        match v["type"]
            .as_str()
            .expect("Failed to parse type from json response")
        {
            "choice" => {
                let choice: String = String::from(
                    v["choice"]
                        .as_str()
                        .expect("Failed to parse choice from json response"),
                );
                let confidence: f64 = v["confidence"]
                    .as_f64()
                    .expect("Failed to parse choice confidence from json response");
                ApiAnswer::Choice((choice, confidence))
            },
            "noul" => {
                let ans: f64 = v["noul"]
                    .as_f64()
                    .expect("Failed to parse noul ans from json response");
                ApiAnswer::Noul(ans)
            },
            "score" => {
                let ans: f64 = v["score"]
                    .as_f64()
                    .expect("Failed to parse score ans from json response");
                ApiAnswer::Score(ans)
            },
            t => panic!("Invalid type \"{}\" in json response", t),
        }
    }
}

impl fmt::Display for ApiAnswer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ApiAnswer::Choice((n,c)) => write!(f, "CHOICE:{n}| {c} confidence"),
            ApiAnswer::Noul(n) => write!(f, "NOUL:{n}"),
            ApiAnswer::Score(s) => write!(f, "Score:{s}"),
        }
    }
}
