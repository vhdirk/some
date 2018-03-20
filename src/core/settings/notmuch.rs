use std;
use std::fs::File;
use std::io::prelude::*;

use std::path::Path;
use std::collections::BTreeMap;
use serde;
use serde::de::{self, Deserialize, Deserializer, Visitor, MapAccess};

use shellexpand;

use serde_ini;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub database: DatabaseConfig,
    pub user: UserConfig,
    pub new: NewConfig,
    pub search: SearchConfig,
    pub maildir: MailDirConfig
}


impl Config{
    #[serde(skip_serializing)]
    pub fn load(location: &Path) -> Self {
        let mut conf_contents = String::new();
        let mut expanded = shellexpand::full(location.to_str().unwrap()).unwrap().into_owned();

        let expanded_path = Path::new(&expanded);

        match File::open(&expanded_path) {
            Ok(mut file) => {
                file.read_to_string(&mut conf_contents);
            },
            Err(err) => {
                println!("err {:?}", err);
                conf_contents = "".to_string();
            },
        };

        let mut conf: Config = serde_ini::from_str(&conf_contents).unwrap();

        return conf;
    }

    // #[serde(skip_serializing)]
    // pub fn store(self: &Self, location: &Path) -> Result<(), String> {
    //     // let mut outfile = File::create(location).unwrap();
    //     // outfile.write_all(toml::to_string(&self).unwrap().as_bytes());
    //     // outfile.sync_all();
    //
    //     return Ok(());
    //
    // }
}


#[derive(Serialize, Deserialize, Debug)]
pub struct DatabaseConfig {
    #[serde(default = "default_database_path")]
    pub path: String
}

fn default_database_path() -> String {
    //Default: $MAILDIR variable if set, otherwise $HOME/mail.
    return "~/mail".to_string();
}


#[derive(Serialize, Deserialize, Debug)]
pub struct UserConfig {
    #[serde(default = "default_user_name")]
    pub name: String,
    #[serde(default = "default_user_primary_email")]
    pub primary_email: String,
    pub other_email: String
}


fn default_user_name() -> String {
    //Default: $NAME variable if set, otherwise read from /etc/passwd.

    let env_var = std::env::var("NAME");
    if env_var.is_ok(){
        return env_var.unwrap();
    };

    return "".to_string();
}

fn default_user_primary_email() -> String {
    //Default: $EMAIL variable if set, otherwise constructed from  the
    //username and hostname of the current machine.
    let env_var = std::env::var("EMAIL");
    if env_var.is_ok(){
        return env_var.unwrap();
    };

    return "".to_string();
}


#[derive(Serialize, Deserialize, Debug)]
pub struct NewConfig {
    // A  list  of tags that will be added to all messages incorporated by notmuch new.
    #[serde(deserialize_with="parse_csv")]
    pub tags: Vec<String>,

    pub ignore: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchConfig {
    #[serde(deserialize_with="parse_csv")]
    pub exclude_tags: Vec<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MailDirConfig {
    #[serde(default = "default_maildir_synchronize_flags", deserialize_with="parse_bool")]
    pub synchronize_flags: bool
}

fn default_maildir_synchronize_flags() -> bool {
    return true;
}


fn parse_csv<'de, D>(d: D) -> Result<Vec<String>, D::Error> where D: Deserializer<'de> {
    Deserialize::deserialize(d)
        .map(|x: Option<String>| {
            return x.unwrap().split(";").map(|s| s.to_string()).collect();
        })
}

fn parse_bool<'de, D>(d: D) -> Result<bool, D::Error> where D: Deserializer<'de> {
    Deserialize::deserialize(d)
        .map(|x: Option<String>| {
            return x.unwrap().parse().unwrap();
        })
}