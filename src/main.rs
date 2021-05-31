#![allow(unused_imports)]
extern crate clap;
extern crate tbot;
extern crate egg_mode;
extern crate yansi;
extern crate tokio_stream;
extern crate futures;
extern crate regex;
extern crate serde_regex;

mod config;
mod twitter;
mod rule;
mod test;

use clap::{App, Arg};
use clap::value_t;
use regex::Regex;
use tbot::prelude::*;
use tbot::Bot;
use tbot::types::parameters::Text;
use tbot::types::chat::Id;
//use tbot::types::chat::Chat;
use tbot::types::chat;
use std::io;
use crate::rule::TweetInfo;

use chrono::{Local, Timelike, Datelike};
use egg_mode::{stream::StreamMessage, user::TwitterUser};
use egg_mode::error::Result;
use egg_mode::cursor::CursorIter;

use tokio_stream::StreamExt;
use futures::TryStreamExt;
use futures::executor::block_on;


use crate::twitter::Auth;

//parse cfg/rules
//log onto twitter
//create handler for twitter events
//create handler for telegram events
//the end!

#[tokio::main]
async fn main() {
    let matches = App::new("Twat ")
        .version("0.1")
        .author("Barry Corrigan <b.j.corrigan@gmail.com>")
        .about("Twitter and Telegram bot for notification use")
        .arg(Arg::with_name("config")
            .short("c")
            .long("config")
            .value_name("FILE")
            .help("The config and rules file which governs which accounts twat uses and what rules it has for following and alerting")
            .required(true)
            .takes_value(true))
        .get_matches();

    let cfg_file = Box::leak(value_t!(matches, "config", String).unwrap_or_else(|_| "./config.ron".to_string()).into_boxed_str() );

    let config = config::parse(cfg_file).unwrap_or_else(|error| {
        eprintln!("Couldn't parse the config: {:#?}", error);
        std::process::exit(1);
    });

    //twitter log in
    let twauth = Auth::load(&config).await;

    let t:Vec<u64> = egg_mode::user::friends_ids(twauth.user_id, &twauth.token)
            .take(10)
            .map_ok(|r| r.response)
            .try_collect::<Vec<_>>()
            .await.unwrap();

    let tbot = Bot::new(config.telegram.bot_token.to_string());
    let rules = &config.rules;

    let _ = egg_mode::stream::filter()
        .follow(&t)
        .language(&["en"])
        .start(&twauth.token)
        .try_for_each(|m| {
            if let StreamMessage::Tweet(tweet) = m {
                twitter::print_tweet(&tweet);
                for rule in rules {
                    //TODO Handle attached media - pictures etc
                    //we construct this because mocking it is a complete pain
                    let tweetinfo = TweetInfo {
                        text: twitter::get_text(&tweet),
                        hour: Local::now().hour(),
                        day: Local::now().date().weekday().to_string(),
                        retweeted: tweet.retweeted.unwrap_or(false),
                        user: tweet.user.as_ref().unwrap().id,
                        rtuser : twitter::get_root_user(&tweet),
                        screen_name: &tweet.user.as_ref().unwrap().screen_name,
                        followed_users: &t,
                    };

                    if rule.matches(&tweetinfo) { 
                        for chat in &rule.chats {
                            //TODO I suppose should try not blocking here...
                            let _ = block_on(tbot.send_message(Id(chat.chat), Text::with_html(format!("<b>{}</b>: {}" , tweet.user.as_ref().unwrap().screen_name, twitter::get_text(&tweet))))
                                                            .call()).map_err(|e| format!("There was a telegram error: {}", e));
                        }
                    }
                }
                
                println!("──────────────────────────────────────");
                //TODO check rules etc here and print to telegram
            } else {
                println!("{:?}", m);
            }
            futures::future::ok(())
        }).await.map_err(|e| format!("There was a tweeter error: {}", e));
}

#[test]
fn regexx() {
    let rgx = Regex::new("a76[\\D$]|irvine|kilmarnock|a77[\\D$]|m77[\\D$]|bellfield|galston").unwrap();
    let teststr = "A77 B730 Symington - A78 Monkton - Closure, All lanes closed Northbound https://t.co/v42ucR1Q32 #TSIncident";

    println!("Match? {}", rgx.is_match(&teststr));
}