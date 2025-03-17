use std::ops::{Add, Sub};

pub fn cartesian_product_flat_map<I, J, T>(iter1: I, iter2: J) -> impl Iterator<Item=(T, T)>
    where
        I: IntoIterator<Item=T>,
        J: Clone + IntoIterator<Item=T>,
        T: Clone,
{
    iter1.into_iter().flat_map(move |a| {
        let iter2_clone = iter2.clone();
        iter2_clone.into_iter().map(move |b| (a.clone(), b))
    })
}

/// 100x slower than [`cartesian_product_flat_map`] and 10_000x slower than [`cartesian_product_refs`]
/// This function is deprecated. Please use one of the faster options instead.
#[deprecated]
pub fn cartesian_product_mut_push<I, J, T>(iter1: I, iter2: J) -> impl Iterator<Item=(T, T)>
    where
        I: IntoIterator<Item=T>,
        J: IntoIterator<Item=T> + Clone,
        T: Clone,
{
    let vec1: Vec<T> = iter1.into_iter().collect();
    let vec2: Vec<T> = iter2.into_iter().collect();
    let mut result = Vec::new();

    for i in &vec1 {
        for j in &vec2 {
            result.push((i.clone(), j.clone()));
        }
    }

    result.into_iter()
}

/// constant runtime of 250pico-seconds - which is absolutely nuuuuuts
pub fn cartesian_product_refs<'a, T>(vec1: &'a [T], vec2: &'a [T])
                                     -> impl Iterator<Item=(&'a T, &'a T)>
{
    vec1.iter().flat_map(move |a| {
        vec2.iter().map(move |b| (a, b))
    })
}

pub fn get_vector_between<T: Sub>(p1: (T, T), p2: (T, T)) -> (T, T)
    where T: Sub<Output=T> + Copy {
    (p2.0 - p1.0, p2.1 - p1.1)
}

/// mirrors p1 on p2
pub fn mirror<T: Sub + Add>(p1: (T, T), p2: (T, T)) -> (T, T)
    where T: Sub<Output=T> + Add<Output=T> + Copy {
    let v = get_vector_between(p1, p2);
    (p2.0 + v.0, p2.1 + v.1)
}

pub fn apply_vec<T: Add>(p: (T, T), v: (T, T)) -> (T, T) where T: Add<Output=T> {
    (p.0 + v.0, p.1 + v.1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_get_vec_between_points() {
        let p1 = (0, 0);
        let p2 = (3, 3);
        let p3 = (5, 5);

        assert_eq!((3, 3), get_vector_between(p1, p2));
        assert_eq!((-3, -3), get_vector_between(p2, p1));
        assert_eq!((2, 2), get_vector_between(p2, p3));
    }

    #[test]
    fn should_apply_vec() {
        let p1 = (5, 5);
        let vec = (3, 3);

        assert_eq!((8, 8), apply_vec(p1, vec));
    }

    #[test]
    fn should_get_mirrored() {
        let p1 = (0, 0);
        let p2 = (3, 3);
        let p3 = (1, 2);

        assert_eq!((6, 6), mirror(p1, p2));
        assert_eq!((2, 4), mirror(p1, p3));
        assert_eq!((-1, 1), mirror(p2, p3));
    }
}