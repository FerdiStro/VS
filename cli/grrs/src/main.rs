mod generator;

use std::thread::sleep;
use std::{thread, time};
use std::io;

use generator::generator as gen;





use clap::Parser;
use colored::Colorize;
use dialoguer::{Input, Select};
use dialoguer::theme::ColorfulTheme;
use inquire::MultiSelect;
use crate::generator::generator::{CV, Skill};

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


    let mut skill_choice : Vec<&str>            = vec![];
    let mut skill_select_text : &str            = "";
    let mut skill_list_text: &str               = "";
    let mut skill_point_text: &str              = "";

    let mut name_first_ask_text : &str          = "";
    let mut name_second_ask_text : &str         = "";

    let mut contact_email_ask_text: &str        = "";
    let mut contact_phone_aks_text: &str        = "";


    let true_list  : Vec<&str>  = vec!["yes", "Ja"];
    let false_list : Vec<&str>  =  vec!["no", "Nein"];


    if language_choice[language_select].eq("Deutsch"){

        skill_choice            = vec!["Ja", "Nein"];
        skill_select_text       = "Willst du deine Programmierskills im Lebenslauf ";
        skill_list_text         = "Waehle deine Skills:";
        skill_point_text        = "Weise Punkte von 0 bis 5 zu für:";

        name_first_ask_text     = "Gib deinen Vornamen ein :";
        name_second_ask_text    = "Gib deinen Nachnamen ein:";

        contact_email_ask_text  = "Gib deine Email-Adresse ein:";
        contact_phone_aks_text  = "Gib deine Telefonnummer ein:";
    }
    if language_choice[language_select].eq("English"){

        skill_choice            = vec!["yes", "no"];
        skill_select_text       = "Do you want your Programming Skills in your CV?";
        skill_list_text         = "Choice your SKILLS:";
        skill_point_text        = "Choice point from 0 to 5:";

        name_first_ask_text     = "Enter your firstname: ";
        name_second_ask_text    = "Enter your lastname :";

        contact_email_ask_text  = "Enter your E-Mail      :";
        contact_phone_aks_text  = "Enter your Phone-nummer:";

    }
    println!("\n");


    /*
        Name
     */
    println!("{}",
             name_first_ask_text.bold()
    );

    let mut first_name = String::new();
    io::stdin()
        .read_line(&mut first_name)
        .expect("Failed to read line");

    println!("{}",
             name_second_ask_text.bold()
    );

    let mut second_name = String::new();
    io::stdin()
        .read_line(&mut second_name)
        .expect("Failed to read line");


    /*
        Contact information
     */

    println!("{}",
             contact_email_ask_text.bold()
    );


    let mut email_address = String::new();
    io::stdin()
        .read_line(&mut email_address)
        .expect("Failed to read line");

    println!("{}",
             contact_phone_aks_text.bold()
    );


    let mut phone_nummer = String::new();
    io::stdin()
        .read_line(&mut phone_nummer)
        .expect("Failed to read line");


    /*
        Skills
    */

    println!("\n");

    let skill_select =  Select::with_theme(&ColorfulTheme::default())
        .with_prompt(skill_select_text)
        .items(&skill_choice)
        .default(0)
        .interact()
        .unwrap();

    let mut selected_skills_ob:Vec<Skill> = vec![];


    println!("\n\n");


    if true_false_convert(skill_choice[skill_select], true_list, false_list){
        let skills: Vec<&str> = vec![
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

                        let skill  : Skill =  Skill{
                            skill_name: selection,
                            rating: "".to_string(),
                        };

                        selected_skills_ob.push(skill)
                    }
                }
            }
            Err(err) => println!("Error: {}", err),
        }







        for i in 0..selected_skills_ob.len() {
            println!("{} {}", &skill_point_text, selected_skills_ob[i].skill_name);

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

            if let Some(skill) = selected_skills_ob.get_mut(i) {
                skill.rating = format_rating(point); ;
            }
        }






    }




    let cv_data: CV = CV {
        first_name:         &*first_name.replace("\n", ""),
        last_name:          &*second_name.replace("\n", ""),
        phone_number :      &*phone_nummer.replace("\n", ""),
        email_address:      &*email_address.replace("\n", ""),
        skills:             selected_skills_ob,
    };

    gen::generate(cv_data);



    // println!("Punkteverteilung:");
    // for (option, point) in SKILLS.iter().zip(SKILLS.iter()) {
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


    // let number = if condition { 5 } else { 6 };

}
fn format_rating(rating: i32) -> String {
    let filled = "<div class=\"rounded-circle-fill\"> </div>".repeat(rating as usize);
    let empty = "<div class=\"rounded-circle-non-fill\"> </div>".repeat(5 - rating as usize);
    format!("{}{}", filled, empty)
}


fn true_false_convert( message: &str , true_list : Vec<&str> , false_list : Vec<&str>, ) -> bool {
    if true_list.contains(&message){
        return true
    }
    if false_list.contains(&message){
        return false
    }
    false
}

