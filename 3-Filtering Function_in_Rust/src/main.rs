struct FilterCondition<T>{
    filter : T
}

impl <T: PartialOrd> FilterCondition<T> {
    fn is_match (&self, item: &T) -> bool {
        item > &self.filter
    }
}

fn custom_filter<T>(list: Vec<T> , condition: &FilterCondition<T>) -> Vec<T> where T: PartialOrd{
    return list.into_iter().filter(|item: &T| condition.is_match(item)).collect();
}

fn main() {
    let numbers = vec![5, 10, 15, 20, 25, 30, 35, 40, 45, 50];
    let condition = FilterCondition { filter: 39 };
    let filtered_list = custom_filter(numbers, &condition);

    println!("{:?}", filtered_list);
}
