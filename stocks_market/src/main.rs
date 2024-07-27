fn main() {
    let price = vec![-1,3,-9,2,2,-1,2,-1,-5];
    let mut max_profit = 0;
    let mut start_index = 0;
    let mut end_index = 0;
    let mut current_profit = 0;
    let mut current_start_index = 0;

    for i in 0..price.len(){
        current_profit += price[i];
        if current_profit > max_profit {
            max_profit = current_profit;
            start_index = current_start_index;
            end_index = i;
        }
        if current_profit < 0 {
            current_profit = 0;
            current_start_index = i + 1;
        }
    }

    println!("Max profit: {}", max_profit);
    println!("Start index: {}", start_index);
    println!("End index: {}", end_index);

}
