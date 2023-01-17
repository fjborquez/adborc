use crate::command::filesystem::functions::create_file;
use std::fs::File;
use std::io;
use std::path::PathBuf;

pub fn print_mangen_error(result: &io::Result<()>, message: &str) {
    let error_message: String = result.as_ref().unwrap_err().to_string();
    let mut temp_message: String = message.to_string();
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
    command: &Command,
    section: &str,
    message: &str,
    out_file: &mut dyn Write,
    path: &PathBuf,
) {
    use clap::Command;
    use clap_mangen::Man;

    let man_page = Man::new(command.to_owned());
    let mut result;

    match section {
        "title" => result = man_page.render_title(out_file),
        "section" => result = man_page.render_name_section(out_file),
        "synopsis" => result = man_page.render_synopsis_section(out_file),
        "description" => result = man_page.render_description_section(out_file),
        "options" => result = man_page.render_options_section(out_file),
        "subcommands" => result = man_page.render_subcommands_section(out_file),
        "authors" => result = man_page.render_authors_section(out_file),
        "version" => result = man_page.render_version_section(out_file),
        &_ => panic!("There is no such section"),
    }

    print_mangen_error(&result, message);
    println!("Wrote man page to {}", path.display());
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

    let mut out_file = file.unwrap();
    let cli_command = Cli::command();
    let marketmaker_subcommand = cli_command.find_subcommand("marketmaker").unwrap();
    let supplier_subcommand = cli_command.find_subcommand("supplier").unwrap();
    let consumer_subcommand = cli_command.find_subcommand("consumer").unwrap();

    // TODO: Should we be using the render method instead setting up every page section?
    // TODO: Iterate
    print_mangen(
        &cli_command,
        "title",
        "Error generating man page title:",
        out_file,
        &path,
    );
    print_mangen(
        &cli_command,
        "section",
        "Error generating man page name section:",
        out_file,
        &path,
    );
    print_mangen(
        &cli_command,
        "synopsis",
        "Error generating man page synopsis:",
        out_file,
        &path,
    );
    print_mangen(
        &cli_command,
        "description",
        "Error generating man page description:",
        out_file,
        &path,
    );
    print_mangen(
        &cli_command,
        "options",
        "Error generating man page options:",
        out_file,
        &path,
    );
    print_mangen(
        &cli_command,
        "subcommands",
        "Error generating man page subcommands:",
        out_file,
        &path,
    );

    // TODO: every subcommand should have its own mangen file
    print_mangen(
        marketmaker_subcommand,
        "subcommands",
        "Error writing MarketMaker subcommands:",
        out_file,
        &path,
    );
    print_mangen(
        supplier_subcommand,
        "subcommands",
        "Error writing Supplier subcommands:",
        out_file,
        &path,
    );
    print_mangen(
        consumer_subcommand,
        "subcommands",
        "Error writing Consumer subcommands:",
        out_file,
        &path,
    );

    print_mangen(
        &cli_command,
        "version",
        "Error generating man page version section:",
        out_file,
        &path,
    );
    print_mangen(
        &cli_command,
        "authors",
        "Error generating man page authors section:",
        out_file,
        &path,
    );

    println!("Wrote man page to {}", path.join("adborc.man").display());
    return;
}
