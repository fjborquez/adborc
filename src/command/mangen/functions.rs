use crate::command::filesystem::functions::create_file;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::PathBuf;
use crate::command::commands::commands::Commands;
use crate::command::mangen::consumer_mangen::ConsumerMangen;
use crate::command::mangen::generable::Generable;
use crate::command::mangen::marketmaker_mangen::MarketMakerMangen;
use crate::command::mangen::supplier_mangen::SupplierMangen;

pub fn print_mangen_error(result: &io::Result<()>, message: String) {
    let error_message: String = result.as_ref().unwrap_err().to_string();
    let mut temp_message: String = message;
    temp_message.push_str(" ");
    temp_message.push_str(error_message.as_str());

    if result.is_err() {
        println!("{}", temp_message);
    }
}

pub fn create_man_page(path: PathBuf) -> File {
    let file_path = path.join("adborc.man");
    return create_file(file_path);
}

#[cfg(feature = "mangen")]
pub fn print_mangen(
    &mut command: &Command,
    section: String,
    message: String,
    out_file: &mut dyn Write,
    path: &String,
) {
    use clap::Command;
    use clap_mangen::Man;

    let man_page = Man::new(command);
    let mut result;

    match section {
        String::from("title") => result = man_page.render_title(out_file),
        String::from("section") => result = man_page.render_name_section(out_file),
        String::from("synopsis") => result = man_page.render_synopsis_section(out_file),
        String::from("description") => result = man_page.render_description_section(out_file),
        String::from("options") => result = man_page.render_options_section(out_file),
        String::from("subcommands") => result = man_page.render_subcommands_section(out_file),
        String::from("authors") => result = man_page.render_authors_section(out_file),
        String::from("version") => result = man_page.render_version_section(out_file),
        &_ => panic!("There is no such section"),
    }

    print_mangen_error(&result, message);
    println!("Wrote man page to {}", path);
}

/// Implementation for mangen command, available only when mangen feature is enabled.
#[cfg(feature = "mangen")]
pub fn create_mangen(path: Option<String>) {
    use clap::CommandFactory;
    use std::env;
    use std::fs::File;
    use std::path::PathBuf;
    use create_man_page;
    use crate::command::filesystem::functions::create_path;

    let path = create_path(path);
    let file = create_man_page(path);

    let &mut out_file = file.unwrap();
    let &mut cli_command = Cli::command();
    let path_text: String = path.into_os_string().into_string().unwrap();

    generate_subcommand_mangen(cli_command, out_file, &path_text);

    // TODO: Should we be using the render method instead setting up every page section?
    // TODO: Iterate
    print_mangen(
        &cli_command,
        String::from("title"),
                     String::from("Error generating man page title:"),
        out_file,
        &path_text,
    );
    print_mangen(
        &cli_command,
        String::from("section"),
                     String::from("Error generating man page name section:"),
        out_file,
        &path_text
        ,
    );
    print_mangen(
        &cli_command,
        String::from("synopsis"),
                     String::from("Error generating man page synopsis:"),
        out_file,
        &path_text,
    );
    print_mangen(
        &cli_command,
        String::from("description"),
                     String::from("Error generating man page description:"),
        out_file,
        &path_text,
    );
    print_mangen(
        &cli_command,
        String::from("options"),
                     String::from("Error generating man page options:"),
        out_file,
        &path_text,
    );
    print_mangen(
        &cli_command,
        String::from("subcommands"),
                     String::from("Error generating man page subcommands:"),
        out_file,
        &path_text,
    );

    print_mangen(
        &cli_command,
        String::from("version"),
                     String::from("Error generating man page version section:"),
        out_file,
        &path_text,
    );
    print_mangen(
        &cli_command,
        String::from("authors"),
                     String::from("Error generating man page authors section:"),
        out_file,
        &path_text,
    );

    println!("Wrote man page to {}", path.join("adborc.man").display());
    return;
}

pub fn generate_subcommand_mangen(command: &Commands, file: &mut dyn Write, path: &String) -> () {
    MarketMakerMangen::generate(command, file, path);
    SupplierMangen::generate(command, file, path);
    ConsumerMangen::generate(command, file, path);
}
