/***
Generation  Children    Adults  Total
1           1           0       1
2           0           1       1
3           3           1       4
4           3           4       7
5           12          7       19

 */

pub fn solve() -> String {
    let input = include_str!("../input/fib.txt").trim();

    if let Some((generations, new_pairs_per_litter)) = input.split_once(" ") {
        forecast_rabbit_population(generations.parse().unwrap(), new_pairs_per_litter.parse().unwrap())
    } else {
        panic!("Could not parse input")
    }
}

fn forecast_rabbit_population(target_generation: usize, new_pairs_per_litter: usize) -> String {
    let breeder = RabbitBreeder {
        new_pairs_per_litter
    };
    breeder.calc_generation_rabbit_pairs(target_generation).to_string()
}

struct RabbitBreeder {
    new_pairs_per_litter: usize
}

impl RabbitBreeder {
    /** This method calculates recursively backwards to build up the target generation from the base cases **/
    fn calc_generation_rabbit_pairs(&self, generation: usize) -> usize {
        return match generation {
            1 => 1,
            2 => 1,
            _ => self.calc_generation_rabbit_pairs(generation - 1) + self.calc_generation_rabbit_pairs(generation - 2) * self.new_pairs_per_litter
        }
    }

    // Note: The below implementations work as well, and arguably make the 'business logic' clearer, as opposed to
    //the more optimized yet abstract approach provided by the above function
    //
    // fn calc_generation_rabbit_pairs(&self, generation: usize, gen_minus_1_pairs: usize, gen_minus_2_pairs: usize) -> usize {
    //     let adults = match generation {
    //         1 => 0,
    //         2 => 1,
    //         _ => gen_minus_1_pairs
    //     };
    //
    //     let children = match generation {
    //         1 => 1,
    //         2 => 0,
    //         _ => gen_minus_2_pairs * self.new_pairs_per_litter
    //     };
    //
    //     return if generation == self.target_generation {
    //         adults + children
    //     } else {
    //         return self.calc_generation_rabbit_pairs(generation + 1, adults + children, gen_minus_1_pairs)
    //     }
    // }

    // fn calc_generation_rabbit_pairs(&self, generation: usize, gen_children: usize, gen_adults: usize) -> usize {
    //
    //     if generation == self.target_generation {
    //         return gen_children + gen_adults
    //     }
    //
    //     let next_gen_adults = gen_children + gen_adults;
    //     let next_get_children = gen_adults * self.new_pairs_per_litter;
    //     return self.calc_generation_rabbit_pairs(generation + 1, next_get_children, next_gen_adults)
    // }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!("19", forecast_rabbit_population(5, 3))
    }
}


