use std::{f64::{INFINITY}};

pub fn add(x: &Vec<isize>, y: &Vec<isize>) -> Vec<isize> {
    let mut result: Vec<isize>;
    let length: usize = usize::min(x.len(), y.len());

    if x.len() > y.len() {
        result = x.clone();
    } else {
        result = y.clone();
    }

    for i in 0..length {
        let temp = &x[i] + &y[i];
        result[i] = temp;
    }
    result
}

pub fn subtract(x: &Vec<isize>, y: &Vec<isize>) -> Vec<isize> {
    let mut result: Vec<isize>;
    let length: usize = usize::min(x.len(), y.len());

    if x.len() > y.len() {
        result = x.clone();
    } else {
        result = negate(&y);
    }

    for i in 0..length {
        let temp = &x[i] - &y[i];
        result[i] = temp;
    }
    result
}

pub fn negate(x: &Vec<isize>) -> Vec<isize> {
    x.into_iter().map(|x| -x).collect()
}

pub fn magnitude(x: &Vec<isize>) -> f64 {
    let mut sum = 0;
    for element in x {
        sum = sum + element.pow(2);
    }
    (sum as f64).sqrt()
}

pub fn newton<T, N>(n: isize, x: f64, f: T, fprime: N, print: bool) -> Result<f64, &'static str> 
    where T: Fn(f64) -> f64, 
          N: Fn(f64) -> f64,
{
    if fprime(x) == INFINITY {
        return Err("ERROR: Divide by zero.")
    }

    if n < 0 {
        return Ok(x)
    }

    let new_x = x - (f(x)/fprime(x));

    if print {
        println!("{:.52}", new_x);
    }
    newton(n-1, new_x, f, fprime, print)
}

pub fn secant<T>(n: isize, x1: f64, x2: f64, f: T, print: bool) -> Result<f64, &'static str> 
where T: Fn(f64) -> f64,
    {
        if n < 0 || x1 == x2 {
            return Ok(x1)
        }

        let new_x1 = ((x2*f(x1))-(x1*f(x2)))/(f(x1)-f(x2));
        
        if print {
            println!("{:.52}", new_x1);
        }
        secant(n-1, new_x1, x1, f, print)
    }


pub fn dot(a: &Vec<isize>, b: &Vec<isize>) -> Result<isize, String> {
    if a.len() != b.len() {
        return Err(String::from("Vectors are not equal lengths."));
    } else if a.len() == 0 {
        return Err(String::from("Vectors are empty."));
    } else {
        let mut sum = 0;
        for i in 0..a.len() {
            sum = sum + a[i] * b[i];
        }
        Ok(sum)
    }
}

pub fn cross(a: &Vec<isize>, b: &Vec<isize>) -> Result<Vec<isize>, String> {
    if a.len() == 0 || b.len() == 0 {
        return Err(String::from("One or more vectors are empty"));
    } else if a.len() != 3 ||  b.len() != 3 {
        return Err(String::from("One or more vectors are not 3d"));
    } else {
        let s1: isize = a[1]*b[2] - a[2]*b[1];
        let s2: isize = a[2]*b[0] - a[0]*b[2];
        let s3: isize = a[0]*b[1] - a[1]*b[0];
        Ok(vec![s1, s2, s3])
    }
}

#[cfg(test)]
mod add_tests {
    use super::add;

    #[test]
    fn test_simple1() {
        let vec1: Vec<isize> = vec![1,2,3,4];
        let vec2: Vec<isize> = vec![2,4,6,8];

        assert_eq!(add(&vec1, &vec2), [3,6,9,12]);
    }

    #[test]
    fn test_simple2() {
        let vec1: Vec<isize> = vec![4,1,7,5,2];
        let vec2: Vec<isize> = vec![324,90,32,54,9];

        assert_eq!(add(&vec1, &vec2), [328,91,39,59,11]);
    }

    #[test]
    fn test_different_sizes1() {
        let vec1: Vec<isize> = vec![1,2,3,4];
        let vec2: Vec<isize> = vec![2,4];

        assert_eq!(add(&vec1, &vec2), [3,6,3,4]);
    }

    #[test]
    fn test_different_sizes2() {
        let vec1: Vec<isize> = vec![1,2];
        let vec2: Vec<isize> = vec![2,4,6,8];

        assert_eq!(add(&vec1, &vec2), [3,6,6,8]);
    }
}

#[cfg(test)]
mod subtract_tests {
    use super::subtract;

    #[test]
    fn test_simple1() {
        let vec1: Vec<isize> = vec![1,2,3,4];
        let vec2: Vec<isize> = vec![2,4,6,8];

        assert_eq!(subtract(&vec1, &vec2), [-1,-2,-3,-4]);
    }

    #[test]
    fn test_simple2() {
        let vec1: Vec<isize> = vec![324,90,32,54,9];
        let vec2: Vec<isize> = vec![4,1,7,5,2];

        assert_eq!(subtract(&vec1, &vec2), [320,89,25,49,7]);
    }

    #[test]
    fn test_different_sizes1() {
        let vec1: Vec<isize> = vec![1,2,3,4];
        let vec2: Vec<isize> = vec![2,4];

        assert_eq!(subtract(&vec1, &vec2), [-1,-2,3,4]);
    }

    #[test]
    fn test_different_sizes2() {
        let vec1: Vec<isize> = vec![1,2];
        let vec2: Vec<isize> = vec![2,4,6,8];

        assert_eq!(subtract(&vec1, &vec2), [-1,-2,-6,-8]);
    }
}

#[cfg(test)]
mod negate_tests {
    use super::negate;

    #[test]
    fn test_simple1() {
        let vec1: Vec<isize> = vec![1,2,3,4];

        assert_eq!(negate(&vec1), [-1,-2,-3,-4]);
    }

    #[test]
    fn test_simple2() {
        let vec1: Vec<isize> = vec![324,90,32,54,9];

        assert_eq!(negate(&vec1), [-324,-90,-32,-54,-9]);
    }
}

#[cfg(test)]
mod magnitude_tests {
    use super::magnitude;

    #[test]
    fn test_simple1() {
        let vec1: Vec<isize> = vec![3,4];

        assert_eq!(magnitude(&vec1), 5.0);
    }

    #[test]
    fn test_simple2() {
        let vec1: Vec<isize> = vec![9,12];

        assert_eq!(magnitude(&vec1), 15.0);
    }
}

#[cfg(test)]
mod newton_tests {
    use super::newton;

    #[test]
    fn test_prime_equals_zero() {
        let f= |x| -> f64 { (x*x) - 2.0 };
        let fprime = |x| -> f64 { 2.0*x };
        let guess: f64 = 0.0;

        assert_eq!(newton(100, guess, f, fprime, false), Err("ERROR: Divide by zero."));
    }

    #[test]
    fn test_simple_case_positive_guess() {
        let f= |x| -> f64 { x - 2.0 };
        let fprime = |_x| -> f64 { 1.0 };
        let guess: f64 = 5.0;
        
        assert_eq!(newton(100, guess, f, fprime, false), Ok(2.0));
    }

    #[test]
    fn test_simple_case_negative_guess() {
        let f= |x| -> f64 { x - 2.0 };
        let fprime = |_x| -> f64 { 1.0 };
        let guess: f64 = -5.0;
        
        assert_eq!(newton(100, guess, f, fprime, false), Ok(2.0));
    }
}

#[cfg(test)]
mod secant_tests {
    use super::secant;

    #[test]
    fn test_simple_case_positive_guess() {
        let f= |x| -> f64 { x - 2.0 };
        let guess1: f64 = 5.0;
        let guess2: f64 = 2.0;
        
        assert_eq!(secant(100, guess1, guess2, f, false), Ok(2.0));
    }

    #[test]
    fn test_simple_case_negative_guess() {
        let f= |x| -> f64 { x - 2.0 };
        let guess1: f64 = -5.0;
        let guess2: f64 = 2.0;
        
        assert_eq!(secant(100, guess1, guess2, f, false), Ok(2.0));
    }
}

#[cfg(test)]
mod dot_tests {
    use std::vec;
    use super::{add, dot};

    #[test]
    fn test_simple1() {
        let a: Vec<isize> = vec![1,2,3,4];
        let b: Vec<isize> = vec![4,3,2,1];
        assert_eq!(dot(&b, &a).unwrap(), 20);
    }

    #[test]
    fn test_simple2() {
        let a: Vec<isize> = vec![1,2];
        let b: Vec<isize> = vec![4,3];
        assert_eq!(dot(&b, &a).unwrap(), 10);
    }

    #[test]
    fn test_simple3() {
        let a: Vec<isize> = vec![245, 647];
        let b: Vec<isize> = vec![876, 123];
        assert_eq!(dot(&b, &a).unwrap(), 294201);
    }

    #[test]
    fn test_simple_with_negatives1() {
        let a: Vec<isize> = vec![1,-2,3,4];
        let b: Vec<isize> = vec![4,3,2,-1];
        assert_eq!(dot(&b, &a).unwrap(), 0);
    }

    #[test]
    fn test_simple_with_negatives2() {
        let a: Vec<isize> = vec![1,-2];
        let b: Vec<isize> = vec![4,3];
        assert_eq!(dot(&b, &a).unwrap(), -2);
    }

    #[test]
    fn test_simple_with_negatives3() {
        let a: Vec<isize> = vec![245, 647];
        let b: Vec<isize> = vec![-876, 123];
        assert_eq!(dot(&b, &a).unwrap(), -135039);
    }

    #[test]
    fn test_one_empty_vec1() {
        let a: Vec<isize> = vec![1,2,3,4];
        let b: Vec<isize> = vec![];
        assert_eq!(dot(&b, &a).unwrap_err(), "Vectors are not equal lengths.");
    }

    #[test]
    fn test_one_empty_vec2() {
        let a: Vec<isize> = vec![];
        let b: Vec<isize> = vec![1,2,3,4];
        assert_eq!(dot(&b, &a).unwrap_err(), "Vectors are not equal lengths.");
    }

    #[test]
    fn test_two_empty_vec() {
        let a: Vec<isize> = vec![];
        let b: Vec<isize> = vec![];
        assert_eq!(dot(&b, &a).unwrap_err(), "Vectors are empty.");
    }

    #[test]
    fn test_different_length_vecs1() {
        let a: Vec<isize> = vec![1,2,3,4];
        let b: Vec<isize> = vec![];
        assert_eq!(dot(&b, &a).unwrap_err(), "Vectors are not equal lengths.")
    }

    #[test]
    fn test_different_length_vecs2() {
        let a: Vec<isize> = vec![];
        let b: Vec<isize> = vec![1,2,3,4];
        assert_eq!(dot(&b, &a).unwrap_err(), "Vectors are not equal lengths.")
    }   

    #[test]
    fn test_commutative() {
        let a: Vec<isize> = vec![1,2,3,4];
        let b: Vec<isize> = vec![4,3,2,1];
        assert_eq!(dot(&a, &b), dot(&b, &a))
    }

    #[test]
    fn test_distributive() {
        let a: Vec<isize> = vec![1,2,3,4];
        let b: Vec<isize> = vec![4,3,2,1];
        let c: Vec<isize> = vec![5,6,7,8];
        let d: Vec<isize> = add(&b, &c);
        assert_eq!(dot(&a, &d).unwrap(), dot(&a, &b).unwrap() + dot(&a, &c).unwrap())
    }

    #[test]
    fn test_bilinear() {
        
    }

    #[test]
    fn test_scalar_multiplication() {
        
    }

    #[test]
    fn test_not_associative() {
        
    }

    #[test]
    fn test_orthogonal() {
        
    }

    #[test]
    fn test_not_cancellation() {
        
    }

    #[test]
    fn test_product_rule() {
        
    }
}

#[cfg(test)]
mod cross_tests {
    use std::vec;
    use super::{add, cross};

    #[test]
    fn test_one_not_3d1() {
        let a: Vec<isize> = vec![1,2,3];
        let b: Vec<isize> = vec![1,2];
        assert_eq!(cross(&a, &b).unwrap_err(), "One or more vectors are not 3d");
    }

    #[test]
    fn test_one_not_3d2() {
        let a: Vec<isize> = vec![1,2];
        let b: Vec<isize> = vec![1,2,3];
        assert_eq!(cross(&a, &b).unwrap_err(), "One or more vectors are not 3d");
    }

    #[test]
    fn test_one_empty1() {
        let a: Vec<isize> = vec![1,2,3];
        let b: Vec<isize> = vec![];
        assert_eq!(cross(&a, &b).unwrap_err(), "One or more vectors are empty");
    }  

    #[test]
    fn test_one_empty2() {
        let a: Vec<isize> = vec![];
        let b: Vec<isize> = vec![1,2,3];
        assert_eq!(cross(&a, &b).unwrap_err(), "One or more vectors are empty");
    }

    #[test]
    fn test_both_empty() {
        let a: Vec<isize> = vec![];
        let b: Vec<isize> = vec![];
        assert_eq!(cross(&a, &b).unwrap_err(), "One or more vectors are empty");
    }  

    #[test]
    fn test_same_vectors() {
        // a X a = 0
        let a: Vec<isize> = vec![1,2,3];
        let b: Vec<isize> = vec![1,2,3];
        assert_eq!(cross(&a, &b).unwrap(), vec![0,0,0]);
    }

    #[test]
    fn test_anticommutative() {
        // a X b = -(b X a)
        let a: Vec<isize> = vec![1,2,3];
        let b: Vec<isize> = vec![3,2,3];
        let negative_ans: Vec<isize> = cross(&b, &a).unwrap().into_iter().map(|x| -x).collect();
        assert_eq!(cross(&a, &b).unwrap(), negative_ans);
    }

    #[test]
    fn test_distributive_over_addition() {
        // a X (b + c) = (a X b) + (a X c)
        let a: Vec<isize> = vec![1,2,3];
        let b: Vec<isize> = vec![3,2,3];
        let c: Vec<isize> = vec![3,2,3];
        let d: Vec<isize> = add(&b, &c); // b + c
        assert_eq!(cross(&a, &d).unwrap(), add(&cross(&a, &b).unwrap(), &cross(&a, &c).unwrap()));
    }

    #[test]
    fn test_scalar_multiplication() {
        // (ra) X b = a X (rb) = r(a X b)
        // r = 2
        let a: Vec<isize> = vec![1,2,3];
        let ra: Vec<isize> = vec![2,4,6];
        let b: Vec<isize> = vec![3,2,3];
        let rb: Vec<isize> = vec![6,4,6];

        let first: Vec<isize> = cross(&ra, &b).unwrap();
        let second: Vec<isize> = cross(&a, &rb).unwrap();
        let third: Vec<isize> = cross(&a, &b).unwrap().into_iter().map(|x| x*2).collect();
        assert_eq!(first, second);
        assert_eq!(second, third);
        assert_eq!(first, third);
    }

    #[test]
    fn test_not_associative_but_satisfies_jacobi_iden() {
        // a X (b X c) + 
        // b X (c X a) + 
        // c X (a X b) 
        // = 0
        let a: Vec<isize> = vec![1,2,3];
        let b: Vec<isize> = vec![3,2,3];
        let c: Vec<isize> = vec![3,2,4];

        let first: Vec<isize> = cross(&a, &cross(&b, &c).unwrap()).unwrap();
        let second: Vec<isize> = cross(&b, &cross(&c, &a).unwrap()).unwrap();
        let third: Vec<isize> = cross(&c, &cross(&a, &b).unwrap()).unwrap();

        assert_eq!(add(&add(&first, &second), &third), vec![0,0,0]);
    }
}