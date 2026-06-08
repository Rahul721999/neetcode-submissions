impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut buy_day = 0;
        let mut sell_day = 1;
        let mut max_profit = 0;

        let len = prices.len();
        while sell_day < len{
            let buy = prices[buy_day];
            let sell = prices[sell_day];
            if buy > sell{
                buy_day = sell_day;
            }
            let curr_profit = sell - buy;
            max_profit = max_profit.max(curr_profit);
            sell_day += 1;
        }
        max_profit
    }
}
