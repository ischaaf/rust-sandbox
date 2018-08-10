extern crate clap;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use std::collections::HashMap;
use std::fs::File;
use clap::{Arg, App, SubCommand, ArgMatches};


#[derive(Serialize, Deserialize)]
struct JobRunnerEnv {
    host: String,
    user: String,
    root: String,
}

#[derive(Serialize, Deserialize)]
struct JobRunnerStatic {
    environments: HashMap<String, JobRunnerEnv>,
}


fn read_static() -> JobRunnerStatic {
    let err_msg = "could_not_read_static_data";
    let file = File::open("static/jobrunner.json").expect(err_msg);
    let val: JobRunnerStatic = serde_json::from_reader(file).expect(err_msg);
    val
}

fn cmd_list_envs(_: &ArgMatches, static_data: JobRunnerStatic) {
    println!("Available environments are:");
    for (env, data) in &static_data.environments {
        println!("{} {}@{}:{}", env, data.user, data.host, data.root);
    }
}

fn main() {
    let matches = App::new("Job Runner CLI")
        .version("0.0.1")
        .author("Isaac Schaaf")
        .about("CLI tool for interacting with the JobRunner")
        .arg(Arg::with_name("env")
             .short("e")
             .long("environment")
             .value_name("ENV")
             .help("Sets the JobRunner environment")
             .takes_value(true))
        .subcommand(SubCommand::with_name("envs")
                    .about("List the available environments")
                    .version("0.0.1")
                    .author("Isaac Schaaf"))
        .get_matches();
    let static_data = read_static();
    let env = matches.value_of("env").unwrap_or("local");
    println!("Environment set to: {}", env);
    if let Some(matches) = matches.subcommand_matches("envs") {
        cmd_list_envs(matches, static_data);
        return ();
    }
}
