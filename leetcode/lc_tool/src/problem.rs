use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use ureq::{json, serde_json};
const GRAPHQL_URL: &str = "https://leetcode.com/graphql";
const QUESTION_LIST_OPERATION: &str = "allQuestionsRaw";
const QUESTION_LIST_QUERY_STRING: &str = r#"
query allQuestionsRaw {
  allQuestions: allQuestionsRaw {
    titleSlug
    questionId
    questionFrontendId
    isPaidOnly
  }
}
"#;
const QUESTION_QUERY_STRING: &str = r#"
query questionData($titleSlug: String!) {
    question(titleSlug: $titleSlug) {
        content
        stats
        codeDefinition
        sampleTestCase
		title
		titleSlug
		difficulty
    }
}"#;
const QUESTION_QUERY_OPERATION: &str = "questionData";

static UREQ_AGENT: Lazy<ureq::Agent> = Lazy::new(|| {
	if let Ok(url) = std::env::var("all_proxy") {
		ureq::AgentBuilder::new()
			.proxy(ureq::Proxy::new(url).unwrap())
			.build()
	} else {
		ureq::Agent::new()
	}
});

pub fn get_daily_id() -> anyhow::Result<u32> {
	Ok(UREQ_AGENT
		.post(GRAPHQL_URL)
		.send_json(Query::daily_problem_query())?
		.into_json::<serde_json::Value>()?["data"]["activeDailyCodingChallengeQuestion"]["question"]
		["frontendQuestionId"]
		.as_str()
		.unwrap()
		.parse::<u32>()?)
}

pub fn get_problem(id: u32) -> anyhow::Result<Problem> {
	let id = id.to_string();
	let problem = get_problems()?
		.into_iter()
		.find(|p| p.question_frontend_id == id)
		.ok_or_else(|| anyhow::format_err!("problem-{id} not found"))?;

	anyhow::ensure!(!problem.is_paid_only, "this problem is paid only");

	let resp = UREQ_AGENT
		.post(GRAPHQL_URL)
		.send_json(Query::question_query(&problem.title_slug))?
		.into_json::<ProblemDetailResp>()?;
	Ok(resp.data.question.into())
}

fn get_problems() -> anyhow::Result<Vec<QuestListEntry>> {
	Ok(UREQ_AGENT
		.post(GRAPHQL_URL)
		.send_json(Query::question_list_query())?
		.into_json::<QuestionListResp>()?
		.data
		.all_questions)
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Problem {
	pub title: String,
	pub title_slug: String,
	pub content: String,
	pub code_definition: Vec<CodeDefinition>,
	pub sample_test_case: String,
	pub difficulty: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CodeDefinition {
	pub value: String,
	pub text: String,
	pub default_code: String,
}

// Graphql struct
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct Query {
	operation_name: &'static str,
	variables: serde_json::Value,
	query: &'static str,
}

impl Query {
	fn question_query(title_slug: &str) -> Query {
		Query {
			operation_name: QUESTION_QUERY_OPERATION,
			variables: json!({ "titleSlug": title_slug }),
			query: QUESTION_QUERY_STRING,
		}
	}

	fn question_list_query() -> Query {
		Query {
			operation_name: QUESTION_LIST_OPERATION,
			variables: json!({}),
			query: QUESTION_LIST_QUERY_STRING,
		}
	}

	fn daily_problem_query() -> Query {
		Query {
			operation_name: "",
			variables: json!({}),
			query: r#"
	query questionOfToday {
	  activeDailyCodingChallengeQuestion {
		question {
		  frontendQuestionId: questionFrontendId
		  titleSlug
		}
	  }
	}
"#,
		}
	}
}

#[derive(Debug, Deserialize)]
struct ProblemDetailResp {
	data: ProblemDetailData,
}

#[derive(Debug, Deserialize)]
struct ProblemDetailData {
	question: ProblemDetail,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ProblemDetail {
	content: String,
	//stats: String,
	code_definition: String,
	sample_test_case: String,
	//meta_data: String,
	title: String,
	title_slug: String,
	difficulty: String,
}

impl From<ProblemDetail> for Problem {
	fn from(q: ProblemDetail) -> Self {
		Problem {
			title: q.title,
			title_slug: q.title_slug,
			code_definition: serde_json::from_str(&q.code_definition).unwrap(),
			content: q.content,
			sample_test_case: q.sample_test_case,
			difficulty: q.difficulty.to_string(),
		}
	}
}

#[derive(Debug, Deserialize)]
struct QuestionListResp {
	data: QuestionListData,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct QuestionListData {
	all_questions: Vec<QuestListEntry>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct QuestListEntry {
	//question_id: String,
	question_frontend_id: String,
	title_slug: String,
	is_paid_only: bool,
}
