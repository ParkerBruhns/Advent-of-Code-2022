/* GOALS:
 *    Write clean, readable code
 *    Write "rusty" code -more declarative code
 */

fn main() {
    println!("hello world");
    let path = r"src/input.txt";
    let input = std::fs::read_to_string(path).unwrap().lines().map(String::from).collect::<Vec<String>>();
    
    for (i, line) in input.into_iter().enumerate() {
        let vec = line.split_whitespace().collect::<String>();
        println!("{}", vec);
        // vec
    }
    
}
