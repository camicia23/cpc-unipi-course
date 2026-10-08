impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut max_height_idx = 0;
        for i in 0..height.len() {
            if height[i] > height[max_height_idx] {
                max_height_idx = i;
            }
        }

        let mut sol = 0;
        let mut partial = 0;
        let mut highest = 0;
        
        for i in 0..max_height_idx + 1 {
            if height[i] >= highest {
                highest = height[i];
                sol += partial;
                partial = 0;
            } else {
                partial += highest - height[i];
            }
        }

        highest = 0;
        for i in (max_height_idx..height.len()).rev() {
            if height[i] >= highest {
                highest = height[i];
                sol += partial;
                partial = 0;
            } else {
                partial += highest - height[i];
            }
        }

        sol
    }
}