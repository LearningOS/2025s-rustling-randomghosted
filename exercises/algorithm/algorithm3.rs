/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/
// I AM NOT DONE
use std::cmp::Ordering;
// use rand::Rng;
fn sort<T:Ord+Copy>(array: &mut [T]){
	//TODO
    if array.len()<=1{
        return;
    }

    let (leftBound,rightBound)=partition(array,0,array.len()-1);
    let len=&mut array.len();
    sort(&mut array[0..leftBound]);
    sort(&mut array[rightBound+1..(*len as usize)]);
}

fn partition<T:Ord+Copy>(array:&mut [T], left:usize, right:usize)->(usize,usize){
    // let mut rng=rand::thread_rng();
    // let random=rng.gen_range(left..=right);

    let pivot=array[left];

    let mut leftBound=left; let mut rightBound=right;
    let mut i=left;
    loop{
        match array[i].cmp(&pivot){
            Ordering::Less=>{
                array.swap(leftBound,i);
                leftBound+=1;
                i+=1;
            },
            Ordering::Greater=>{
                array.swap(rightBound,i);
                rightBound-=1;
            },
            Ordering::Equal=>{
                i+=1;
            }
        }

        if i==rightBound+1{
            break;
        }
    }

    (leftBound,rightBound)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}