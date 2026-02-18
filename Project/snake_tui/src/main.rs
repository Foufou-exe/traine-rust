use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    style::{self, Color, Print, SetForegroundColor},
    terminal::{self, Clear, ClearType},
    ExecutableCommand,
};
use std::{
    collections::VecDeque,
    io::{stdout, Write},
    time::Duration,
};

// --- TYPES ---

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: u16,
    y: u16,
}

#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct GameState {
    snake: VecDeque<Point>, // On utilise VecDeque pour ajouter devant et retirer derrière efficacement
    food: Point,
    direction: Direction,
    width: u16,
    height: u16,
    game_over: bool,
}

// --- LOGIQUE (À TOI DE JOUER) ---

impl GameState {
    fn new(w: u16, h: u16) -> Self {
        let mut snake = VecDeque::new();
        // Le serpent commence au milieu, longueur 3
        snake.push_back(Point { x: w / 2, y: h / 2 });
        snake.push_back(Point { x: w / 2 - 1, y: h / 2 });
        snake.push_back(Point { x: w / 2 - 2, y: h / 2 });

        Self {
            snake,
            food: Point { x: 5, y: 5 }, // Pomme statique pour l'instant
            direction: Direction::Right,
            width: w,
            height: h,
            game_over: false,
        }
    }

    fn update(&mut self) {
        // TODO 1: Calculer la nouvelle position de la tête en fonction de self.direction
        // Attention : (0,0) est en haut à gauche.
        // let new_head = ...
        self.snake.front().unwrap(); // Juste pour éviter un warning de variable non utilisée, à remplacer par le calcul de new_head
        let new_head = match self.direction {
            Direction::Up => Point { x: self.snake.front().unwrap().x, y: self.snake.front().unwrap().y.saturating_sub(1) },
            Direction::Down => Point { x: self.snake.front().unwrap().x, y: self.snake.front().unwrap().y.saturating_add(1) },
            Direction::Left => Point { x: self.snake.front().unwrap().x.saturating_sub(1), y: self.snake.front().unwrap().y },
            Direction::Right => Point { x: self.snake.front().unwrap().x.saturating_add(1), y: self.snake.front().unwrap().y },
        };
        self.snake.push_front(new_head); // On ajoute la nouvelle tête

        // TODO 2: Vérifier les collisions (Mur ou soi-même)
        // Si collision, self.game_over = true;
        if new_head.x >= self.width || new_head.y >= self.height {
            self.game_over = true; // Collision avec le mur
        } else if self.snake.iter().skip(1).any(|&part| part == new_head) {
            self.game_over = true; // Collision avec soi-même
        }

        // TODO 3: Faire avancer le serpent
        // - Ajouter la nouvelle tête au début de self.snake (push_front)
        // - Si la tête est sur la pomme : Générer une nouvelle pomme (pour l'instant, ne rien faire)
        // - Sinon : Retirer la queue (pop_back) pour garder la même taille
        if new_head == self.food {
            // TODO 4: Générer une nouvelle pomme à une position aléatoire qui n'est pas sur le serpent
            // Pour l'instant, on laisse la même pomme
            self.food = Point { x: (self.food.x + 7) % self.width, y: (self.food.y + 3) % self.height }; // Juste pour changer la position de la pomme, à remplacer par une génération aléatoire
        } else {
            self.snake.pop_back(); // Retirer la queue pour faire avancer le serpent
        }
    }
}

// --- MOTEUR GRAPHIQUE (DÉJÀ FAIT) ---

fn main() -> std::io::Result<()> {
    // 1. Initialisation du Terminal
    terminal::enable_raw_mode()?; // Pas d'echo des touches, pas besoin d'appui sur Entrée
    let mut stdout = stdout();
    execute!(stdout, terminal::EnterAlternateScreen, cursor::Hide)?;

    // 2. Création du jeu
    let (w, h) = terminal::size()?;
    let mut game = GameState::new(w, h);

    // 3. Boucle principale (Game Loop)
    loop {
        // A. Gestion des entrées (Input) - Non bloquant !
        // On attend un événement pendant 100ms max. Si rien ne se passe, on continue (le serpent avance).
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(KeyEvent { code, .. }) = event::read()? {
                match code {
                    KeyCode::Char('q') => break,
                    KeyCode::Up => game.direction = Direction::Up,
                    KeyCode::Down => game.direction = Direction::Down,
                    KeyCode::Left => game.direction = Direction::Left,
                    KeyCode::Right => game.direction = Direction::Right,
                    _ => {}
                }
            }
        }

        // B. Mise à jour de la logique
        if !game.game_over {
            game.update();
        }

        // C. Dessin (Render)
        stdout.execute(Clear(ClearType::All))?; // Effacer l'écran
        
        // Dessiner la pomme
        stdout.execute(cursor::MoveTo(game.food.x, game.food.y))?;
        stdout.execute(SetForegroundColor(Color::Red))?;
        stdout.execute(Print("O"))?;

        // Dessiner le serpent
        stdout.execute(SetForegroundColor(Color::Green))?;
        for part in &game.snake {
            stdout.execute(cursor::MoveTo(part.x, part.y))?;
            stdout.execute(Print("█"))?;
        }

        if game.game_over {
            stdout.execute(cursor::MoveTo(0, 0))?;
            stdout.execute(SetForegroundColor(Color::White))?;
            stdout.execute(Print("GAME OVER - Appuie sur 'q' pour quitter"))?;
        }

        stdout.flush()?; // Forcer l'affichage
    }

    // 4. Nettoyage (Très important pour ne pas casser le terminal)
    execute!(stdout, cursor::Show, terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}