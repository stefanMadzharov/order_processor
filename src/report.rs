use crate::{configs::Configs, excel, structs::parse_stcker_error::ParseStickerError};
use colored::*;
use std::fmt::Write;
use strsim::normalized_levenshtein;

pub fn print_errors(errors: &[ParseStickerError], configs: &Configs) {
    let orders = match excel::parse_orders(configs) {
        Ok(orders) => orders,
        Err(e) => {
            eprintln!("{}: {:?}", "Failed to parse orders".red().bold(), e);
            return;
        }
    };

    eprintln!(
        "\n{}: [{} {} {}]\n",
        "Filtered Errors Based on Description Similarity"
            .underline()
            .bold()
            .blue(),
        "Limit".bold(),
        configs
            .error_output_levenshtein_distance
            .to_string()
            .yellow(),
        "similarity".dimmed()
    );

    for order in &orders {
        let mut similarity_matches = String::new();
        let mut code_matches = String::new();

        for error in errors {
            let error_str = error.get_description();
            let similarity = normalized_levenshtein(
                error_str.as_str(),
                format!("{}_{}", order.code, &order.description).as_str(),
            );

            if similarity >= configs.error_output_levenshtein_distance
                && writeln!(
                    &mut similarity_matches,
                    "\t\t{} \"{}\" {} \"{}\" {} {:.2}{}",
                    "↳ Similar to file name:".cyan(),
                    error_str.yellow(),
                    "with error".dimmed(),
                    format!("{:?}", error).italic(),
                    "(similarity:".dimmed(),
                    similarity,
                    ")".dimmed(),
                )
                .is_err()
            {
                eprintln!("Write to buffer failed");
            }

            if error_str.contains(&order.code.to_string())
                && writeln!(
                    &mut code_matches,
                    "\t\t{} {}: {}",
                    "↳ Error contains code".magenta(),
                    order.code.to_string().yellow(),
                    format!("{:?}", error).italic()
                )
                .is_err()
            {
                eprintln!("Write to buffer failed");
            }
        }

        if !similarity_matches.is_empty() || !code_matches.is_empty() {
            eprintln!(
                "{}: \"{}_{}\"",
                "Order".bright_blue().bold(),
                order.code.to_string().yellow(),
                order.description.green()
            );

            if !similarity_matches.is_empty() {
                eprintln!(
                    "\t{}:\n{}",
                    "Similarity Matches".bold().cyan(),
                    similarity_matches
                );
            }

            if !code_matches.is_empty() {
                eprintln!(
                    "\t{}:\n{}",
                    "Code Containment Matches".bold().magenta(),
                    code_matches
                );
            }

            eprintln!("{}", "_".repeat(100).dimmed());
        }
    }
}
