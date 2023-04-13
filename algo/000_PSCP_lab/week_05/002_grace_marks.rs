/*

Write a program to find the grace marks for a student using switch. The user should enter the class obtained by the student and the number
of subjects he has failed in. Use the following rules:
i) If the student gets first class and the number of subjects failed is >3, then no grace marks are awarded. If the number of subjects failed is less than or equal to ‘3’ then the grace is 5 marks per subject.
ii) If the student gets second class and the number of subjects failed in is >2, then no grace marks are awarded. If the number of subjects failed in less than or equal to ‘3’ then the grace is 4 marks per subject.
iii) If the student gets third class and the number of subjects failed in is >1, then no grace marks are awarded. If the number of subjects failed in is equal to ‘1’ then the grace is 5 marks per subject.

 */

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let [class, failed_subjects]: [u8; 2] = match args.as_slice() {
        [class, failed_subjects] => [
            class.parse::<u8>().expect("Class should be a positive integer"),
            failed_subjects.parse::<u8>().expect("Failed subject should be a positive integer"),
        ],
        _ => panic!("Please enter class and filed subjects. ex: 2 1")
    };
    let grace_marks: u8 = match class {
        1 => {
            if failed_subjects > 3 { 0_u8 } else { failed_subjects * 5_u8 }
        }
        2 => {
            if failed_subjects > 2 { 0_u8 } else { failed_subjects * 4_u8 }
        }
        3 => {
            if failed_subjects > 1 { 0_u8 } else { failed_subjects * 5_u8 }
        }
        _ => panic!("Class should be either 1, 2 or 3"),
    };
    println!("Grace marks awarded for class({class}) with failed subjects({failed_subjects}) is {grace_marks}");
}
