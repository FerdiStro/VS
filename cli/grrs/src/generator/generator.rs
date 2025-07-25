use ramhorns::Template;
use same_file::Handle;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};
use std::path::Path;

use std::fs;
use std::fs::File;
use std::io;
use std::io::Write;

use crate::merger::pdf_merger::merge_pdf;
use crate::structures::cv::CV;
use colored::Colorize;
use std::process::{Command, Stdio};



pub fn generate(cv: CV) {
    println!(
        "{} \n  {} \n  {}\n  \n",
        "Welcome to the CV generator".bold(),
        "Dependency Needed:",
        "- docker-compose",
    );

    generate_cv(&cv);

    if !cv.time_stamps.is_empty() {
        generate_time_line(&cv)
    }

    if !cv.cover_letter.is_empty() {
        generate_cover_letter(&cv);
    }
}

fn generate_cover_letter(cv: &CV) {
    let mut html_file: String = "".to_owned();
    read(&mut html_file, "html_open").expect("TODO: panic message");
    read(&mut html_file, "cover/css_style").expect("TODO: panic message");


    read(&mut html_file, "cover/html_body_cover").expect("TODO: panic message");

    read(&mut html_file, "html_body_close").expect("TODO: panic message");
    read(&mut html_file, "html_footer").expect("TODO: panic message");
    read(&mut html_file, "html_close").expect("TODO: panic message");

    let tpl = Template::new(html_file).unwrap();
    let rendered = tpl.render(&cv);

    fs::rename("../../CV.pdf", "../../CV-finish.pdf").expect("TODO: panic message");


    match write_to_file(rendered, "../../src/VC.html") {
        Ok(_) => println!("File written successfully."),
        Err(e) => eprintln!("Error writing to file: {}", e),
    }

    build_cv_docker().expect("");

    fs::rename("../../VC.pdf", "../../Cover-Letter.pdf").expect("TODO: panic message");
    fs::rename("../../CV-finish.pdf", "../../CV.pdf").expect("TODO: panic message");

}

fn generate_cv(cv: &CV) {
    let mut html_file: String = "".to_owned();

    read(&mut html_file, "html_open").expect("TODO: panic message");
    read(&mut html_file, "cv/css_style").expect("TODO: panic message");

    read(&mut html_file, "html_header").expect("TODO: panic message");
    read(&mut html_file, "html_body_open").expect("TODO: panic message");

    read(&mut html_file, "cv/html_body_cv").expect("TODO: panic message");

    read(&mut html_file, "html_body_close").expect("TODO: panic message");
    read(&mut html_file, "html_footer").expect("TODO: panic message");
    read(&mut html_file, "html_close").expect("TODO: panic message");

    let tpl = Template::new(html_file).unwrap();
    let rendered = tpl.render(&cv);

    match write_to_file(rendered, "../../src/VC.html") {
        Ok(_) => println!("File written successfully."),
        Err(e) => eprintln!("Error writing to file: {}", e),
    }

    build_cv_docker().expect("");
}

fn generate_time_line(cv: &CV) {
    fs::rename("../../VC.pdf", "../../CV-page-1.pdf").expect("TODO: panic message");

    let mut time_line_html_file: String = "".to_owned();

    read(&mut time_line_html_file, "html_open").expect("TODO: panic message");
    read(&mut time_line_html_file, "timeline/css_style").expect("TODO: panic message");

    read(&mut time_line_html_file, "html_header").expect("TODO: panic message");
    read(&mut time_line_html_file, "html_body_open").expect("TODO: panic message");

    read(&mut time_line_html_file, "timeline/html_body_timeline").expect("TODO: panic message");

    read(&mut time_line_html_file, "html_body_close").expect("TODO: panic message");

    read(&mut time_line_html_file, "html_footer").expect("TODO: panic message");
    read(&mut time_line_html_file, "html_close").expect("TODO: panic message");

    let tpl = Template::new(time_line_html_file).unwrap();
    let rendered = tpl.render(&cv);

    match write_to_file(rendered, "../../src/VC.html") {
        Ok(_) => println!("File written successfully."),
        Err(e) => eprintln!("Error writing to file: {}", e),
    }

    build_cv_docker().expect("");

    fs::rename("../../VC.pdf", "../../CV-page-2.pdf").expect("TODO: panic message");

    merge_pdf("../../CV-page-1.pdf", "../../CV-page-2.pdf", "../../CV.pdf");
}


fn read(html_file: &mut String, path: &str) -> Result<(), Error> {
    let stdout_handle = Handle::stdout()?;

    let html_open_path_string = format!("rsc/mustache/{}.mustache", path);
    let html_open_path = Path::new(&html_open_path_string);

    let html_open_handle = Handle::from_path(html_open_path)?;

    if stdout_handle == html_open_handle {
        return Err(Error::new(
            ErrorKind::Other,
            "You are reading and writing to the same file",
        ));
    } else {
        let file = File::open(&html_open_path)?;
        let file = BufReader::new(file);

        for (_num, line) in file.lines().enumerate() {
            html_file.push_str(&*line?);
            html_file.push_str("\n");
        }
    }
    Ok(())
}

fn write_to_file(content: String, path: &str) -> io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

fn build_cv_docker() -> io::Result<()> {
    println!("{}", "Building CV with docker-compose".bold());

    let script_path = "./generate.sh";
    let script_dir = Path::new("../../");

    let mut child = Command::new("sh")
        .arg(script_path)
        .stdout(Stdio::piped())
        .current_dir(script_dir)
        .spawn()?;

    if let Some(mut stdout) = child.stdout.take() {
        let mut output = String::new();
        stdout.read_to_string(&mut output)?;
        println!("Output from script: {}", output);
    }

    let status = child.wait()?;

    if status.success() {
        println!("Script executed successfully.");
    } else {
        eprintln!("Script failed with status: {}", status);
    }

    Ok(())
}
