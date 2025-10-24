#![allow(dead_code, unused_variables, clippy::useless_vec)]

// Functional programming is a programming paradigm centred around the concepts of pure functions
// (i.e. functions without side effects or internal state - like actual mathematical functions), 
// immutability, and functions as first-class objects often being used and composed to solve problems 
// over lists of data. Functional programming is not a paradigm unique to Rust, but Rust plays quite well with it.

// This is compared to imperative or procedural programming, which is often more about mutable state and 
// declarative statements. 
// For example, the classic imperative approach to calculating the sum of an array might be as follows:
fn imperative_sum() {
	let arr = [1,2,3,4,5,6,7,8,9,10];
	let mut sum = 0;
	for element in arr {
	    sum += element;
	}
}


// Such an implementation is perfectly serviceable, though if this program needed to be made multithreaded then
// some form of access guard must be placed on sum to ensure atomicity and prevent race conditions. 
// It's also fairly wordy and requires four lines for a relatively simple operation.

// A functional approach would instead notice if you remove all of the boilerplate, we are really only applying a 
// function onto an array. Many such problems reduce to the same form, too. Functional programming is all about 
// formalising what it means to apply a function to an array in various ways. A functional implementation of the
// above is shown below. By the end of this 
// chapter you should understand what this is doing.
// let arr = [1,2,3,4,5,6,7,8,9,10];
// let sum = arr.into_iter().reduce(|sum, n| sum + n);
// In this particular case we could just do arr.into_iter().sum() ...


fn main() {
	iterators();
	map();
	filter();
	flatten();
	reduce_fold();
	chaining();
}

fn iterators() {
	// This was glossed over earlier, but Rust's for loop syntax is the 'enhanced for loop', that rather than producing 
	// a counter value that is often used to index an array, instead it directly produces values for you to use. This is 
	// because anything in Rust that can be used in a for loop must implement the Iterator trait, which is a structure that 
	// produces Some(T) values until the underlying data structure runs out of values, then returns None.

	// In a for loop the compiler can infer that you want an iterator and it is done implicitly, but here it must be 
	// done manually 
	// if you want to use the following functions like map, filter, reduce, etc.
	// As mentioned previously in the for loop section, there are three functions:
	// .iter() always provides &T elements,
	// .iter_mut() always provides &mut T elements,
	// .into_iter() provides the same level of ownership as the container (e.g. Vec<T> → T, &Vec<T> → &T). 

	// This is called implicitly if you simply write ‘for x in y’, and is typically the most performant, 
	// as if you own the container then the compiler knows that this call will consume the collection and thus 
	// nothing can use it after, so it may be able to reuse the allocation for the new values.

	// Once you have done whatever you need to do with the iterator, you probably want to convert it back into 
	// some collection, a vector for instance. You do this with the .collect() method. 
	// Here's an example of the identity function:

	let mut vec = vec![1,2,3,4,5];
	vec = vec.into_iter().collect();
	println!("vec: {vec:?}"); // Still [1,2,3,4,5].
}

fn map() {
	// The simplest functional construct is map.  It takes a function and applies it to each element produced by the iterator. 
	// You can think of map as something that takes two things: a collection of T's, and a function that maps a T to a U, and 
	// uses those to produce a collection of type U. In Rust the function signature would be something like:

	// fn map<T,U>(self: Iterator<T>, f: fn(T) -> U) -> Iterator<U>

	// Here's an example:
	let vec = vec![1,2,3,4,5];
	let doubled = vec.into_iter().map(|x| 2*x);  // [2,4,6,8,10]

	// Note that converting from a fixed-size collection (e.g. array) to an iterator (a non-fixed size object) will 
	// lose you length information. The compiler will not be happy if you try and then collect that iterator into 
	// a fixed-length collection, as this may fail. 
	// Instead, arrays directly implement map to avoid the issue altogether:
	let arr = [1,2,3,4,5];
	let doubled = arr.map(|x| 2*x);  // [2,4,6,8,10]

	// If the operation is fallible, consider .and_then() instead, where each element is returned 
	// as an Option<U> instead of just U.
}

fn filter() {
	// Sometimes we may only be interested in certain values inside a collection. 
	// For example, suppose we want everything larger than 3:
	let vec = vec![1,2,3,4,5];
	let gt3: Vec<_> = vec.into_iter().filter(|&x| x > 3).collect(); // [4,5]

	// Quite often we want to both filter elements and then do something to whatever remains. 
	// In this case we can use filter_map. If an invocation returns None it is removed, otherwise if an 
	// element returns Some(val) then val is added to the output:
	let vec = vec![1,2,3,4,5];
	let v: Vec<_> = vec.iter().filter_map(|&x| if x > 3 {Some(x*2)} else {None}).collect();
	println!("{v:?}"); // [8, 10]
}

fn flatten() {
	// In the case we have some nested structure we want to flatten we can use flatten:
	let vec = vec![ vec![1,2], vec![3,4,5] ];
	let flattened: Vec<_> = vec.into_iter().flatten().collect(); //[1,2,3,4,5]

	// Because this works by trying to iterate through each element, and Option<T> and Result<T,E> 
	// happen to be iterable (returning either one value or zero values), this also flattens Option<T> 
	// and Result<T,E> into T, discarding None and Error<E> variants:
	let vec = vec![ Some(1), Some(2), None, Some(3), Some(4) ];
	let somes: Vec<_> = vec.into_iter().flatten().collect(); // [1,2,3,4]
}

fn reduce_fold() {
	// Sometimes you might want to take all the elements of a collection and somehow combine them into one element, 
	// such as a sum or product operation. This is where fold and reduce come in.

	// Reduce takes a function that has an accumulated value and another next element and produces a combined value.
	// This combining operation is then performed on each element in the collection:
	let arr = [1,2,3,4,5];
	let sum = arr.into_iter().reduce(|sum, n| sum + n);

	// In the odd case where you may want to define a different initial value, or the accumulated value is a 
	// different type to your elements, you could instead use fold:
	let arr = [1,2,3,4,5];
	let str = arr.iter().fold(String::new(), |str, n| str + &n.to_string());
	// str is "12345"
}

fn chaining() {
	// Functional operations chain very nicely. For example, a weighted sum of positive numbers:
	let list = vec![(1.0, 1.0), (-1.0, 2.0), (5.0, 1.0), (0.0, 10.0)];

	let weighted_positive_sum = list.into_iter()
		.filter( |&(val, wgt)| val >= 0.0 ) // Remove negative values
		.map( |(val, wgt)| (val*val, wgt) ) // Square values
		.fold(0.0, |acc, (val, wgt)| acc + (val*wgt) ); // Weighted sum
}