# Rust_Server :rocket: and Deployment on Heroku

[Rust][] is a low level programming language for high performance and security.
[Rocket][] :rocket: is web framework for web development in Rust Language.
[Heroku][] is used to deploy Rust web applications.

[Rust]: https://www.rust-lang.org/
[Rocket]: https://rocket.rs/
[Heroku]: https://www.heroku.com/


Link : https://rust-rocket01.herokuapp.com/

## - Rust Server

```
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::request::Form;
use rocket::response::content::Html;
```

First, we need to include rocket framework. [rocket:: rocket::Form](https://api.rocket.rs/v0.4/rocket/request/struct.Form.html) is used to get data of form on **POST** request. [rocket:: rocket:content::Html](https://api.rocket.rs/v0.4/rocket/response/content/) is used for returning Html so that our page shows HTML rather than string.

This is get request from which client is make a post request.

```
#[get("/num")]
fn getRequest()-> Html<String>{
    let h1=format!("
        <form action='/num' method='post'>
            <input type='number' placeholder='Enter number'
                name='number' id='number'>
            <input type='Submit'>
        </form>
    ");
    Html(h1)
}
```

It shows a form which has input field of type Number and make a post request to the ("/num"), which is the post request url, in **action** atrbute.

```
#[derive(FromForm)]
struct Number{
    number : i32,
}
```
The above code is use to tell rust what fields are contain in our post request data and of which DataType. 

```
#[post("/num", data = "<number>")]
fn recieveRequest(number : Form<Number>) -> Html<String> {
   let h1=format!("
   <p>{}</p>
    ", number.number);
    Html(h1)
}
```

This code is used when post request (submit button is clicked)  is made, the data sent by client is recieved in data='\<number>' and then rust use this parameter as argument variable. This variable *number* has datatype of Form\<Number>, 'Number' which is struct.Then we write the html in format!() macro, which is then return as Html\<String>.


> ### For More Help, go to [Rocket Framework](https://rocket.rs/)

This site is helps a lot in writing your server.


## - Deployment

## For Linux 

### 1. Heroku Installation


1. First of all, Make a account on [Heroku](https://signup.heroku.com/login) 


For deployment of Application, I will recommend Heroku CLI

2. For installation of [Heroku CLI](https://devcenter.heroku.com/articles/getting-started-with-ruby#set-up), Run this command in your terminal

```
sudo snap install --classic heroku
```
3. Verify your installation using this command.

```
heroku --version 
# heroku/7.0.0 (darwin-x64) node-v8.0.0
```

4. Login to Heroku Account
```
heroku login -i
```

### 2. Deployment

1. In the root directory of your written application, open terminal and enter this command.

#### What is root Directory?

```
myrepo
    |_src
    |   |_main.rs
    |
    |_target
    |_Cargo.toml
    |_Cargo.lock
```

In this case, myrepo is your root Directory. 

Example:
```
myname@mycomp:~/Documents/myrepo$
```

Now write this command,

```
heroku create YourRepoName \
  --buildpack https://github.com/emk/heroku-buildpack-rust.git
```

2. Now, make these four files namely : Procfile, rust-toolchain, RustConfig, .travis.yml

**Procfile**

Write this content in your Procfile

```
web: ROCKET_PORT=$PORT target/release/YourLocalRepoName

like according to example: 
web: ROCKET_PORT=$PORT target/release/myrepo
```

**rust-toolchain**

Write this content in your rust-toolchain file

```
nightly-2020-05-14
```

As the rocket framework works with rust-nightly version, so we have to tell Heroku that our application works with rust-nightly.

**RustConfig**

Write this content in your RustConfig file

```
VERSION=nightly
```

**.travis.yml**

Write this content in your .travis.yml file

```
language: rust
sudo: required
rust:
- nightly
script:
- |
  cargo build &&
  cargo test
```

3. Your repo structure is something like this.

```
myrepo
    |_src
    |   |_main.rs
    |
    |_target
    |_Cargo.toml
    |_Cargo.lock
    |_.gitignore
    |_.travis.yml
    |_Procfile
    |_README.md
    |_Rocket.toml
    |_rust-toolchain
    |_RustConfig
```

4. Now, the last step to deploy your application, run:

```
heroku config:set ROCKET_ENV=production
git add .
git commit -m 'commitMsg'
git push heroku master
```

**Your application has been deployed :smiley::smiley::smiley:, now you can see your application using url given by heroku :clap::clap::clap:**

You have done!! :rocket::rocket:
