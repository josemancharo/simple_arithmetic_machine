#[derive(Parser)]
#[grammar = "./parser/grammar.pest"]
pub struct SamParser;
pub type SamRule = Rule; 


#[cfg(test)]
mod test {
    use pest::Parser;
 
    #[test]
    fn test_matrix_parsing(){
        const EXAMPLE: &str = "[2, 2 * 4; pi, sin(10)]";
        let mut parsed = super::SamParser::parse(super::SamRule::MatrixDefinition, EXAMPLE).unwrap();
        println!("{}", parsed.next().unwrap());
    }

    #[test]
    fn test_anonymous_function_parsing(){
        const EXAMPLE: &str = "x -> x + 1";
        const EXAMPLE_2: &str = "(x, y) -> x * y";
        let parsed = super::SamParser::parse(super::SamRule::AnonymousFunction, EXAMPLE).unwrap();
        println!("{}", parsed);
        let parsed = super::SamParser::parse(super::SamRule::AnonymousFunction, EXAMPLE_2).unwrap();
        println!("{}", parsed);
    }
}