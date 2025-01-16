pub mod sort {
    pub fn bubble_sort(mut vec: Vec<u32>) -> Vec<u32> {
        for _k in 0..vec.len() {
            for i in 0..vec.len() - 1 {
                if vec[i] > vec[i + 1] {
                    vec.swap(i, i + 1);
                }
            }
        }
        vec
    }
    pub fn sort_by_choice(mut vec: Vec<u32>) -> Vec<u32> {
        for k in 0..vec.len() - 1 {
            for i in k + 1..vec.len() {
                if vec[k] > vec[i] {
                    vec.swap(i, k);
                }
            }
        }
        vec
    }
    // this one is decreasing by defualt
    pub fn insertion_sort(mut vec: Vec<u32>) -> Vec<u32> {
        for k in 0..vec.len() {
            let mut i = k;
            while i > 0 && vec[i - 1] < vec[i] {
                vec.swap(i, i - 1);
                i -= 1;
            }
        }
        vec
    }
}
