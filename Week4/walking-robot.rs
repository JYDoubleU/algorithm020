struct Solution;

impl Solution {
    pub fn robot_sim(commands: Vec<i32>, obstacles: Vec<Vec<i32>>) -> i32 {
        use std::collections::HashSet;

        // 在北,东,南,西方向前进一步后, x或y坐标对应的增量. 表驱动法代替if else.
        const DX: [i32; 4] = [0, 1, 0, -1];
        const DY: [i32; 4] = [1, 0, -1, 0];

        let mut x = 0;
        let mut y = 0;
        let mut di = 0;

        let mut set = HashSet::<(i32, i32)>::new();
        for o in &obstacles {
            set.insert((o[0], o[1]));
        }
        let mut ans = 0;

        for &cmd in &commands {
            if cmd == -2 {
                di = (di + 3) % 4;
            } else if cmd == -1 {
                di = (di + 1) % 4;
            } else {
                for _ in 0..cmd {
                    let nx = x + DX[di];
                    let ny = y + DY[di];

                    if !set.contains(&(nx, ny)) {
                        x = nx;
                        y = ny;
                        ans = std::cmp::max(ans, x * x + y * y);
                    }
                }
            }
        }
        ans
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_robot_sim_01() {
        assert_eq!(Solution::robot_sim(vec![4, -1, 3], vec![]), 25);
    }

    #[test]
    fn test_robot_sim_02() {
        assert_eq!(
            Solution::robot_sim(vec![4, -1, 4, -2, 4], vec![vec![2, 4]]),
            65
        );
    }
}
