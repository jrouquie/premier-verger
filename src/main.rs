use std::cmp::max;
// use std::cmp::min;

fn main() {
    // Possible board positions: nb remaining fruits on each tree (0..5), then raven's position (0..6).
    let mut positions = [[[[[-999999999.;6];5];5];5];5];
    println!("position,proba to win");
    for i in 0..5 {
        for j in 0..5 {
            for k in 0..5 {
                for l in 0..5 {
                    for c in (0..6).rev() { // c for raven
                        if i == 0 && j == 0 && k == 0 && l == 0 {
                            positions[0][0][0][0][c] = 1.;
                            println!("'{i}{j}{k}{l}{c},{}", positions[i][j][k][l][c]);
                            continue
                        }

                        let mut sum_expectancies = 0.0;
                        let mut num_cases = 0.0;
                        // 4 cases where the die lands on a colored side.
                        // Increase num_cases only if we don't reject the die outcome,
                        // ie the tree of the drawn color is not empty.
                        if i > 0 {
                            num_cases += 1.;
                            assert!(positions[i-1][j][k][l][c] >= 0.);
                            sum_expectancies += positions[i-1][j][k][l][c]
                        }
                        if j > 0 {
                            num_cases += 1.;
                            assert!(positions[i][j-1][k][l][c] >= 0.);
                            sum_expectancies += positions[i][j-1][k][l][c]
                        }
                        if k > 0 {
                            num_cases += 1.;
                            assert!(positions[i][j][k-1][l][c] >= 0.);
                            sum_expectancies += positions[i][j][k-1][l][c]
                        }
                        if l > 0 {
                            num_cases += 1.;
                            assert!(positions[i][j][k][l-1][c] >= 0.);
                            sum_expectancies += positions[i][j][k][l-1][c]
                        }

                        // case die shows joker side
                        // if we can choose which tre to pick from, pick the tree with the largest nb of fruits
                        let m = max(max(i,j),max(k,l));

                        // alt strategy: worst choice : pick from the non-empty tree with minimal nb fruits
                        // let mut m = 9999;
                        // if i>0 {m=min(m,i)}
                        // if j>0 {m=min(m,j)}
                        // if k>0 {m=min(m,k)}
                        // if l>0 {m=min(m,l)}

                        num_cases += 1.;
                        if      m == i {
                            assert!(positions[i-1][j][k][l][c] >= 0.);
                            sum_expectancies += positions[i-1][j][k][l][c]
                        }
                        else if m == j {
                            assert!(positions[i][j-1][k][l][c] >= 0.);
                            sum_expectancies += positions[i][j-1][k][l][c]
                        }
                        else if m == k {
                            assert!(positions[i][j][k-1][l][c] >= 0.);
                            sum_expectancies += positions[i][j][k-1][l][c]
                        }
                        else {
                            assert!(m == l);
                            assert!(positions[i][j][k][l-1][c] >= 0.);
                            sum_expectancies += positions[i][j][k][l-1][c]
                        }

                        // case die shows raven side
                        num_cases += 1.; // the raven can always move
                        if c < 5 {
                            sum_expectancies += positions[i][j][k][l][c+1]
                        } // else lost game, sum_expectancies += 0

                        positions[i][j][k][l][c] = sum_expectancies / num_cases;
                        println!("'{i}{j}{k}{l}{c},{}", positions[i][j][k][l][c]);
                    }
                }
            }
        }
    }
}
