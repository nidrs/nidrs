//！ cargo set tools
//！ cargo set rust-version 1.62
use toml_edit::{value, DocumentMut};

use std::fs;

use clap::Parser;

#[derive(Debug, clap::Parser)]
struct Cli {
    #[command(flatten)]
    manifest: clap_cargo::Manifest,
    #[command(flatten)]
    workspace: clap_cargo::Workspace,
    #[command(flatten)]
    features: clap_cargo::Features,

    /// Dot path to the manifest key, eg: package.name
    dot_path: String,

    /// Value to set int、float、bool、string
    value: String,
}

fn main() {
    let cwd = std::env::current_dir().unwrap();
    let opts: Cli = Cli::parse();

    let cargo_toml = cwd.join("Cargo.toml");

    let cargo_toml_path = cargo_toml.clone();

    let cargo_toml = fs::read_to_string(cargo_toml).expect("Error reading Cargo.toml");

    let mut doc = cargo_toml.parse::<DocumentMut>().expect("invalid doc");

    let mut key = opts.dot_path.split('.').collect::<Vec<_>>();
    // let mut key = vec!["workspace", "members", "0"]; //opts.dot_path.split('.').collect::<Vec<_>>();

    let table = doc.as_table_mut();

    // println!("{:?}", table);

    let mut table_ref = table.get_mut(key.remove(0));

    // read workspace.members.1

    for k in key.iter() {
        if let Some(t) = table_ref {
            let kt = k.parse::<usize>();
            if let Ok(kt) = kt {
                table_ref = t.get_mut(kt)
            } else {
                table_ref = t.get_mut(k)
            }
        } else {
            panic!("not found");
        }
    }

    let set_value = opts.value;

    if let Some(t) = table_ref {
        // update value
        // int\float\bool\string

        if let Ok(v) = set_value.parse::<f64>() {
            *t = value(v);
        } else if let Ok(v) = set_value.parse::<f64>() {
            *t = value(v);
        } else if let Ok(v) = set_value.parse::<bool>() {
            *t = value(v);
        } else {
            *t = value(set_value.trim_matches('"'));
        }
    } else {
        panic!("not found");
    }

    // println!("{:?}", table_ref);

    fs::write(cargo_toml_path.clone(), doc.to_string()).expect("Error writing to file");

    println!("set {:?} value {:?} success. ", cargo_toml_path.to_str().unwrap(), opts.dot_path);
}
