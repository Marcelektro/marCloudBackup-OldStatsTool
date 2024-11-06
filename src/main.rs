mod dce_mappings;


use std::ffi::OsStr;
use std::{env, fs};
use std::io::BufReader;
use std::path::Path;
use chrono::{DateTime, Utc};
use regex::Regex;


fn main() -> std::io::Result<()> {

    // Define variables
    let input_dir = &env::var("INPUT_DIRECTORY").unwrap_or("./input".to_string());
    let output = &env::var("OUTPUT_DIRECTORY").unwrap_or("./output".to_string());
    //


    // Regex patterns
    let stats_cmd_regex: Regex = Regex::new(r"^mb!stats$").unwrap();

    let server_count_regex: Regex = Regex::new(r"Servers:\s*``(\d+)``").unwrap();
    let members_count_regex: Regex = Regex::new(r"Users:\s*``(\d+)``").unwrap();
    //



    let input_path = Path::new(input_dir);

    if !input_path.exists() {
        return Err(std::io::Error::new(std::io::ErrorKind::NotFound, format!("Input directory `{}` does not exist", input_path.to_str().unwrap())));
    }


    let output_path = Path::new(output);

    fs::create_dir_all(output_path)
        .expect("Unable to create output directory");


    let mut processed_files = 0;

    for dir_entry in fs::read_dir(input_path)? {
        let dir_entry = dir_entry?;

        let path = dir_entry.path();

        if path.extension().and_then(OsStr::to_str) != Some("json") {
            continue;
        }

        println!("Processing file `{}`...", dir_entry.file_name().to_string_lossy());

        let reader = BufReader::new(fs::File::open(path)?);


        // parse input file
        let root: dce_mappings::Root = serde_json::from_reader(reader).expect("Unexpected json formatting");


        // init res writer
        let out_file_name = "output-".to_owned() + &root.guild.id + "-" + &root.channel.id + ".csv";

        let mut out_file_writer = csv::Writer::from_path(output_path.join(out_file_name))?;

        StatisticEntry::write_header(&mut out_file_writer)?;
        //


        let mut stat_entry: Option<StatisticEntry> = None;
        let mut written_entries = 0;

        for message in root.messages.into_iter() {

            // if the message is the stats command
            if stats_cmd_regex.is_match(&message.content) {

                if let Some(stat) = &stat_entry {
                    stat.write_to_csv(&mut out_file_writer)?;
                    written_entries += 1;
                }

                stat_entry = Some(StatisticEntry::new(message.timestamp));

                continue;
            }

            if let Some(stat) = &mut stat_entry {

                if message.timestamp > stat.timestamp + chrono::Duration::hours(1) {

                    if let Some(stat) = &stat_entry {
                        stat.write_to_csv(&mut out_file_writer)?;
                        written_entries += 1;
                    }

                    stat_entry = None;

                    continue;
                }


                if let Some(bot_response_embed) = message.embeds.get(0) {

                    if let Some(bot_response_description) = &bot_response_embed.description {

                        if let Some(servers) = extract_int_value(&server_count_regex, &bot_response_description) {
                            stat.servers_sum += servers;
                        }

                        if let Some(members) = extract_int_value(&members_count_regex, &bot_response_description) {
                            stat.members_sum += members;
                        }

                    }

                }

            }

        }

        if let Some(stat) = &stat_entry {
            stat.write_to_csv(&mut out_file_writer)?;
            written_entries += 1;
        }


        println!("Processed `{} - [{}] {}`: {} entries saved.",
                 root.guild.name,
                 root.channel.category.unwrap_or("N/A".to_string()), root.channel.name,
                 written_entries
        );

        out_file_writer.flush()?;
        processed_files+=1;

    }

    println!("Processed {} files.", processed_files);

    Ok(())
}


fn extract_int_value(regex: &Regex, value: &str) -> Option<u32> {

    regex.captures(&value)
        .and_then(|t| t.get(1)?.as_str().parse::<u32>().ok())

}



struct StatisticEntry {
    timestamp: DateTime<Utc>,
    servers_sum: u32,
    members_sum: u32
}

impl StatisticEntry {
    fn new(t: DateTime<Utc>) -> Self {
        Self {
            timestamp: t,
            servers_sum: 0,
            members_sum: 0
        }
    }

    fn write_to_csv(&self, writer: &mut csv::Writer<fs::File>) -> csv::Result<()> {
        writer.write_record(&[
            self.timestamp.to_string(),
            self.servers_sum.to_string(),
            self.members_sum.to_string(),
        ])
    }

    fn write_header(writer: &mut csv::Writer<fs::File>) -> csv::Result<()> {
        writer.write_record(&[
            "timestamp",
            "servers",
            "members",
        ])
    }

}


