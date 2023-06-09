mod audio_clip;
mod baza;

use audio_clip::AudioKlip;
use baza::Baza;

// Za rad sa komandnom linijom
use clap::AppSettings;
use clap::Parser;
use clap::Subcommand;
// Za obradu gresaka
use color_eyre::eyre::Result;


#[derive(Parser, Debug)]
#[clap(name = "2023_Audio-Recorder")]
#[clap(about = "Pozz")]

struct Cli{             // Cli za parsiranje komandi i formiranje strukture od njih
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]// Parsiranje podkomandi
enum Commands{              

    Record{                 // Snimanje, ime opciono
        name: Option<String>,
    },
    
    #[clap(setting(AppSettings::ArgRequiredElseHelp))] // Neophodan argument(koga pustamo)
    Play{

        name: String,
    },
    
    List{},
    
    #[clap(setting(AppSettings::ArgRequiredElseHelp))] // Neophodan argument(koga brisemo)
    Delete{
        name:String,
    },
       
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let args = Cli::parse();
    let baza = Baza::open()?;
    
    match args.command{                     // Switch na osnovu komande

        Commands::Record { name } => {
            //println!("Snimanje {:?}", name);
            
            let name = name.unwrap_or_else(|| "audio_recording".to_string());
            let mut clip = AudioKlip::record(name)?.resample(44100);
            baza.save(&mut clip)?;
        }

        Commands::Play { name } => {
            if let Some(clip) = baza.load(&name)? {
                //println!("Reprodukcija {:?}", name);
                clip.play()?;
            }
            else {
                eprintln!("Audio zapis sa tim imenom ne postoji!");
            }
            
            
        }

        Commands::List{} => {
            println!("\nLista audio zapisa:\n");
            println!("\tid\tnaziv\n");
            for pod in baza.list()? {
                println!("\t{}\t{}", pod.id, pod.name);
            }
            
        }
        Commands::Delete {name} => {

            println!("\nBrisanje {:?} u toku.", name);
            baza.delete(&name)?;

            println!("\nTrenutna lista audio zapisa:\n");
            println!("\tid\tnaziv\n");
            for pod in baza.list()? {
                println!("\t{}\t{}", pod.id, pod.name);
            }
        }
    }

    Ok(())
}
