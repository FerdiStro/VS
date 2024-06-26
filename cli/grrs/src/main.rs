use std::thread::sleep;
use std::{thread, time};

use clap::Parser;
use colored::Colorize;
use dialoguer::{Input, Select};
use dialoguer::theme::ColorfulTheme;
use indicatif::ProgressBar;
use inquire::MultiSelect;


#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() {

    println!(
        "{} \n  {} \n  {}\n  \n",
        "Welcome to the CV generator".bold(),
        "Dependency Needed:",
        "- docker-compose",
    );

    /*
        Language Choicer
    */
    let language_choice = vec!["Deutsch", "English"];


    let language_select =  Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a Language")
        .items(&language_choice)
        .default(0)
        .interact()
        .unwrap();


    let mut skill_choice : Vec<&str>    = vec![];
    let mut skill_select_text : &str    = "";

    let mut skill_list_text: &str       = "";
    let mut skill_point_text: &str      = "";



    let true_list  : Vec<&str>  = vec!["yes", "Ja"];
    let false_list : Vec<&str>  =  vec!["no", "Nein"];


    if(language_choice[language_select].eq("Deutsch")){
        skill_choice            = vec!["Ja", "Nein"];
        skill_select_text       = "Willst du deine Programmierskills im Lebenslauf ";
        skill_list_text         = "Waehle deine Skills:";
        skill_point_text        = "Weise Punkte von 0 bis 5 zu für:";

    }
    if(language_choice[language_select].eq("English")){
        skill_choice            = vec!["yes", "no"];
        skill_select_text       = "Do you want your Programming Skills in your CV?";
        skill_list_text         = "Choice your skills:";
        skill_point_text        = "Choice point from 0 to 5:";
    }





    println!("\n\n");






    /*
        Skills
    */

    let skill_select =  Select::with_theme(&ColorfulTheme::default())
        .with_prompt(skill_select_text)
        .items(&skill_choice)
        .default(0)
        .interact()
        .unwrap();
    println!("\n\n");


    if(true_false_convert(skill_choice[skill_select], true_list, false_list)){
        let  mut skills : Vec<&str> = vec![
            "Java",
            "C",
            "C++",
            "c#",
        ];

        let mut rating = vec![0; skills.len()];


        let selection = MultiSelect::new(skill_list_text, skills.clone())
            .with_page_size(skills.len())
            .prompt();

        let mut selected_skills : Vec<&str> = vec![];

        match selection {
            Ok(selections) => {
                if selections.is_empty() {
                    println!("No skills selected ");
                } else {
                    for selection in selections {
                        selected_skills.push(selection);
                    }
                }
            }
            Err(err) => println!("Error: {}", err),
        }
        println!("\n\n");


        for (i, selected_skills) in selected_skills.iter().enumerate() {
            println!("{} {}", &skill_point_text , selected_skills);

            let point: i32 = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("")
                .validate_with(|input: &i32| -> Result<(), &str> {
                    if *input >= 0 && *input <= 5 {
                        Ok(())
                    } else {
                        Err("Between 0 and 5")
                    }
                })
                .interact()
                .unwrap();

            rating[i] = point;
        }
    }





    // println!("Punkteverteilung:");
    // for (option, point) in skills.iter().zip(skills.iter()) {
    //     println!("{}: {} Punkte", option, point);
    // }







    // let args = Cli::parse();
    // let content = std::fs::read_to_string(&args.path).expect("could not read file");
    //
    // for line in content.lines() {
    //     if line.contains(&args.pattern) {
    //         println!("{}", line);
    //     }
    // }


    // ProgressBar
    // let pb = ProgressBar::new(100);
    //
    // let ten_millis = time::Duration::from_millis(10);
    //
    // for i in 0..100 {
    //     sleep(ten_millis);
    //     pb.inc(1);
    // }
    // pb.finish_with_message("done");
}

fn true_false_convert( message: &str , true_list : Vec<&str> , false_list : Vec<&str>, ) -> bool {
    if(true_list.contains(&message)){
        return true
    }
    if(false_list.contains(&message)){
        return false
    }
    return false;
}