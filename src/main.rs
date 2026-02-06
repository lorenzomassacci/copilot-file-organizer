use std::fs;
use std::path::PathBuf;

fn main() {
    // Ottieni la directory home dell'utente
    let home_dir = dirs::home_dir().expect("Impossibile trovare la home directory");
    let download_dir: PathBuf = home_dir.join("Downloads");

    println!("Contenuto della cartella: {}", download_dir.display());
    match fs::read_dir(&download_dir) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(e) => println!("{}", e.file_name().to_string_lossy()),
                    Err(err) => eprintln!("Errore nella lettura di un file: {}", err),
                }
            }
        }
        Err(err) => eprintln!("Errore nell'apertura della cartella Downloads: {}", err),
    }
}

