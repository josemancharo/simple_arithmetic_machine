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
}