use std::process::Stdio;

use std::process::Command;

#[derive(Debug)]
pub(crate) struct Commit {
    pub(crate) hash: String,
    pub(crate) date: String,
    pub(crate) message: String,
}

pub(crate) struct Git {
    pub(crate) commits: Vec<Commit>,
}

impl Git {
    pub(crate) fn log() -> String {
        let r = Command::new("git")
            .arg("log")
            .arg("--pretty=format:%h,%ad,%s")
            .arg("--date=iso")
            .output()
            .expect("Failed to execute git log");

        String::from_utf8_lossy(&r.stdout).to_string()
    }
    
    pub(crate) fn get_first_commit(&self) -> String {
        let r = Command::new("git")
            .arg("rev-list")
            .arg("--max-parents=0")
            .arg("HEAD")
            .output()
            .expect("Failed to execute git log");

        let output = String::from_utf8_lossy(&r.stdout);

        if let Some(first_commit) = output.lines().next() {
           return first_commit.to_string();
        } else {
            panic!("No commit found")
        }

    }

    pub(crate) fn bisect_start(&self) {
        Command::new("git")
            .arg("bisect")
            .arg("start")
            .stdout(Stdio::null())
            .status()
            .expect("Could not start git bisect");
    }

    pub(crate) fn current_hash(&self) {
        Command::new("git")
            .arg("rev-parse")
            .arg("--verify")
            .arg("HEAD")
            .status()
            .expect("Could not get current hash");
    }

    pub(crate) fn bisect_good(&self) -> bool {
        let res = Command::new("git")
            .arg("bisect")
            .arg("good")
            .output()
            .expect("Weird");

        let res = String::from_utf8_lossy(&res.stdout).to_string();
        if res.contains("is the first bad commit") {
            return true;
        }
        return false;
    }

    pub(crate) fn bisect_bad(&self) -> bool {
        let res = Command::new("git")
            .arg("bisect")
            .arg("bad")
            .output()
            .expect("Weird");

        let res = String::from_utf8_lossy(&res.stdout).to_string();

        if res.contains("is the first bad commit") {
            return true;
        }
        return false;
    }

    pub(crate) fn bisect_reset(&self) {
        Command::new("git")
            .arg("bisect")
            .arg("reset")
            .stdout(Stdio::null())
            .output()
            .expect("Weird");
    }

    pub(crate) fn checkout(&self, hash: &String) {
        Command::new("git")
            .arg("checkout")
            .arg(&hash)
            .stdout(Stdio::null())
            .output()
            .unwrap();
    }

    pub(crate) fn bisect(&self, test_cmd: &String, args: &[String]) -> Result<&str, &str> {
        println!("Bisecting all commits");

        self.bisect_reset();
        self.bisect_start();
        self.bisect_bad();
    
        let first_commit = &self.get_first_commit();
        self.checkout(first_commit);        

        let mut iterations = 0;

        loop {
            // println!("Current : {:?}", self.current_hash());
            if iterations >= 1000 {
                return Err("Bug not found after max iterations reached.");
            } else {
                iterations = iterations + 1
            }

            let status = Command::new(&test_cmd)
                .args(args)
                .stdout(Stdio::null())
                .status()
                .expect("oups");

            if status.success() {
                let guilty = self.bisect_good();
                if guilty {
                    println!("Guilty found:");
                    self.current_hash();
                    println!("Found the bug in {iterations} steps.");
                    break;
                }
            } else {
                let guilty = self.bisect_bad();
                if guilty {
                    println!("Guilty found:");
                    self.current_hash();
                    println!("Found the bug in {iterations} steps.");
                    break;
                }
            }
        }

        self.bisect_reset();

        return Ok("Bug found");
    }

    pub(crate) fn new() -> Git {
        let git_log_output = Git::log();

        let commits: Vec<_> = git_log_output
            .split("\n")
            .map(|x| {
                let mut split = x.split(",");
                Commit {
                    hash: split.next().unwrap().to_string(),
                    date: split.next().unwrap().to_string(),
                    message: split.next().unwrap().to_string(),
                }
            })
            .collect();

        return Git { commits };
    }
}
