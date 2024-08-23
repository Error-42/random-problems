#![allow(dead_code)]

use rand::seq::SliceRandom;
use serde::Deserialize;

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields, tag = "status", rename_all = "UPPERCASE")]
enum ProblemsetProblems {
    Ok { result: ProblemsetProblemsResult },
    Failed { comment: String },
}

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
struct ProblemsetProblemsResult {
    problems: Vec<Problem>,
    problem_statistics: Vec<ProblemStatistics>,
}

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
struct Problem {
    /// Id of the contest, containing the problem.
    contest_id: Option<usize>,
    /// Short name of the problemset the problem belongs to.
    problemset_name: Option<String>,
    /// Usually, a letter or letter with digit(s) indicating the problem index in a contest.
    index: String,
    /// Localized.
    name: String,
    #[serde(rename = "type")]
    ty: ProblemType,
    /// Maximum amount of points for the problem.
    points: Option<f64>,
    /// Problem rating (difficulty).
    rating: Option<usize>,
    /// Problem tags.
    tags: Vec<String>,
}

#[derive(Clone, Copy, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
enum ProblemType {
    Programming,
    Question,
}

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
struct ProblemStatistics {
    /// Id of the contest, containing the problem.
    contest_id: Option<usize>,
    /// Usually, a letter or letter with digit(s) indicating the problem index in a contest.
    index: String,
    /// Number of users, who solved the problem.
    solved_count: usize,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://codeforces.com/api/problemset.problems";

    let response = minreq::get(url).send()?;

    if !(200..300).contains(&response.status_code) {
        println!("{}; {}", response.status_code, response.reason_phrase);
        return Ok(());
    }

    println!("GET COMPLETE!");

    // println!("{}", response.as_str()?);

    let deserialized: ProblemsetProblems = serde_json::from_str(response.as_str()?)?;

    let ProblemsetProblems::Ok { result } = deserialized else {
        panic!();
    };

    let mut rng = rand::thread_rng();

    for rating in (800..=3500).step_by(200) {
        let problems: Vec<&Problem> = result
            .problems
            .iter()
            .filter(|p| p.rating == Some(rating) && p.contest_id.is_some())
            .collect();

        let Some(problem) = problems.choose(&mut rng) else {
            continue;
        };

        println!(
            "[{}{}: {}](https://codeforces.com/problemset/problem/{}/{})",
            problem.contest_id.unwrap(),
            problem.index,
            problem.name,
            problem.contest_id.unwrap(),
            problem.index,
        );
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize() {
        let url = "https://codeforces.com/api/problemset.problems";

        let response = minreq::get(url).send().unwrap();

        if !(200..300).contains(&response.status_code) {
            panic!()
        }

        let deserialized: ProblemsetProblems =
            serde_json::from_str(response.as_str().unwrap()).unwrap();

        assert!(matches!(deserialized, ProblemsetProblems::Ok { result: _ }));
    }
}
