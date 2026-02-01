use clap::{Parser, ValueEnum};
use colored::Colorize;
use std::path::PathBuf;
use anyhow::{Context, Result};

/// Clone de grep en Rust
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)] // Utilise ton Cargo.toml pour la version
struct Args {
    // --- ARGUMENTS POSITIONNELS ---

    /// Le motif à chercher (Pattern)
    #[arg(required = true)]
    pattern: String,

    /// Les fichiers dans lesquels chercher
    /// Vec<PathBuf> permet d'accepter plusieurs fichiers : minigrep "mot" f1.txt f2.txt
    #[arg(required = true)]
    files: PathBuf,

    // --- PATTERN SELECTION ---

    /// Ignorer la casse (-i)
    #[arg(short = 'i', long)]
    ignore_case: bool,

    /// Considérer le motif comme une chaîne fixe, pas une Regex (-F)
    #[arg(short = 'F', long)]
    fixed_strings: bool,

    /// Inverser la recherche (afficher ce qui ne match PAS) (-v)
    #[arg(short = 'v', long)]
    invert_match: bool,

    /// Chercher seulement les mots entiers (-w)
    #[arg(short = 'w', long)]
    word_regexp: bool,

    // --- OUTPUT CONTROL ---

    /// Afficher le numéro de ligne (-n)
    #[arg(short = 'n', long)]
    line_number: bool,

    /// Compter le nombre de lignes correspondantes (-c)
    /// Conflit : On ne peut pas afficher le texte SI on demande juste le compte
    #[arg(short = 'c', long, conflicts_with = "line_number")]
    count: bool,

    /// Ne pas afficher les messages d'erreur (-s)
    #[arg(short = 's', long)]
    no_messages: bool,

    /// Afficher uniquement la partie qui match (-o)
    #[arg(short = 'o', long)]
    only_matching: bool,

    /// Arrêter après NUM résultats (-m)
    #[arg(short = 'm', long, value_name = "NUM")]
    max_count: Option<usize>,

    // --- CONTEXT CONTROL (Avancé) ---

    /// Afficher NUM lignes avant le match (-B)
    #[arg(short = 'B', long, value_name = "NUM")]
    before_context: Option<usize>,

    /// Afficher NUM lignes après le match (-A)
    #[arg(short = 'A', long, value_name = "NUM")]
    after_context: Option<usize>,

    /// Afficher NUM lignes autour du match (-C)
    #[arg(short = 'C', long, value_name = "NUM")]
    context: Option<usize>,

    /// Colorer la sortie (--color)
    /// Utilise une Enum pour limiter les choix à always, never, auto
    #[arg(long, value_enum, default_value_t = ColorMode::Auto)]
    color: ColorMode,
}

// Pour l'argument --color, on définit une Enum
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum ColorMode {
    Always,
    Never,
    Auto,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Permet de gérer les erreurs plus proprement
    let file_path = &args.files;

    // Lecture du fichier avec gestion d'erreur
    let content = std::fs::read_to_string(file_path)
        .with_context(|| format!("Échec de la lecture du fichier {:?}", file_path))?;

    // Préparation du pattern selon l'option -i
    let pattern_lower = args.pattern.to_lowercase();
    
    // Compteur pour l'option -c
    let mut match_count = 0;

    for (index, line) in content.lines().enumerate() {
        // 1. Détection du match
        let is_match = if args.ignore_case {
            line.to_lowercase().contains(&pattern_lower)
        } else {
            line.contains(&args.pattern)
        };

        // Condition d'affichage selon l'inversion
        if is_match != args.invert_match {
            
            // Si on demande juste le compte, on incrémente et on continue sans afficher
            if args.count {
                match_count += 1;
                continue;
            }

            // Affichage normal
            let prefix = if args.line_number {
                format!("{}: ", (index + 1).to_string().blue())
            } else {
                String::new()
            };

            println!("{}{}", prefix, line);
        }
    }

    // Affichage du résultat de -c à la fin
    if args.count {
        println!("{}", match_count);
    }

    Ok(())
}