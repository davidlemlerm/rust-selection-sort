use std::vec::Vec;
use std::io::{self, BufRead};

fn main() {
	let mut numbers: Vec<i32> = Vec::new();
	let mut collect_input: bool = true;
	println!("Enter the numbers to sort:");
	while collect_input {
		let input = io::stdin().lock().lines().next().unwrap().unwrap();
		let input_int = input.parse::<i32>();
		if !input_int.is_err() {
			numbers.push(input_int.unwrap());
		}
		else {
			collect_input = false;
		}
	}
	sort(&mut numbers);
	println!("Sorted list:\n{:?}", numbers);
}

fn sort(array: &mut [i32]) {
	let mut current_index: usize = 0;
	while current_index < array.len() {
		let smallest: usize = find_smallest_index(array, current_index, array.len());
		swap(array, current_index, smallest);
		current_index = current_index + 1;
	}
}

fn find_smallest_index(array: &mut [i32], start: usize, end: usize) -> usize {
	let mut current_index: usize = start;
	let mut smallest_index: usize = start;
	while current_index < end {
		if array[current_index] < array[smallest_index] {
			smallest_index = current_index;
		}
		current_index = current_index + 1;
	}
	return smallest_index;
}

fn swap(array: &mut [i32], i: usize, j: usize) -> bool {
	if i == j {
		return false;
	}
	else {
		let temp: i32 = array[i];
		array[i] = array[j];
		array[j] = temp;
		return true;
	}
}
