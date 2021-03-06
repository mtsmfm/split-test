use anyhow::{bail, Result};
use glob::glob;
use log::Level::Debug;
use log::{debug, log_enabled, warn};
use quick_xml::de::from_reader;
use serde::Deserialize;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs::canonicalize;
use std::fs::File;
use std::io::BufReader;
use std::iter::FromIterator;
use std::path::Path;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(short, long)]
    debug: bool,
    #[structopt(long, required = true)]
    tests_glob: Vec<String>,
    #[structopt(long)]
    node_index: usize,
    #[structopt(long)]
    node_total: usize,
    #[structopt(long)]
    junit_xml_report_dir: PathBuf,
}

#[derive(Debug, Deserialize, PartialEq)]
struct TestResultXml {
    #[serde(rename = "testsuite", alias = "testcase", default)]
    test_results: Vec<TestResult>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct TestResult {
    #[serde(alias = "filepath", default)]
    file: Option<PathBuf>,
    time: f64,
    #[serde(rename = "testsuite", alias = "testcase", default)]
    test_results: Vec<TestResult>,
}

struct TestResultData {
    file: Option<PathBuf>,
    time: f64,
}

impl IntoIterator for TestResultXml {
    type IntoIter = IntoIter;
    type Item = TestResultData;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            remaining: self.test_results.into_iter().collect(),
        }
    }
}

struct IntoIter {
    remaining: VecDeque<TestResult>,
}

impl Iterator for IntoIter {
    type Item = TestResultData;

    fn next(&mut self) -> Option<Self::Item> {
        self.remaining.pop_front().and_then(
            |TestResult {
                 file,
                 time,
                 test_results,
             }| {
                self.remaining.extend(test_results);

                Some(TestResultData {
                    file: file,
                    time: time,
                })
            },
        )
    }
}

struct Node<'a> {
    test_files: Vec<&'a PathBuf>,
    recorded_total_time: f64,
}

impl<'a> Node<'a> {
    fn add(&mut self, test_file: &'a PathBuf, time: f64) {
        self.test_files.push(test_file);
        self.recorded_total_time += time;
    }
}

fn expand_globs(patterns: &Vec<String>) -> Result<Vec<PathBuf>> {
    let mut files = HashSet::new();

    for pattern in patterns {
        for path in glob(&pattern)? {
            files.insert(canonicalize(path?)?);
        }
    }

    let mut files = files.into_iter().collect::<Vec<_>>();
    files.sort();

    Ok(files.to_vec())
}

fn get_test_file_results(
    junit_xml_report_dir: &PathBuf,
) -> Result<HashMap<std::path::PathBuf, f64>> {
    let xml_glob_path_buf = Path::new(junit_xml_report_dir).join("**/*.xml");
    let xml_glob = match xml_glob_path_buf.to_str() {
        Some(x) => x,
        None => bail!("--junit-xml-dir error {:?}", xml_glob_path_buf),
    };

    let mut test_file_results = HashMap::new();

    for xml_path in expand_globs(&vec![String::from(xml_glob)])? {
        let reader = BufReader::new(File::open(xml_path)?);
        let test_result_xml: TestResultXml = from_reader(reader)?;

        for TestResultData { file, time } in test_result_xml {
            file.map(|f| {
                canonicalize(f).map(|normalized_file| {
                    let total_time = test_file_results.entry(normalized_file).or_insert(0.0);
                    *total_time += time;
                })
            });
        }
    }

    debug!("{:?}", test_file_results);

    Ok(test_file_results)
}

fn main() -> Result<()> {
    let args = Opt::from_args();
    if args.debug {
        std::env::set_var("RUST_LOG", "debug");
    } else {
        std::env::set_var("RUST_LOG", "info");
    }
    env_logger::init();

    let mut nodes = Vec::from_iter((0..args.node_total).map(|_| Node {
        test_files: Vec::new(),
        recorded_total_time: 0.0,
    }));

    let test_file_results = get_test_file_results(&args.junit_xml_report_dir)?;

    let test_files = expand_globs(&args.tests_glob)?;
    if test_files.len() == 0 {
        bail!("Test file is not found. pattern: {:?}", args.tests_glob);
    }

    let (mut recorded_test_files, not_recorded_test_files): (Vec<_>, Vec<_>) = test_files
        .iter()
        .partition(|&f| test_file_results.contains_key(f));

    recorded_test_files.sort_by(|&a, &b| {
        let v1 = test_file_results.get(a).unwrap();
        let v2 = test_file_results.get(b).unwrap();
        v2.partial_cmp(v1).unwrap()
    });

    for test_file in recorded_test_files {
        nodes.sort_by(|a, b| {
            a.recorded_total_time
                .partial_cmp(&b.recorded_total_time)
                .unwrap()
        });
        nodes
            .get_mut(0)
            .unwrap()
            .add(test_file, *test_file_results.get(test_file).unwrap());
    }

    for (i, test_file) in not_recorded_test_files.iter().enumerate() {
        warn!("Timing data not found: {}", test_file.to_str().unwrap());
        let len = nodes.len();
        nodes.get_mut(i % len).unwrap().add(test_file, 0.0);
    }

    if log_enabled!(Debug) {
        for (i, node) in nodes.iter().enumerate() {
            debug!(
                "node {}: recorded_total_time: {}",
                i, node.recorded_total_time
            );

            for test_file in node.test_files.iter() {
                debug!("{}", test_file.to_str().unwrap());
            }
        }
    }

    for test_file in nodes.get(args.node_index).unwrap().test_files.iter() {
        println!("{}", test_file.to_str().unwrap());
    }

    Ok(())
}

/*
TODOS:

- Test <testsuites><testsuite file="/foo" time="0.1"></testsuite></testsuites>
- Test <testsuite><testcase file="./foo" time="0.1"></testcase></testsuite>
*/
