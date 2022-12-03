fn main() {

let arr = [1, 3, 2, 4, 1, 0, 1, 2, 9, 10];

println!("{:?}", find_peak(&arr));

    fn find_peak(arr: &[i32]) -> i32 {

      let mid = arr.len() / 2;

        if arr[mid] < arr[mid] {
                find_peak(&arr[mid + 1..])
            } else if arr[mid] < arr[mid - 1] {
                find_peak(&arr[..mid])
            } else {
                arr[mid]
            }
        }
}
