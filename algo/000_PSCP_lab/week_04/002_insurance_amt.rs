/*

An insurance company calculates premium as follows:

i) If a person’s health is excellent and the person is between 25 and 35 years
of age and lives in a city and is a male then premium is Rs.4 per thousand and the policy
amount cannot exceed Rs.2 lacks.

ii) If a person satisfies all the above conditions and is female then the premium
is Rs.3 per thousand and the policy amount cannot exceed Rs.1 lack.

iii) If a person’s health is poor and the person is between 25 and 35 years of
age and lives in a village and is a male then premium is Rs.6 per
thousand and the policy cannot exceed Rs. 10000.

iv) In all other cases the person is not insured.

Write a program to determine whether the person should be insured or not,
his/her premium rate and maximum amount for which he/she can be insured.

 */


#[derive(PartialEq)]
enum HealthCondition {
    Good,
    Poor,
}

#[derive(PartialEq)]
enum LivingLocation {
    City,
    Rural,
}

#[derive(PartialEq)]
enum Gender {
    Male,
    Female,
}

#[derive(PartialEq)]
enum Eligibility {
    Eligible,
    NotEligible,
}

struct Person {
    name: String,
    age: u8,
    gender: Gender,
    health_condition: HealthCondition,
    living_location: LivingLocation,
}

fn main() {
    let person = Person {
        name: "Naveen Pantra".to_string(),
        age: 24,
        gender: Gender::Male,
        health_condition: HealthCondition::Good,
        living_location: LivingLocation::Rural,
    };
    if !(person.age >= 25 && person.age <= 35) {
        eprintln!("Not eligible for insurance");
        return;
    }
    let (premium, max_amount, eligibility): (u8, u32, Eligibility) = match person.health_condition {
        HealthCondition::Good => {
            match person.gender {
                Gender::Male => {
                    (4, 200000, Eligibility::Eligible)
                }
                Gender::Female => {
                    (3, 100000, Eligibility::Eligible)
                }
            }
        }
        HealthCondition::Poor => {
            if person.gender == Gender::Male
                && person.health_condition == HealthCondition::Poor
                && person.living_location == LivingLocation::Rural
            {
                (6, 10000, Eligibility::Eligible)
            } else {
                (0, 0, Eligibility::Eligible)
            }
        }
    };
    if eligibility == Eligibility::NotEligible {
        eprintln!("Not eligible for insurance");
        return;
    }
    println!("{} is eligible for insurance of max amount {} at {} per thousand", person.name, max_amount, premium);
}
