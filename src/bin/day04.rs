use anyhow::{Context, Result};
use aoc_2020::input_lines;
use lazy_static::lazy_static;
use regex::Regex;

fn main() -> Result<()> {
    let input = input_lines(4)?;

    let mut passports = Vec::new();
    let mut current = Passport::default();

    for line in input {
        let line = line.context("Failed to read line")?;

        if line.is_empty() {
            passports.push(current.take());
        } else {
            current.merge_data(&line);
        }
    }

    passports.push(current.take());

    part1(&passports);
    part2(&passports);

    Ok(())
}

fn part1(passports: &[Passport]) {
    let count = passports.iter().filter(|pp| pp.is_valid()).count();
    println!("There are {} valid passports.", count);
}

fn part2(passports: &[Passport]) {
    let count = passports
        .iter()
        .filter(|pp| pp.is_valid_data_checked())
        .count();
    println!("There are {} valid passports.", count);
}

#[derive(Default, Debug)]
struct Passport {
    birth_year: Option<String>,
    issue_year: Option<String>,
    exp_year: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<()>,
}

impl Passport {
    pub fn merge_data(&mut self, line: &str) {
        let parts = line.split(" ").map(|part| {
            let mut parts = part.split(":");
            (parts.nth(0).unwrap(), parts.nth(0).unwrap())
        });

        for (part, value) in parts {
            match part {
                "byr" => self.birth_year = Some(value.to_owned()),
                "iyr" => self.issue_year = Some(value.to_owned()),
                "eyr" => self.exp_year = Some(value.to_owned()),
                "hgt" => self.hgt = Some(value.to_owned()),
                "hcl" => self.hcl = Some(value.to_owned()),
                "ecl" => self.ecl = Some(value.to_owned()),
                "pid" => self.pid = Some(value.to_owned()),
                "cid" => self.cid = Some(()),
                _ => eprintln!("Unknown passport data key: '{}'", line),
            }
        }
    }

    pub fn take(&mut self) -> Passport {
        let value = Passport {
            birth_year: self.birth_year.clone(),
            issue_year: self.issue_year.clone(),
            exp_year: self.exp_year.clone(),
            hgt: self.hgt.clone(),
            hcl: self.hcl.clone(),
            ecl: self.ecl.clone(),
            pid: self.pid.clone(),
            cid: self.cid,
        };

        *self = Passport::default();

        value
    }

    pub fn is_valid(&self) -> bool {
        let mandatory_fields = [
            &self.birth_year,
            &self.issue_year,
            &self.exp_year,
            &self.hgt,
            &self.hcl,
            &self.ecl,
            &self.pid,
        ];

        if mandatory_fields.iter().any(|opt| opt.is_none()) {
            false
        } else {
            true
        }
    }

    pub fn is_valid_data_checked(&self) -> bool {
        if !self.is_valid() {
            return false;
        }

        let byr = self.birth_year.as_ref().unwrap().parse::<usize>();
        let is_okay = match byr {
            Ok(val) if val >= 1920 && val <= 2002 => true,
            _ => false,
        };

        if !is_okay {
            return false;
        }

        let iyr = self.issue_year.as_ref().unwrap().parse::<usize>();
        let is_okay = match iyr {
            Ok(val) if val >= 2010 && val <= 2020 => true,
            _ => false,
        };

        if !is_okay {
            return false;
        }

        let eyr = self.exp_year.as_ref().unwrap().parse::<usize>();
        let is_okay = match eyr {
            Ok(val) if val >= 2020 && val <= 2030 => true,
            _ => false,
        };

        if !is_okay {
            return false;
        }

        let hgt = self.hgt.as_ref().unwrap();
        lazy_static! {
            static ref HGT_REGEX: Regex = Regex::new(r"([0-9]*)(in|cm)").unwrap();
        }

        if let Some(caps) = HGT_REGEX.captures(hgt) {
            let value = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
            let units = caps.get(2).unwrap().as_str();

            let is_valid = match units {
                "cm" => {
                    if value >= 150 && value <= 193 {
                        true
                    } else {
                        false
                    }
                }
                "in" => {
                    if value >= 59 && value <= 76 {
                        true
                    } else {
                        false
                    }
                }
                _ => panic!("Unknown units"),
            };

            if !is_valid {
                return false;
            }
        } else {
            return false;
        }

        let hcl = self.hcl.as_ref().unwrap();
        lazy_static! {
            static ref HCL_REGEX: Regex = Regex::new(r"^#(([0-9]|[a-f]){6})$").unwrap();
        }

        if !HCL_REGEX.is_match(hcl) {
            return false;
        }

        let ecl = self.ecl.as_ref().unwrap();
        let is_okay = match ecl.as_str() {
            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
            _ => false,
        };

        if !is_okay {
            return false;
        }

        let pid = self.pid.as_ref().unwrap();
        lazy_static! {
            static ref PID_REGEX: Regex = Regex::new(r"(^[0-9]{9})$").unwrap();
        }

        PID_REGEX.is_match(pid)
    }
}
