use std::io::{Read, Write};

pub const FILE: &str = "log.txt";
const PASSWORD_SIZE: u8 = 20;

pub struct Email {
  pub user: String,
  pub domain: String,
  pub password: String,
}

impl Email {
  pub fn new() -> Self {
    Self {
      user: String::new(),
      domain: String::new(),
      password: String::new(),
    }
  }

  pub fn create_email(&mut self) -> Result<(), ()>{
    let possible_domains: [&str; 8] = [
      "1secmail.com",
      "1secmail.org",
      "1secmail.net",
      "kzccv.com",
      "qiott.com",
      "wuuvo.com",
      "icznn.com",
      "ezztt.com"
    ];

    let mut input = String::new();

    println!("domains - {:?}", possible_domains);
    while input.len() < 5 {
      print!("please write your customized email: ");
      std::io::stdout().flush().unwrap();
      input.clear();
      std::io::stdin().read_line(&mut input).expect("cannot read");
    }

    let mut is_domain = false;
    for domain in possible_domains {
      if input.contains(&format!("@{}", domain)) {
        is_domain = true;
      }
    }

    if is_domain {
      let input: Vec<&str> = input.trim().split('@').collect();

      self.user = input[0].to_string();
      self.domain = input[1].to_string();
    } else {
      eprintln!("invalid domain");
      return Err(());
    }

    Ok(())
  }

  pub fn get_email_set(&mut self) {
    print!("write your email, to view recived emails: ");
    std::io::stdout().flush().unwrap();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("cannot read");

    let input = input.trim().split('@').collect::<Vec<&str>>();

    self.user = input[0].to_string();
    self.domain = input[1].to_string();
  }
}

#[derive(PartialEq)]
pub enum Interface {
  CREATE_EMAIL,
  CREATE_RANDOM_EMAIL,
  GET_EMAIL,
}

pub async fn get_response_users(url: &str) -> Vec<String> {
  let body = reqwest::get(url)
      .await.unwrap()
      .text()
      .await.unwrap();

  let mut emails: Vec<String> = Vec::new();
  let mut email = String::new();
  for letter in body.chars() {
    if letter != '"' && letter != '[' && letter != ']' && letter != ',' {
      email.push(letter);
    } else {
      emails.push(email.clone());
      email.clear();
    }
  }

  emails
}

pub async fn get_response_recived(url: &str) -> String {
   reqwest::get(url)
      .await.unwrap()
      .text()
      .await.unwrap()
}

pub fn create_file(emails: &Vec<String>) -> std::io::Result<std::fs::File>{
  let random_password: fn() -> String = || {
    let mut letters: Vec<char> = Vec::new();
    for az in b'a'..=b'z' {
      letters.push(az as char);
    }

    let mut result: String = String::new();
    let mut count = 0;
    while count <= PASSWORD_SIZE {
      result.push(letters[random_number::random!(0..=25)]);
      count += 1;
    }

    result
  };

  match std::fs::File::create(FILE) {
    Ok(file) => {
      let mut d_email = String::new();
      for email in emails {
        if email.ne("") {
          d_email.push_str(&format!("email:{} password:{}\n", email, random_password()));
        }
      }
      std::fs::write(
        FILE,
        d_email
      ).expect("cannot write FILE");
      Ok(file)
    },
    Err(e) => panic!("an error occured!, {}", e),
  }
}

pub fn read_file() -> String {
  match std::fs::read_to_string(FILE) {
    Ok(content) => content,
    Err(e) => panic!("error occured {}", e),
  }
}

pub fn write_file(content: &str) {
  match std::fs::write(
    FILE,
    content,
  ) {
    Ok(_) => (),
    Err(e) => panic!("an error occured! {}", e),
  }
}