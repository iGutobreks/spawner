use tokio;

mod proxy;

fn file_exists() -> bool {
  match std::fs::File::open(proxy::FILE) {
    Ok(_) => false,
    Err(_) => true,
  }
}

fn menu() -> proxy::Interface {
  print!("what you want do?\nCreate Email or Get Email\n");
  let mut input = String::new();
  std::io::stdin().read_line(&mut input).expect("cannot read");

  if input.trim().starts_with('C') || input.trim().starts_with('c') {
    return proxy::Interface::CREATE_EMAIL;
  }

  proxy::Interface::GET_EMAIL
}

#[tokio::main]
async fn main() {
  const COUNT: u8 = 10;

  let mut user = proxy::Email::new();
  let url_random:String = format!("https://www.1secmail.com/api/v1/?action=genRandomMailbox&count={}", COUNT);

  let mut want_make = proxy::Interface::CREATE_RANDOM_EMAIL;

  match file_exists() {
    true => {
      let rand_emails = proxy::get_response_users(&url_random).await;
      proxy::create_file(&rand_emails).unwrap();

      println!("{}", proxy::read_file());
      println!("created!");
    }
    false => {
      want_make = menu();
      if want_make == proxy::Interface::CREATE_EMAIL {
        match user.create_email() {
          Ok(_) => {
            let mut content = proxy::read_file();
            content.push_str(
              &format!("\nemail:{}@{} password:{}", user.user, user.domain, user.password)
            );
            proxy::write_file(&content);

            println!("email as been created");
          },
          Err(_) => println!("occurred an error. email fail to create"),
        };
      } else if want_make == proxy::Interface::GET_EMAIL{
        user.get_email_set(); // instance Email with all credentials passed by user
        let url_get:String = format!("https://www.1secmail.com/api/v1/?action=getMessages&login={}&domain={}", user.user, user.domain);
        let recived_content = proxy::get_response_recived(&url_get).await;

        if recived_content.len() < 5 {
          println!("you not recived anything");
        } else {
          println!("{}", recived_content);
        }
      }
    }
  }
}


