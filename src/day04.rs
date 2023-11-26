use std::{fs, str::FromStr};

#[derive(Debug)]
pub struct Passport {
    byr: Option<usize>,
    iyr: Option<usize>,
    eyr: Option<usize>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    _cid: Option<usize>,
}

impl Passport {
    pub fn is_valid2(&self) -> bool {
        let mut valid = 0;
        if let Some(val) = self.byr {
            if val >= 1920 && val <= 2002 {
                valid += 1;
            }
        }
        if let Some(val) = self.iyr {
            if val >= 2010 && val <= 2020 {
                valid += 1;
            }
        }
        if let Some(val) = self.eyr {
            if val >= 2020 && val <= 2030 {
                valid += 1;
            }
        }
        if let Some(val) = &self.hgt {
            if let Some((cm, _)) = val.split_once("cm") {
                let cm = cm.parse::<usize>().unwrap();
                if cm >= 150 && cm <= 193 {
                    valid += 1;
                }
            } else {
                if let Some((inch, _)) = val.split_once("in") {
                    let inch = inch.parse::<usize>().unwrap();
                    if inch >= 59 && inch <= 76 {
                        valid += 1;
                    }
                }
            }
        }
        if let Some(val) = &self.hcl {
            if val.starts_with("#") {
                if val.len() == 7 {
                    if val.chars().skip(1).all(|c| c.is_alphanumeric()) {
                        valid += 1;
                    }
                }
            }
        }
        if let Some(val) = &self.ecl {
            let should_be = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
            if val.len() == 3 && should_be.contains(&val.as_str()) {
                valid += 1;
            }
        }
        if let Some(val) = &self.pid {
            if val.len() == 9 && val.chars().all(|c| c.is_numeric()) {
                valid += 1;
            }
        }
        valid >= 7
    }

    pub fn is_valid1(&self) -> bool {
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
    }
}

impl FromStr for Passport {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let mut byr = None;
        let mut iyr = None;
        let mut eyr = None;
        let mut hgt = None;
        let mut hcl = None;
        let mut ecl = None;
        let mut pid = None;
        let mut cid = None;

        for field in s.split_whitespace() {
            let (identifier, value) = field.split_once(":").unwrap();
            match identifier {
                "ecl" => ecl = Some(value.to_string()),
                "pid" => pid = Some(value.to_string()),
                "eyr" => eyr = Some(value.parse()?),
                "hcl" => hcl = Some(value.to_string()),
                "byr" => byr = Some(value.parse()?),
                "iyr" => iyr = Some(value.parse()?),
                "cid" => cid = Some(value.parse()?),
                "hgt" => hgt = Some(value.to_string()),
                _ => panic!("Cant convert this :( should never happen"),
            }
        }

        Ok(Passport {
            byr,
            iyr,
            eyr,
            hgt,
            hcl,
            ecl,
            pid,
            _cid: cid,
        })
    }
}

fn parse_input(input: &str) -> Vec<Passport> {
    let mut s = String::new();
    let mut passports = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            let passport = Passport::from_str(&s).unwrap();
            passports.push(passport);
            s = String::new();
        } else {
            if let Some(c) = s.chars().last() {
                if c.is_whitespace() == false {
                    s.push_str(" ");
                }
            }
            s.push_str(line);
        }
    }

    if s.is_empty() == false {
        passports.push(Passport::from_str(&s).unwrap());
    }

    passports
}

fn get_input() -> String {
    fs::read_to_string("input/day04.txt").unwrap()
}

fn solve_1() -> usize {
    let input = get_input();
    let passports = parse_input(&input);
    passports.iter().fold(
        0,
        |acc, passport| if passport.is_valid1() { acc + 1 } else { acc },
    )
}

fn solve_2() -> usize {
    let input = get_input();
    let passports = parse_input(&input);
    passports.iter().fold(
        0,
        |acc, passport| if passport.is_valid2() { acc + 1 } else { acc },
    )
}

pub fn answer_1() {
    println!("day04 part1: {}", solve_1());
}

pub fn answer_2() {
    println!("day04 part2: {}", solve_2());
}

#[test]
fn test1() {
    let input = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
    byr:1937 iyr:2017 cid:147 hgt:183cm

    iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
    hcl:#cfa07d byr:1929

    hcl:#ae17e1 iyr:2013
    eyr:2024
    ecl:brn pid:760753108 byr:1931
    hgt:179cm

    hcl:#cfa07d eyr:2025 pid:166559648
    iyr:2011 ecl:brn hgt:59in";

    let passports = parse_input(input);
    let valid = passports.iter().fold(
        0,
        |acc, passport| if passport.is_valid1() { acc + 1 } else { acc },
    );

    assert_eq!(2, valid);
}

#[test]
fn test2() {
    let input = "eyr:1972 cid:100
    hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

    iyr:2019
    hcl:#602927 eyr:1967 hgt:170cm
    ecl:grn pid:012533040 byr:1946

    hcl:dab227 iyr:2012
    ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

    hgt:59cm ecl:zzz
    eyr:2038 hcl:74454a iyr:2023
    pid:3556412378 byr:2007";
    let passports = parse_input(input);
    let valid = passports.iter().fold(
        0,
        |acc, passport| if passport.is_valid2() { acc + 1 } else { acc },
    );

    assert_eq!(0, valid);
}

#[test]
fn test3() {
    let input = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
    hcl:#623a2f

    eyr:2029 ecl:blu cid:129 byr:1989
    iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

    hcl:#888785
    hgt:164cm byr:2001 iyr:2015 cid:88
    pid:545766238 ecl:hzl
    eyr:2022

    iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";
    let passports = parse_input(input);
    let valid = passports.iter().fold(
        0,
        |acc, passport| if passport.is_valid2() { acc + 1 } else { acc },
    );

    assert_eq!(4, valid);
}
