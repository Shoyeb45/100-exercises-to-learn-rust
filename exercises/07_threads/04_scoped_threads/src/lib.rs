// TODO: Given a vector of integers, split it in two halves
//  and compute the sum of each half in a separate thread.
//  Don't perform any heap allocation. Don't leak any memory.

pub fn sum(v: Vec<i32>) -> i32 {
    let midpoint = v.len() / 2;
    let val = std::thread::scope(|scope| {
        let handler1 = scope.spawn(|| {
            let first_half = &v[..midpoint];
            first_half.iter().sum::<i32>()
        });
        let handler2 = scope.spawn(|| {
            let second_half = &v[midpoint..];
            second_half.iter().sum::<i32>()
        });
        handler1.join().unwrap() + handler2.join().unwrap()
    });

    val
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(sum(vec![]), 0);
    }

    #[test]
    fn one() {
        assert_eq!(sum(vec![1]), 1);
    }

    #[test]
    fn five() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn nine() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]), 45);
    }

    #[test]
    fn ten() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 55);
    }
}
