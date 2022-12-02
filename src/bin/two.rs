fn main() {
    let input = include_str!("../inputs/two/input.in");

    fn calc_score(me : &str, op : &str) -> i32 {
        let mut score : i32 = 0;
        // Draw
        if me == op {
            score += 3;
        }
        // Me wins
        else if op == "A" && me == "B"
            || op == "B" && me == "C"
            || op == "C" && me == "A" {
            score += 6;
        }
        // My choice
        match me {
            "A" => score += 1,
            "B" => score += 2,
            "C" => score += 3,
            _ => panic!("Crash and burn baby!")
        }
        score
    }

    let result : i32 = input.split("\n").into_iter()
        .map(| r | r.trim().split(" ").collect())
        .map(| symbols : Vec<&str> | {
            let op = symbols[0];
            let me = match symbols[1] {
                "X" => "A", // Rock
                "Y" => "B", // Paper
                "Z" => "C", // Scissors
                _ => panic!("Crash and burn baby!")
            };
            calc_score(me, op)
        }).into_iter().sum();
    println!("Part 1: {result}");


    let result2 : i32 = input.split("\n").into_iter()
        .map(| r | r.trim().split(" ").collect())
        .map(| symbols : Vec<&str> | {
            let op = symbols[0];
            let outcome = symbols[1];
            let me = match outcome {
                // Lose
                "X" => match op {
                    "A" => "C",
                    "B" => "A",
                    "C" => "B",
                    _ => panic!("Crash and burn baby!")
                },
                // Draw
                "Y" => op,
                // Win
                "Z" => match op {
                    "A" => "B",
                    "B" => "C",
                    "C" => "A",
                    _ => panic!("Crash and burn baby!")
                },
                _ => panic!("Crash and burn baby!")
            };
            calc_score(me, op)
        }).into_iter().sum();

    println!("Part 2: {result2}");
}
