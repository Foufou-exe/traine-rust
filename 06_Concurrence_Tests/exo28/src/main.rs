// exo28/src/main.rs

// Une fonction simple à tester
fn est_pair(n: i32) -> bool {
    if n % 2 == 0 {
        true
    } else {
        false
    }
}

// Module dédié aux tests.
// #[cfg(test)] signifie que ce code ne sera compilé que lors d'un 'cargo test'.
#[cfg(test)]
mod tests {
    // Importe tout ce qui est défini dans le module parent (pour accéder à 'est_pair')
    use super::*;

    #[test]
    fn test_pair() {
        // Vérifie que est_pair(4) retourne bien true
        assert_eq!(est_pair(4), true);
    }

    #[test]
    fn test_impair() {
        // Vérifie que est_pair(5) retourne bien false
        assert_eq!(est_pair(5), false);
    }
}

fn main() {
    // La fonction main est vide ici car tout se passe dans les tests
}
