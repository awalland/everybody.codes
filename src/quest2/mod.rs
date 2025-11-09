mod part2 {
    pub use crate::common::Point;

    pub fn part2() {
        /*
            The engraving should cover a square grid on the breastplate. The order includes a specific "complex" number A ( your notes ),
            which defines the coordinates of the first point (the top-left corner) of the grid. To determine the coordinates
            of the final point (the bottom-right corner), you simply add [1000,1000] to A. The remaining points depend on how finely the
            blacksmith decides to divide the grid, but they have to be at equal distances from each other both horizontally and vertically.
            Each point on the breastplate may or may not be engraved. To determine whether a point should be engraved, follow this rules:
            Initialise the check result as [0,0]. Then perform 100 cycles of the following steps:

                Multiply the result by itself.
                Divide the result by [100000,100000].
                Add the coordinates of the point under examination.

            If at the end of any of the 100 cycles the X or Y coordinate of the result exceeds 1000000 or falls below -1000000, that point
            will not be engraved. If we complete all 100 cycles without exceeding this range, we should engrave the point.
            The blacksmith must decide how many points he can calculate and engrave while still completing the knight’s order in time. His plan
            is to divide the area into a 101x101 grid, meaning he needs to check 10201 points. However, he has not yet finished all the calculations and is unsure whether the symbol will be distinct enough.
        */
        // A=[-21723,-68997]

        let top_left = Point::new(-21723, -68997);
        let distance = Point::new(1000, 1000);
        let bottom_right = top_left + distance;

        let result = calculate_engraved_points(top_left, bottom_right, distance, 1001);
        println!("Points to be engraved: {:?}", result);
    }
    pub fn calculate_engraved_points(
        top_left: Point,
        bottom_right: Point,
        distance: Point,
        grid_size: i64,
    ) -> i64 {
        let mut engraved_points = 0;
        let mut total_points = 0;
        let scale_factor: i64 = distance.x / (grid_size - 1);
        for x in 0..grid_size {
            for y in 0..grid_size {
                let current_point = top_left + Point::new(x * scale_factor, y * scale_factor);
                if should_engrave(current_point) {
                    engraved_points += 1;
                    //     print!("x")
                    // } else {
                    //     print!(" ")
                }

                if y == grid_size - 1 {
                    print!("\n")
                }

                total_points += 1;

                if x == grid_size - 1 && y == grid_size - 1 {
                    println!(
                        "Final point {:?}, should be {:?}.",
                        current_point, bottom_right
                    )
                }
            }
        }

        println!("Total points: {}", total_points);
        engraved_points
    }

    fn should_engrave(point: Point) -> bool {
        let one_hundred_k = Point::new(100000, 100000);
        let mut check_result = Point::default();
        for _ in 0..100 {
            check_result = (check_result * check_result / one_hundred_k) + point;
            if check_result.x > 1000000
                || check_result.y > 1000000
                || check_result.x < -1000000
                || check_result.y < -1000000
            {
                return false;
            }
        }
        true
    }

    #[cfg(test)]
    #[test]
    fn test_calculate_engraved_points() {
        let top_left = Point::new(-21723, -68997);
        let distance = Point::new(1000, 1000);
        let bottom_right = top_left + distance;
        let result = calculate_engraved_points(top_left, bottom_right, distance, 1001);
        assert_eq!(result, 117387)
    }

    #[cfg(test)]
    #[test]
    fn test_calculate_engraved_points_with_lower_resolution() {
        let top_left = Point::new(-21723, -68997);
        let distance = Point::new(1000, 1000);
        let bottom_right = top_left + distance;
        let result = calculate_engraved_points(top_left, bottom_right, distance, 101);
        assert_eq!(result, 1196)
    }
}

mod part1 {
    use crate::common::Point;

    pub fn part1() {
        /*
           To ensure you understand these calculations correctly, the instructions provide a sample number, referred to as: A
           ( your notes ), along with a sequence of operations to perform on it.
               Begin by setting the result to [0,0].
               Then, complete three cycles of the following operations:
               Multiply the result by itself.
               Divide the result by [10,10].
               Add A to the result.
               After three cycles, the final result is your answer.
        */
        // A=[165,57]
        let a = Point::new(165, 57);

        println!("The result is {:?}", calculate_result(a));
    }
    pub fn calculate_result(a: Point) -> Point {
        let mut result = Point::new(0, 0);
        for _ in 0..3 {
            result = result * result / Point::new(10, 10) + a
        }
        result
    }
    #[cfg(test)]
    #[test]
    fn test_calculate_result() {
        let result = calculate_result(Point::new(165, 57));

        assert_eq!(result, Point::new(280965, 993088))
    }
}
