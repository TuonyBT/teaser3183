use std::collections::BTreeSet;

fn main() {

    let mut magnitudes = Vec::<u32>::new();

    //  The solution boils down to finding Pythagorean triplets with hypotenuse equal to the radius 50
    //  The magnitude is equal to 2(r - d) / 2r where d is either one of the sides of a given triplet
    //  Only values of d for which magnitude > 10% are allowed, i.e. d < 5

    for triplet in pythag_triplets(50) {
        for d in [triplet[0], triplet[1]] {
            if d < 45 {
                println!("d: {}", d);
                magnitudes.push(50 - d);
            }
        }
    }
    magnitudes.sort();
    println!("Magnitudes are {:?}", magnitudes);
}



//  Return all Pythagorean triplets with hypotenuse n
pub fn pythag_triplets(c: u32) -> BTreeSet<[u32; 3]> {

    let mut triplets = BTreeSet::<[u32; 3]>::new();
    let mut b = c - 1;
    let mut a = 1u32;

    while a < b {
        let a_sq = c.pow(2) - b.pow(2);
        let a_ = (a_sq as f64).sqrt();
        a = a_.floor() as u32;
        if a.pow(2) + b.pow(2) == c.pow(2) {
            triplets.insert([a, b, c]);
        }
        b -= 1;
    }
    triplets
} 