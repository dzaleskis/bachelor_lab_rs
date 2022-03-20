fn merge<T: Copy + Ord>(left: &[T], right: &[T], dest: &mut [T]) -> u32 {
    let mut inversions = 0;

    let mut i: usize = 0;
    let mut j: usize = 0;
    let n = left.len();
    let m = right.len();

    for k in 0..dest.len() {
        if i < n {
            if j < m {
                if left[i] <= right[j] {
                    dest[k] = left[i];
                    i += 1;
                } else {
                    dest[k] = right[j];
                    j += 1;
                    inversions += n - i;
                }
            } else {
                dest[k] = left[i];
                i += 1;
            }
        } else {
            dest[k] = right[j];
            j += 1;
        }
    }

    inversions as u32
}

pub fn count_inversions<T: Copy + Ord>(slice: &mut [T]) -> u32 {
    let n = slice.len();
    let mut inversions = 0;

    if n <= 1 {
        return inversions;
    }

    let mut copy = slice.to_vec();
    let (left, right) = copy.as_mut_slice().split_at_mut(n / 2);

    inversions += count_inversions(left);
    inversions += count_inversions(right);
    inversions += merge(left, right, slice);

    inversions
}

#[test]
fn test_merge() {
    let mut data = [3, 1, 2];
    let mut copy = data.clone();

    let (left, right) = copy.split_at_mut(1);

    merge(left, right, &mut data);

    assert_eq!(data, [1, 2, 3]);
}

#[test]
fn test_inversion_counting() {
    let mut data = [1, 3, 2];

    assert_eq!(count_inversions(&mut data), 1);
}