/// Finds the best global alignment of `a` and `b` using the Needleman-Wunsch algorithm.
pub fn needleman_wunsch(a: &str, b: &str) -> (i32, String, String) {
    let a: Vec<char> = a.chars().collect();
    let b: Vec<char> = b.chars().collect();
    let rows = a.len() + 1;
    let cols = b.len() + 1;

    // table[i][j] is the best score for aligning the first i letters of `a` with the first j letters of `b`.
    let mut table = vec![vec![0; cols]; rows];

    // This fills out the first column and row of the matrix with the penalties increasing by a multiple of two. 
    for i in 0..rows {
        table[i][0] = i as i32 * -2;
    }
    for j in 0..cols {
        table[0][j] = j as i32 * -2;
    }

    // Iterate row by row, left to right 
    // Each cell takes the best of the three ways to reach it:
    // diagonal (pair a letter from each), up (gap in b), left (gap in a)
    for i in 1..rows {
        for j in 1..cols {
            // Checks if the two letters equal each other or not (a match or mismatch)
            let pair = if a[i - 1] == b[j - 1] { 1 } else { -1 };
            // Computes the diagonal sum
            let diagonal = table[i - 1][j - 1] + pair;

            // Computes the up and left sum (a gap penalty)
            let up = table[i - 1][j] - 2;
            let left = table[i][j - 1] - 2;

            // Fills in the table with the most positive value
            table[i][j] = diagonal.max(up).max(left);
        }
    }

    // Backtrace from the bottom-right corner
    let mut aligned_a = String::new();
    let mut aligned_b = String::new();

    // Start at the bottom-right
    let mut i = a.len();
    let mut j = b.len();

    // Keep stepping back until reaching the top-left cell (0, 0)
    while i > 0 || j > 0 {
        // Check diagonal first
        if i > 0 && j > 0 {
            let pair = if a[i - 1] == b[j - 1] { 1 } else { -1 };
            // If diagonal score + current square score is current square
            if table[i][j] == table[i - 1][j - 1] + pair {
                // Diagonal, add both letters regardless of match or mismatch
                aligned_a.push(a[i - 1]);
                aligned_b.push(b[j - 1]);
                i -= 1;
                j -= 1;
                continue;
            }
        }

        // Check if the cell above plus a gap penalty is current cell. 
        if i > 0 && table[i][j] == table[i - 1][j] - 2 {
            // Up: letter from a, gap in b
            aligned_a.push(a[i - 1]);
            aligned_b.push('-');
            i -= 1;
        } else {
            // Must be left otherwise
            aligned_a.push('-');
            aligned_b.push(b[j - 1]);
            j -= 1;
        }
    }

    // Flip results
    let aligned_a = aligned_a.chars().rev().collect();
    let aligned_b = aligned_b.chars().rev().collect();

    // Return score and the sequences
    (table[a.len()][b.len()], aligned_a, aligned_b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_sequences() {
        // 3 matches and 1 gap: 3 - 2 = 1
        assert_eq!(needleman_wunsch("ACGT", "AGT"), (1, "ACGT".to_string(), "A-GT".to_string()));
    }

    #[test]
    fn test_salmonella_genome() {
        let genome_snippet = "TGGCGACAACCGTAGCGGAATATTTTCGCGACCAGGGAAAACGGGTCGTGCTTTTTATCGATTCCATGACCCGTTATGCGCGTGCTTTGCGAGACGTGGCACTGGCGTCGGGAGAGCGTCCGGCTCGTCGAGGTTATCCCGCCTCCGTATTCGATAATTTGCCCCGCTTGCTGGAACGCCCAGGGGCGACCAGCGAGGGAAGCATTACTGCCTTTTATACGGTACTGCTGGAAAGCGAGGAAGAGGCGGACCCGATGGCGGATGAAATTCGCTCTATCCTTGACGGTCACCTGTATCTGAGCAGAAAGCTGGCCGGGCAGGGACATTACCCGGCAATCGATGTACTGAAAAGCGTAAGCCGCGTTTTT";
        let test_against = "TGGCCACCACGATAGCAGAATTTTTTCGCGATAATGGAAAGCGAGTCGTCTTGCTTGCCGACTCACTGACGCGTTATGCCAGGGCCGCACGGGAAATCGCTCTGGCCGCCGGAGAGACCGCGGTTTCTGGAGAATATCCGCCAGGCGTATTTAGTGCATTGCCACGACTTTTAGAACGTACGGGAATGGGAGAAAAAGGCAGTATTACCGCATTTTATACGGTACTGGTGGAAGGCGATGATATGAATGAGCCGTTGGCGGATGAAGTCCGTTCACTGCTTGATGGACATATTGTACTATCCCGACGGCTTGCAGAGAGGGGGCATTATCCTGCCATTGACGTGTTGGCAACGCTCAGCCGCGTTTTT";

        let (score, aligned_a, aligned_b) = needleman_wunsch(genome_snippet, test_against);
        println!("score: {score}\n{aligned_a}\n{aligned_b}");
    }
}
