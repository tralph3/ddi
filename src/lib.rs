//A safer dd
//Copyright (C) 2021  Tomás Ralph
//
//This program is free software: you can redistribute it and/or modify
//it under the terms of the GNU General Public License as published by
//the Free Software Foundation, either version 3 of the License, or
//(at your option) any later version.
//
//This program is distributed in the hope that it will be useful,
//but WITHOUT ANY WARRANTY; without even the implied warranty of
//MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//GNU General Public License for more details.
//
//You should have received a copy of the GNU General Public License
//along with this program.  If not, see <https://www.gnu.org/licenses/>.
//
////////////////////////////////////
//                                //
//       Created by tralph3       //
//   https://github.com/tralph3   //
//                                //
////////////////////////////////////

use std::{collections::HashMap, io::Write};
use std::{io, process};
use termion::{color, style};
mod device;

fn parse_arguments(args: &Vec<String>) -> HashMap<String, String> {
    /*
       Separates each key=value pair into a HashMap.
       If --version or --help are present, it deletes
       all other arguments. Also deleted unrecognized ones.
    */


    let mut map: HashMap<String, String> = HashMap::new();

    for arg in args {

        // skip invalid arguments
        if arg != "--version" && arg != "--help" && !arg.contains("=") {
            continue;
        }

        if !arg.contains("=") {
            // if --version or --help are passed, ignore all other arguments
            map.clear();
            map.insert(arg.to_string(), arg.to_string());
            break;
        }

        let key_val: Vec<&str> = arg.split('=').collect();

        let key: String = key_val[0].to_string();
        let val: String = key_val[1].to_string();

        map.insert(key, val);
    }

    map
}

fn generate_warning_text(device_info: device::DeviceInfo) -> String {

    let part_text: String;

    if device_info.partitions.len() == 0 {
        part_text = String::from("There are no present partitions");
    } else {
        part_text = String::from("The following partitions are present:");
    }

    let mut warning_text = format!(
            "{red}{bold}WARNING: {yellow}You are about to write data to {}\
, this device has the following information.{white}{reset}

    {italic}{cyan}{bold}* Name:{reset} {}
    {italic}{cyan}{bold}* Model:{reset} {}
    {italic}{cyan}{bold}* Size:{reset} {}

{yellow}{bold}{part_text}

",          device_info.name, device_info.name, device_info.model,
            device_info.size, red=color::Fg(color::Red), bold=style::Bold,
            white=color::Fg(color::White), reset=style::Reset,
            italic=style::Italic, cyan=color::Fg(color::Cyan),
            yellow=color::Fg(color::Yellow), part_text=part_text
        );

    for partition in device_info.partitions {
        warning_text = format!("{}\

    {italic}{cyan}{bold}* Name:{reset} {}
    {italic}{cyan}* File System:{reset} {}
    {italic}{cyan}* Mount Point:{reset} {}
    {italic}{cyan}* Label:{reset} {}
    {italic}{cyan}* Size:{reset} {}

", warning_text, partition.name, partition.fs,
            partition.mount, partition.label, partition.size,
            reset=style::Reset, italic=style::Italic,
            cyan=color::Fg(color::Cyan), bold=style::Bold)
    }

    warning_text
}

fn warn_user(device: &str) -> Result<String, Box<dyn std::error::Error>> {

    let device_info = device::DeviceInfo::new(device);

    // if the problem is just not using a block device, ommit it
    if let Err(e) = device_info {
        if e.code() == Some(32) {
            return Ok(String::from("y"));
        }
    }

    let warning_text = generate_warning_text(device_info.unwrap());

    println!("{}", warning_text);
    print!("{red}{bold}THIS WILL DESTROY ALL DATA ON THE DEVICE
THIS ACTION CANNOT BE UNDONE
{yellow}Are you absolutely sure you want to proceed? \
{reset}{white}[{red}y{white}/{green}N{white}]{reset}: ",
        yellow=color::Fg(color::Yellow), red=color::Fg(color::Red),
        white=color::Fg(color::White), green=color::Fg(color::Green),
        bold=style::Bold, reset=style::Reset);
    io::stdout().flush()?;

    let mut answer: String = String::new();
    io::stdin().read_line(&mut answer)?;

    Ok(answer)

}

fn process_answer(answer: &str) -> bool {
    answer.trim().to_lowercase() == "y"
}

fn unparse_args(args: &HashMap<String, String>) -> String {
    if args.contains_key("--version") {
        return String::from("--version");
    } else if args.contains_key("--help") {
        return String::from("--help");
    }

    let mut result = String::new();

    for (key, value) in args.iter() {
        result = format!("{}{}={} ", result, key, value);
    }

    result.trim_end().to_string()
}

fn execute_dd(args: &str) -> Result<(), Box<dyn std::error::Error>>{

    let args: Vec<&str> = args.split(" ").collect();

    println!("{:?}", args);

    process::Command::new("dd")
        .args(args)
        .stdout(process::Stdio::inherit())
        .stderr(process::Stdio::inherit())
        .stdin(process::Stdio::inherit())
        .output()?;

    Ok(())
}

pub fn run(args: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {

    let parsed_args = parse_arguments(&args);
    let unparsed_args = unparse_args(&parsed_args);

    if parsed_args.contains_key("of") {

        let result = warn_user(parsed_args["of"].as_str());

        match result {
            Err(e) => { return Err(e); },

            Ok(answer) => {
                if process_answer(&answer) {
                    execute_dd(&unparsed_args)?;
                } else {
                    println!("{}{}Execution aborted{}",
                        color::Fg(color::Green), style::Bold, style::Reset);
                }
            }
        }

    } else {
        execute_dd(&unparsed_args)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn discard_useless() {
        let args = vec!["if=test".to_string(), "asd".to_string()];
        let parsed = parse_arguments(&args);
        assert!(!parsed.contains_key("asd"));
    }

    #[test]
    fn discard_if_version() {
        let args = vec!["if=test".to_string(), "--version".to_string()];
        let parsed = parse_arguments(&args);
        assert!(!parsed.contains_key("if=test"));
    }

    #[test]
    fn discard_if_help() {
        let args = vec!["if=test".to_string(), "--help".to_string()];
        let parsed = parse_arguments(&args);
        assert!(!parsed.contains_key("if=test"));
    }
}
