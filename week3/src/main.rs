
// condition struct for filtering
struct FilterCondition<T> {
    value:T
}

// implementation of FilterCondition
impl<T: PartialEq> FilterCondition<T> {
    fn is_match(&self, item:&T) -> bool { // check if item is equal to given value
        *item == self.value // return true if item is equal to given value
    }
}

// custom filter function with generic type using FilterCondition
fn custom_filter<T>(collection: &[T], filter_condition: &FilterCondition<T>) -> Vec<T> where T:PartialEq + Clone {
    
    let mut filtered_collection = Vec::new(); // create new vector for filtered collection

    for item in collection { // iterate over collection
        if filter_condition.is_match(item) { // check if item is equal to given value
            filtered_collection.push(item.clone()); // push item to filtered collection
        }
    }
    filtered_collection // return filtered collection

}


fn main() {
    
    let collection = vec![1, 5, 3, 6, 9]; // create collection
    let filter_condition = FilterCondition { value: 5}; // create custom filter condition

    let result = custom_filter(&collection, &filter_condition); // get result filter collection

    println!("result : {:?}", result); // print result
}