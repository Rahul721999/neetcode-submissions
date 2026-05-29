impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut buy = 0; // buy on day zero

        let mut max_profit = 0;
        let len = prices.len();
        for sell in 1..len{
            // if sell on that day is in loss, that can be suitable buy date.
            if prices[sell] <= prices[buy]{
                // make it a buy day
                buy = sell;
            }else{ // you are already in profit, so just count the profit
                max_profit = max_profit.max(prices[sell] - prices[buy]);
            }
        }
        max_profit
    }
}
