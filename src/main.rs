use clap::{arg, Parser}; // Interprete les commandes de shell
                         //use core::iter::IntoIterator;
use serde_derive::Deserialize; // Sert a convertir 1:1 le script rust a toml
use serde_derive::Serialize;
use std::collections::BTreeMap;
//use std::ffi::OsString;
//use std::fmt::{self, Error, Formatter};
use std::fs;
use std::path::Path;
//use std::ptr::null;
//use std::string;
use std::{thread, time};
//use toml::*; // Crate pour .toml

//tutorial-setup-01.rs
// Import the standard library's I/O module so we can read from stdin.
use std::io;
use std::error::Error;
use csv::Writer;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
struct OrbProfile {
    orb_treasury: Option<u32>,
    //original_rate: Option<u32>,
    maximum_hero_pull_potential: Option<u32>,
    optimal_hero_pull_potential: Option<u32>,
}

#[derive(Debug, Clone, Copy)]
struct ProbabilitiesToSummonOnce {
    potential_duplicate_count: u32,
    chance_to_summon_one_copy: f32,
    cumulated_no_summon_chance: f32,
}

#[derive(clap::Subcommand, Debug)]
enum Action {
    /// Fetch a print values for the treasury with the same key value (IE -k)
    Get, // <= must not be void
    /// Declare values for treasury (orb count, -t) and original odds (percentage, -o)
    Set,
    /// Shows the information of each treasury stored in the file (either default or -d database)
    Gross,
    /// Analyze the statistical likelihood of pulls targeting one character
    Analyze,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// New sub command
    #[command(subcommand)]
    command: Option<Action>,

    /// How many orbs do you want to declare
    #[arg(short, long)]
    treasury_input: Option<u32>,

    /// What is the original rate of the banner for 5-stars focus
    #[arg(short, long)]
    original_rate_input: Option<u32>,

    /// What is the total amount of characters on the banner
    #[arg(short, long)]
    banner_character_total: Option<u32>,

    /// How many characters are desired
    #[arg(short, long)]
    actual_focus_characters: Option<u32>,

    /// Name of the database that you want to consult or edit
    #[arg(short, long)]
    database: Option<String>,

    /// Name (key) of the treasury (value) that you want to consult or edit
    #[arg(short, long)]
    key: Option<String>,

    /// Choose which projection (if any) you want to see when analyzing a treasury
    #[arg(short, long)]
    projection: Option<String>,
}

fn main() {
    let mut prog_argument = Args::parse();

    let treasury_key: String;

    // apparement il faut trouver le pathbuf du repertoir, suivi par une transition en os_string (?) suivi par une transition en string?
    let path_string: String;
    let path_string_csv_reader: String;
    let path = std::env::current_dir();
    let mut os_path = path.expect("should be a directory").into_os_string();
    let mut csv_reader_var; 
    let mut os_path_csv_reader = std::env::current_dir().expect("should be a directory").into_os_string();
    // Le nom du fichier est maintenant une operation dans un match
    // os_path.push(r"\Orb_Treasury");
    // path_string = os_path.into_string().unwrap();

    //
    
    match prog_argument.actual_focus_characters {
        None => {
            prog_argument.actual_focus_characters = Some(1);
        }

        Some(_t) => {}
    }

    match prog_argument.banner_character_total {
        None => {
            prog_argument.banner_character_total = Some(12);
        }

        Some(_t) => {}
    }

    match prog_argument.original_rate_input {
        None => {
            prog_argument.original_rate_input = Some(8);
        }

        Some(_t) => {}
    }

    match prog_argument.projection {
        None => {
            prog_argument.projection = Some("Optimal".to_string());
        }

        Some(ref _t) => {}
    }

    //os_path_csv_reader
    match &prog_argument.database {
        None => {
            os_path.push(r"\Orb_Treasury");
            os_path.push(r".toml");
            path_string = os_path.into_string().unwrap();
            os_path_csv_reader.push(r"\Orb_Treasury");
            os_path_csv_reader.push(r".csv");
            csv_reader_var = csv::Reader::from_path(&os_path_csv_reader);
            path_string_csv_reader = os_path_csv_reader.into_string().unwrap();
        }
        Some(t) => {
            os_path.push(r"\");
            os_path.push(t);
            os_path.push(r".toml");
            path_string = os_path.into_string().unwrap();

            os_path_csv_reader.push(r"\");
            os_path_csv_reader.push(t);
            os_path_csv_reader.push(r".csv");
            csv_reader_var = csv::Reader::from_path(&os_path_csv_reader);
            path_string_csv_reader = os_path_csv_reader.into_string().unwrap();
        }
    }


    ;

    for result in csv_reader_var {
        // An error may occur, so abort the program in an unfriendly way.
        // We will make this more friendly later!
        let record = result;
        // Print a debug version of the record.
        println!("{:?}", record);
    }


    // println!("{}", Path::new(&path_string).exists());

    match &prog_argument.key {
        None => {
            treasury_key = String::from("Treasury");
        }
        Some(k) => {
            treasury_key = String::from(k);
        }
    }

    match prog_argument.command {
        Some(Action::Set) => {
            let mut orb_profile_map: BTreeMap<String, OrbProfile> = BTreeMap::new();

            let mut testing_orb_profile = OrbProfile {
                orb_treasury: Some(prog_argument.treasury_input.unwrap()),
                // original_rate: Some(prog_argument.original_rate_input.unwrap()),
                maximum_hero_pull_potential: None,
                optimal_hero_pull_potential: None,
            };

            //
            testing_orb_profile.maximum_hero_pull_potential = Some(
                // (testing_orb_profile.orb_treasury.unwrap()
                //     - (testing_orb_profile.orb_treasury.unwrap() % 20))
                //     / 4,
                testing_orb_profile.orb_treasury.unwrap() / 20 * 5,
            );

            testing_orb_profile.optimal_hero_pull_potential = Some(
                (testing_orb_profile.orb_treasury.unwrap()
                    - (testing_orb_profile.orb_treasury.unwrap() % 5))
                    / 5,
            );

            println!(
                "Created a treasury named {} which contains {} orbs.",
                treasury_key,
                prog_argument.treasury_input.unwrap()
            );

            orb_profile_map.insert(String::from(treasury_key), testing_orb_profile);

            let toml_string = toml::to_string(&orb_profile_map).unwrap();

            fs::write(path_string, &toml_string).expect("Could not write database...");
        }

        // getting the content of a default treasury (IE hardcode)
        Some(Action::Get) => {
            // println!(
            //     "the current path is : {}, and it's existence is {}",
            //     path_string,
            //     Path::new(&path_string).exists()
            // );

            match Path::new(&path_string).exists() {
                true => {
                    let mut orb_profile_map: BTreeMap<String, OrbProfile> =
                        toml::from_str(&fs::read_to_string(&path_string).unwrap()).unwrap();

                    let orb_treasury_reviewed: &mut OrbProfile;

                    match prog_argument.key {
                        None => {
                            println!("You will need to enter an exact key value to use the 'get' command.");
                        }
                        Some(q) => {
                            orb_treasury_reviewed =
                                orb_profile_map.get_mut(&String::from(q)).unwrap();

                            println!(
                                "Your treasury contains {} orbs",
                                orb_treasury_reviewed.orb_treasury.unwrap()
                            );
                            println!(
                                "You can pull {} heroes, presuming you pull full circles",
                                orb_treasury_reviewed.maximum_hero_pull_potential.unwrap()
                            );
                            println!(
                                "Presuming you snipe, you can pull {} heroes",
                                orb_treasury_reviewed.optimal_hero_pull_potential.unwrap()
                            );
                            // println!(
                            //     "The reported rate on the target banner is {}",
                            //     orb_treasury_reviewed.original_rate.unwrap()
                            // );
                        }
                    }
                }

                false => {
                    println!("You will need to enter an existing database value to use the 'get' command.");
                    println!("Consider using the set value to set a default database by leaving the database argument blank.");
                }
            }
        }

        Some(Action::Gross) => match Path::new(&path_string).exists() {
            true => {
                let orb_profile_map: BTreeMap<String, OrbProfile> =
                    toml::from_str(&fs::read_to_string(&path_string).unwrap()).unwrap();

                for (orb_profile, v) in orb_profile_map {
                    println!("{orb_profile}");
                    println!("This treasury contains {} orbs", v.orb_treasury.unwrap());
                    println!(
                        "It has enough orbs to pull {} heroes, if pulling full circles",
                        v.maximum_hero_pull_potential.unwrap()
                    );
                    println!(
                        "Presuming you snipe, it has enough orbs for {} heroes",
                        v.optimal_hero_pull_potential.unwrap()
                    );
                    // println!(
                    //     "The reported rate on the target banner is {}",
                    //     v.original_rate.unwrap()
                    // );
                }
            }

            false => {
                println!("There doesn't seem to be a database in this directory.");
                println!("Directory {}", &path_string);
            }
        },

        Some(Action::Analyze) => {
            let mut orb_profile_map: BTreeMap<String, OrbProfile> =
                toml::from_str(&fs::read_to_string(&path_string).unwrap()).unwrap();

            let orb_treasury_reviewed: &mut OrbProfile =
                orb_profile_map.get_mut("Treasury").unwrap();

            println!("Treasury's inventory begins...");
            println!(
                "This treasury contains {} orbs",
                orb_treasury_reviewed.orb_treasury.unwrap()
            );
            println!(
                "It has enough orbs to pull {} heroes, if pulling full circles",
                orb_treasury_reviewed.maximum_hero_pull_potential.unwrap()
            );
            println!(
                "Presuming you snipe, it has enough orbs for {} heroes",
                orb_treasury_reviewed.optimal_hero_pull_potential.unwrap()
            );

            println!("...Treasury's inventory ends");

            println!("Treasury's analysis begins...");
            let abbr_original_rate = prog_argument.original_rate_input.unwrap() as f32;
            let abbr_banner_characters = prog_argument.banner_character_total.unwrap() as f32;
            let abbr_focus_count = prog_argument.actual_focus_characters.unwrap() as f32;
            let rate_per_character = abbr_original_rate / abbr_banner_characters as f32;
            println!(
                "Analyzing odds: {} % / {} (total focus characters) = {} %",
                abbr_original_rate, abbr_banner_characters, rate_per_character as f32
            );
            println!(
                "Result: {} % chance to get a specific character.",
                rate_per_character
            );
            let real_character_rate = (rate_per_character * abbr_focus_count) / 100.0;
            println!(
                "{} character(s) are desired out of {},",
                abbr_focus_count, abbr_banner_characters
            );
            println!(
                "meaning there is a {} % odds of getting any preferred character (per session)",
                real_character_rate * 100.0
            );

            let mut orb_stack_5 = orb_treasury_reviewed.orb_treasury.unwrap()
                - (orb_treasury_reviewed.orb_treasury.unwrap() % 5);
            let mut orb_stack_20 = orb_treasury_reviewed.orb_treasury.unwrap()
                - (orb_treasury_reviewed.orb_treasury.unwrap() % 20);
            let mut current_character_summoned: f32 = 1.00;
            let mut current_summon_rate: f32 = real_character_rate;
            let mut total_summon_rate: f32 = 0.00;

            let min_pulls_for_eleven_heroes: u32 =
                orb_treasury_reviewed.optimal_hero_pull_potential.unwrap() / 10;

            let mut final_pulls_for_eleven_rates: f32;

            let mut favored_unit_summoned_once = ProbabilitiesToSummonOnce {
                potential_duplicate_count: 0,
                cumulated_no_summon_chance: 0.00,
                chance_to_summon_one_copy: 0.00,
            };

            match prog_argument.projection {
                None => {
                    println!("Analysis concluded.")
                }

                Some(t) => {
                    if t == "Optimal" {
                        println!(
                            "Beginning summoning projections with {} attempts",
                            orb_treasury_reviewed.optimal_hero_pull_potential.unwrap()
                        );
                        // println!("Initial rate of success: {}", current_summon_rate);

                        favored_unit_summoned_once.cumulated_no_summon_chance =
                            1.00 - real_character_rate;
                        final_pulls_for_eleven_rates = 1.00 - real_character_rate;

                        let mut pity_rate_tracker: f32 = (abbr_original_rate/abbr_banner_characters) * (abbr_banner_characters - abbr_focus_count);
                        let mut pity_character_tracker: u32 = 0;
                        let mut pity_character_no_summon_chance: f32 = 
                            1.00 - (pity_rate_tracker / 100.0);


                        let mut csv_vector: Vec<String> = Vec::new();
                        let mut csv_vector_tenth = 1;
                        
                        while orb_stack_5 >= 5 {

                            if ((favored_unit_summoned_once.cumulated_no_summon_chance) * 100.00)
                                >= 99.0
                            {
                                //println!("{}th summon, current summon rate is {}, whereas the cumulated no summon chance is {}", current_character_summoned, current_summon_rate, favored_unit_summoned_once.cumulated_no_summon_chance);
                                favored_unit_summoned_once.potential_duplicate_count = favored_unit_summoned_once.potential_duplicate_count + 1;
                                current_summon_rate = real_character_rate;
                                favored_unit_summoned_once.cumulated_no_summon_chance =
                                    1.0 - real_character_rate;
                                
                                pity_rate_tracker = abbr_original_rate / (abbr_banner_characters - 1.0 );
                                pity_character_no_summon_chance = 
                                    1.00 - (pity_rate_tracker / 100.0);
                            }

                            if ((pity_character_no_summon_chance) )
                                <= 0.01
                            {
                                //println!("{}th summon, current summon rate is {}, whereas the cumulated no summon chance is {}", current_character_summoned, current_summon_rate, favored_unit_summoned_once.cumulated_no_summon_chance);
                                current_summon_rate = real_character_rate;
                                favored_unit_summoned_once.cumulated_no_summon_chance =
                                    1.0 - real_character_rate;
                                
                                pity_character_tracker += 1;
                                pity_rate_tracker = abbr_original_rate / (abbr_banner_characters - 1.0 );
                                pity_character_no_summon_chance = 
                                    1.00 - (pity_rate_tracker / 100.0);

                                    println!("Practically assured pity character\n\
                                    {} pity {} so far!", 
                                    pity_character_tracker,
                                    match pity_character_tracker {
                                        0 => "Character",
                                        1 => "Character",
                                        2_u32..=u32::MAX => "Characters"
                                    });
                                thread::sleep(time::Duration::from_secs_f32(1.0));
                                
                            }

                            println!(
                                "\n\
                                Chance to summon a preferred character {0:>14} % | Chance to summon a pity breaker: {1:>11} %",
                                current_summon_rate * 100.0,
                                (1.00 - pity_character_no_summon_chance) * 100.0
                            );
                            //thread::sleep(time::Duration::from_secs_f32(0.0));
                            

                            
                            println!(
                                "{0:>4} - 5 = {1:>4} | + 1 character summoned (current: {2:>4})",
                                orb_stack_5,
                                orb_stack_5 - 5,
                                current_character_summoned
                            );
                            thread::sleep(time::Duration::from_secs_f32(0.05));

                            
                            if current_character_summoned <= min_pulls_for_eleven_heroes as f32 {
                                //println!("{} ({} %): final pulls for eleven rates = {}", current_character_summoned, current_summon_rate * 100.0, final_pulls_for_eleven_rates);
                                final_pulls_for_eleven_rates *= 1.00 - current_summon_rate;
                            }

                            current_character_summoned += 1.00;

                            favored_unit_summoned_once.cumulated_no_summon_chance *=
                                1.00 - current_summon_rate;

                            pity_character_no_summon_chance *=
                                1.00 - (pity_rate_tracker / 100.00);

                            

                            if ((current_character_summoned - 1.00) % 5.0) == 0.0 {
                                //println!("Now doing this math: {} current summon rate + {} scaling bonus", current_summon_rate, ((abbr_focus_count/100.0 )* (0.5 / abbr_banner_characters)));
                                current_summon_rate = current_summon_rate
                                    + ((abbr_focus_count / 100.0) * (0.5 / abbr_banner_characters));

                                pity_rate_tracker = pity_rate_tracker
                                    + ((0.5 / abbr_banner_characters) * (abbr_banner_characters - abbr_focus_count)); 
                            }

                            orb_stack_5 = orb_stack_5 - 5;
                            // if orb_treasury_reviewed.optimal_hero_pull_potential.unwrap() > 10 {
                            //     if min_pulls_for_eleven_heroes as f32 == current_character_summoned {

                            //         favored_unit_summoned_once.chance_to_summon_one_copy = 1.0 - favored_unit_summoned_once.cumulated_no_summon_chance;

                            //         final_pulls_for_eleven_rates = f32::powf(
                            //             favored_unit_summoned_once.chance_to_summon_one_copy,
                            //             10.0,
                            //         );
                            //     }
                            // }

                            


                            csv_vector.push(
                                "\n".to_owned() +
                                &(current_character_summoned as u32 - 1).to_string() + "," +
                                &(current_summon_rate * 100.0).to_string() + "," +
                                &csv_vector_tenth.to_string() + "," +
                                &pity_character_tracker.to_string()
                            );

                            if (current_character_summoned as u32 - 1) % min_pulls_for_eleven_heroes == 0 {
                                csv_vector_tenth += 1;
                            }
                            
                        }

                        csv_writer_builder(csv_vector);

                        println!(
                            "\n\
                            ... end of summoning projections. The projection presumed {} pulls.",
                            orb_treasury_reviewed.optimal_hero_pull_potential.unwrap()
                        );
                        interpret_output(favored_unit_summoned_once);
                        if orb_treasury_reviewed.optimal_hero_pull_potential.unwrap() > 10 {
                            println!("You have {} % chance that the stars will align and that you'll pull 10 copies of a desired hero.", (f32::powf(1.0-final_pulls_for_eleven_rates, 10.0))*100.0);
                            println!("You would need to pull 1 copy of the desired hero every {} pull(s).\n\
                            You have {} % chance to pull a desired character every {} pulls.", min_pulls_for_eleven_heroes, (1.0-final_pulls_for_eleven_rates)*100.0, min_pulls_for_eleven_heroes);
                        }

                        
                            match pity_character_tracker {
                                0 => println!(
                                    "\n\
                                    Pity report:\n\
                                    You might get pity characters, but you are not assured of getting any."),
                                1 => println!(
                                    "\n\
                                    Pity report:\n\
                                    You can expect around 1 pity character, but probably more."),
                                2_u32..=u32::MAX => println!(
                                    "\n\
                                    Pity report:\n\
                                    You can expect around{0:>3} pity characters, but probably more.", pity_character_tracker)
                            }
                        
                    }

                    if t == "Maximal" {
                        println!("Beginning summoning projections...");

                        favored_unit_summoned_once.chance_to_summon_one_copy =
                            1.00 - current_summon_rate / 100.00;

                        while orb_stack_20 > 0 {
                            if ((1.00 - favored_unit_summoned_once.chance_to_summon_one_copy)
                                * 100.00)
                                >= 99.99
                            {
                                favored_unit_summoned_once.potential_duplicate_count =
                                    favored_unit_summoned_once.potential_duplicate_count + 1;
                                current_summon_rate = real_character_rate * abbr_focus_count;
                                favored_unit_summoned_once.chance_to_summon_one_copy =
                                    1.00 - current_summon_rate / 100.00;
                            }

                            println!(
                                "Chance to summon a preferred character {} %",
                                current_summon_rate
                            );
                            println!(
                                "{} - 5 = {} | + 1 character summoned (current: {})",
                                orb_stack_20,
                                orb_stack_20 - 5,
                                current_character_summoned
                            );
                            current_character_summoned = current_character_summoned + 1.00;
                            println!(
                                "{} - 4 = {} | + 1 character summoned (current: {})",
                                orb_stack_20 - 5,
                                orb_stack_20 - 9,
                                current_character_summoned
                            );
                            current_character_summoned = current_character_summoned + 1.00;
                            println!(
                                "{} - 4 = {} | + 1 character summoned (current: {})",
                                orb_stack_20 - 9,
                                orb_stack_20 - 13,
                                current_character_summoned
                            );
                            current_character_summoned = current_character_summoned + 1.00;
                            println!(
                                "{} - 4 = {} | + 1 character summoned (current: {})",
                                orb_stack_20 - 13,
                                orb_stack_20 - 17,
                                current_character_summoned
                            );
                            current_character_summoned = current_character_summoned + 1.00;
                            println!(
                                "{} - 3 = {} | + 1 character summoned (current: {})",
                                orb_stack_20 - 17,
                                orb_stack_20 - 20,
                                current_character_summoned
                            );
                            current_character_summoned = current_character_summoned + 1.00;
                            current_summon_rate = current_summon_rate
                                + (abbr_focus_count * (0.5 / abbr_banner_characters));
                            orb_stack_20 = orb_stack_20 - 20;
                            total_summon_rate = total_summon_rate + current_summon_rate;

                            if current_character_summoned > 5.0 {
                                favored_unit_summoned_once.chance_to_summon_one_copy =
                                    favored_unit_summoned_once.chance_to_summon_one_copy
                                        * (1.00 - (current_summon_rate as f32 / 100.00));
                            }
                        }
                        println!("... end of summoning projections.");

                        interpret_output(favored_unit_summoned_once);
                    }
                }
            }
        }

        None => {
            println!("Did you mean to enter a command?");
        }
    }
}

fn interpret_output(
    ProbabilitiesToSummonOnce {
        potential_duplicate_count,
        chance_to_summon_one_copy: _,
        cumulated_no_summon_chance,
    }: ProbabilitiesToSummonOnce,
) {
    match potential_duplicate_count {
        0 => {
            println!(
                "{} % chances to get at least one desired character.",
                (1.0 - cumulated_no_summon_chance) * 100.00
            );
        }

        1 => {
            println!("High likelihood of getting at least 1 desired character. There is {} % chance to get an additional character beyond that count.", (1.0 - cumulated_no_summon_chance)* 100.00);
        }

        2_u32..=u32::MAX => {
            println!("High likelihood of obtaining at least {} desired units. {} % chance to get an additional character beyond that count.", potential_duplicate_count, (1.0 - cumulated_no_summon_chance) * 100.00);
        }
    }
}

fn csv_writer_builder(T: Vec<String>) -> Result<(), Box<dyn Error>> {
    let mut csv_writer = Writer::from_path("analysis_report.csv");

    let mut csv_writers_string: String = "Character summoned, % chance for favored character, Tenth of the total summon, Pity Breaker Count".to_string(); 
    //csv_writer?.write_record(&["Character summoned", "% chance for favored character", "Tenth of the total summon", "Pity Breaker Count"])?;
    //csv_writer?.write_record(T)?;
    for i in &T {
        // csv_writer?.write_record(i.to_string())?;
        csv_writers_string = csv_writers_string + &i.to_string();        
    }

    csv_writer?.write_record(&[csv_writers_string])?;
    Ok(())
}
