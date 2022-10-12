fn main() {
    println!("Welcome to the Rabbit University Safety Testing Suite (RUSTs)");

    // Run the validation test functions
    validate_grading_system();
    validate_safety_system();
    validate_simulation();
}

/// Validate the grading system
fn validate_grading_system() {
    // Test 1: Validate the grading system
    println!("Validating grading system...");
    let carrots = 10;
    let nuts = 5;
    let seeds = 1;
    let grade = calculate_grade(carrots, nuts, seeds);

    assert!(grade == 240);
    println!("Validation complete ✅\n");
}

/// Calculate the grade
fn calculate_grade(carrots: i32, nuts: i32, seeds: i32) -> i32 {
    todo!()
}

/// Validate the safety system
fn validate_safety_system() {
    // Test 2: Validate the safety system
    println!("Validating safety system...");

    let wolves_nearby = false;
    let day_time = false;
    let has_carrot = true;
    let friends_nearby = 2;

    let safety_status =
        calculate_safety_status(wolves_nearby, day_time, has_carrot, friends_nearby);

    assert!(safety_status == true);
    println!("Validation complete ✅\n");
}

/// Calculate the safety status
fn calculate_safety_status(
    wolves_nearby: bool,
    day_time: bool,
    has_carrot: bool,
    friends_nearby: i32,
) -> bool {
    todo!()
}

/// Validate the simulation
fn validate_simulation() {
    // Test 3: Validate the simulation
    println!("Validating simulation...");

    let starting_rabbits = 332419;

    let count = simulate(starting_rabbits);

    assert_eq!(count, 91);
    println!("Validation complete ✅\n");
}

/// Simulate the rabbit population
fn simulate(starting_rabbits: i128) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::{calculate_grade, calculate_safety_status, simulate};

    #[test]
    fn test_grade_calculation() {
        assert_eq!(168, calculate_grade(5, 7, 2));
        assert_eq!(0, calculate_grade(0, 0, 0));
        assert_eq!(240, calculate_grade(10, 5, 1));
        assert_eq!(56, calculate_grade(3, 4, 1));
    }

    #[test]
    fn test_safety_calculation() {
        assert_eq!(true, calculate_safety_status(false, true, true, 2));
        assert_eq!(false, calculate_safety_status(true, false, false, 0));
        assert_eq!(true, calculate_safety_status(false, true, false, 0));
        assert_eq!(true, calculate_safety_status(false, true, false, 4));

        // From Jersey
        assert_eq!(true, calculate_safety_status(true, false, true, 0));
        assert_eq!(false, calculate_safety_status(true, false, false, 3));
        assert_eq!(true, calculate_safety_status(true, false, false, 4));
    }

    #[test]
    fn test_simulation() {
        assert_eq!(91, simulate(332_419));
        assert_eq!(129, simulate(234_345));
        assert_eq!(34, simulate(39));
        assert_eq!(770, simulate(678_293_106_536_832_832_142));
    }
}
