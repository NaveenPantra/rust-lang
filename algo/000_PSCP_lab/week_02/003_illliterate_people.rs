/*

In a town, the percentage of men is 52. The percentage of total literacy
is 48.If total percentage of literate men is 35 of the total population;
write a program to find the total number of illiterate men and women
if the population of the town is 8000.

 */

fn main() {
    let total_population = 8000;
    let men_population_percentage = 52;
    let women_population_percentage = 100 - men_population_percentage;
    let men_head_count = (men_population_percentage * total_population) / 100;
    let women_head_count = (women_population_percentage * total_population) / 100;
    let literate_men_percentage_in_total_population = 35;
    let literate_men_head_count_in_total_population = (literate_men_percentage_in_total_population * total_population) / 100;
    let illiterate_men_head_count = men_head_count - literate_men_head_count_in_total_population;
    let total_literacy_percentage = 48;
    let total_literates_head_count = (total_literacy_percentage * total_population) / 100;
    let literate_women_head_count = total_literates_head_count - literate_men_head_count_in_total_population;
    let illiterate_women_head_count = women_head_count - literate_women_head_count;
    println!("Illiterate men: {}\nIlliterate women: {}\n", illiterate_men_head_count, illiterate_women_head_count);

    dbg!(illiterate_women_head_count + illiterate_men_head_count + literate_women_head_count + literate_men_head_count_in_total_population);
}

