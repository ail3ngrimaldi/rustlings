// tests2.rs
// This test has a problem with it -- make the test compile! Make the test
// pass! Make the test fail!
// Execute `rustlings hint tests2` or use the `hint` watch subcommand for a hint.


const number: i32 = 3;

#[cfg(test)]
mod tests {

use crate::number;    
    #[test]
    fn you_can_assert_eq() {
        assert_eq!(number, 3); //toma dos valores y verifica que sean iguales
        assert_eq!(false, false);
        assert_eq!(true, true);
    }
}
