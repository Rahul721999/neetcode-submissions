impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut max_profit = 0;
        let mut min_buy = prices[0];

        for &sell in prices.iter(){
            if min_buy < sell{
                max_profit = max_profit.max(sell - min_buy);
            }
            min_buy = min_buy.min(sell);
        }
        max_profit
    }
}
